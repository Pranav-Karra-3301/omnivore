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
    pub fn new(config: Arc<CrawlConfig>) -> Self {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(Duration::from_millis(config.timeout_ms))
            .gzip(true)
            .brotli(true)
            .build()
            .expect("Failed to build HTTP client");

        Self { client, config }
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

        let links = self.extract_links(&url, &content)?;

        Ok(CrawlResult {
            url: url.to_string(),
            status_code,
            content,
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
                Ok(response) => return Ok(response),
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

        Err(Error::Network(last_error.unwrap()))
    }

    fn extract_links(&self, base_url: &Url, html: &str) -> Result<Vec<Url>> {
        let document = scraper::Html::parse_document(html);
        let selector = scraper::Selector::parse("a[href]").unwrap();

        let mut links = Vec::new();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = base_url.join(href) {
                    if absolute_url.scheme() == "http" || absolute_url.scheme() == "https" {
                        links.push(absolute_url);
                    }
                }
            }
        }

        Ok(links)
    }
}
