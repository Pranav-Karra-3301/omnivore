use crate::extractor::ContentExtractor;
use crate::patterns;
use crate::{CrawlConfig, CrawlResult, Error, Result};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub struct Worker {
    client: Client,
    config: Arc<CrawlConfig>,
}

impl Worker {
    /// Create a new worker with the given configuration.
    ///
    /// # Errors
    /// Returns an error if the HTTP client cannot be built.
    pub fn new(config: Arc<CrawlConfig>) -> Result<Self> {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(Duration::from_millis(config.timeout_ms))
            .redirect(reqwest::redirect::Policy::limited(10))
            .gzip(true)
            .brotli(true)
            .build()
            .map_err(Error::Network)?;

        Ok(Self { client, config })
    }

    pub async fn crawl(&self, url: Url) -> Result<CrawlResult> {
        let response = self.fetch_with_retry(&url).await?;
        let status_code = response.status().as_u16();

        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .collect();

        let content = response.text().await?;

        // Extract clean content
        let extractor = ContentExtractor::new();
        let cleaned_content = Some(extractor.extract_clean_content(&content));

        let links = self.extract_links(&url, &content);

        Ok(CrawlResult {
            url: url.to_string(),
            status_code,
            content,
            cleaned_content,
            headers,
            extracted_data: serde_json::json!({}),
            links: links.into_iter().map(|u| u.to_string()).collect(),
            crawled_at: chrono::Utc::now(),
        })
    }

    async fn fetch_with_retry(&self, url: &Url) -> Result<reqwest::Response> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.max_retries {
            match self.client.get(url.as_str()).send().await {
                Ok(response) => {
                    // Log redirect chain if any
                    if response.url() != url {
                        let redirect_msg = format!("Redirected: {} -> {}", url, response.url());
                        tracing::info!("{}", redirect_msg);

                        // Write to warnings log
                        let warning_entry = format!(
                            "[{}] {}\n",
                            chrono::Utc::now().to_rfc3339(),
                            redirect_msg
                        );
                        if let Ok(mut file) = tokio::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("warnings.log")
                            .await
                        {
                            use tokio::io::AsyncWriteExt;
                            if let Err(e) = file.write_all(warning_entry.as_bytes()).await {
                                tracing::warn!("Failed to write warning log: {}", e);
                            }
                        }
                    }
                    return Ok(response);
                }
                Err(e) => {
                    attempts += 1;
                    last_error = Some(e);

                    if attempts < self.config.max_retries {
                        let delay = Duration::from_millis(
                            100 * (self
                                .config
                                .politeness
                                .backoff_multiplier
                                .powi(attempts as i32) as u64),
                        );
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        // last_error is always Some here because we only exit the loop after an error
        match last_error {
            Some(e) => Err(Error::Network(e)),
            None => Err(Error::Unknown(
                "Max retries reached with no error recorded".to_string(),
            )),
        }
    }

    fn extract_links(&self, base_url: &Url, html: &str) -> Vec<Url> {
        let document = scraper::Html::parse_document(html);
        let mut links = Vec::new();

        // Use pre-compiled selector from patterns module
        for element in document.select(&patterns::ANCHOR_HREF) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = base_url.join(href) {
                    if absolute_url.scheme() == "http" || absolute_url.scheme() == "https" {
                        links.push(absolute_url);
                    }
                }
            }
        }

        links
    }
}
