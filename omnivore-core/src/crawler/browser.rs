use crate::{CrawlResult, Error, Result};
use thirtyfour::prelude::*;
use url::Url;

pub struct BrowserEngine {
    driver: Option<WebDriver>,
}

impl BrowserEngine {
    pub async fn new() -> Result<Self> {
        Ok(Self { driver: None })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let caps = DesiredCapabilities::chrome();

        match WebDriver::new("http://localhost:9515", caps).await {
            Ok(driver) => {
                self.driver = Some(driver);
                Ok(())
            }
            Err(e) => Err(Error::Browser(format!("Failed to connect to browser: {e}"))),
        }
    }

    pub async fn crawl_dynamic(&self, url: Url) -> Result<CrawlResult> {
        let driver = self
            .driver
            .as_ref()
            .ok_or_else(|| Error::Browser("Browser not connected".to_string()))?;

        driver
            .goto(url.as_str())
            .await
            .map_err(|e| Error::Browser(format!("Navigation failed: {e}")))?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let content = driver
            .source()
            .await
            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;

        let links = self.extract_links_js(driver, &url).await?;

        Ok(CrawlResult {
            url: url.to_string(),
            status_code: 200,
            content,
            headers: std::collections::HashMap::new(),
            extracted_data: serde_json::json!({}),
            links: links.into_iter().map(|u| u.to_string()).collect(),
            crawled_at: chrono::Utc::now(),
        })
    }

    async fn extract_links_js(&self, driver: &WebDriver, _base_url: &Url) -> Result<Vec<Url>> {
        let script = r#"
            return Array.from(document.querySelectorAll('a[href]'))
                .map(a => a.href)
                .filter(href => href.startsWith('http'));
        "#;

        let links_value = driver
            .execute(script, vec![])
            .await
            .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?;

        let mut links = Vec::new();

        // Convert the ScriptRet value to JSON
        let json_value = links_value.json();
        if let Some(array) = json_value.as_array() {
            for value in array {
                if let Some(href) = value.as_str() {
                    if let Ok(url) = Url::parse(href) {
                        links.push(url);
                    }
                }
            }
        }

        Ok(links)
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(driver) = self.driver.take() {
            driver
                .quit()
                .await
                .map_err(|e| Error::Browser(format!("Failed to quit browser: {e}")))?;
        }
        Ok(())
    }
}

impl Drop for BrowserEngine {
    fn drop(&mut self) {
        if let Some(driver) = self.driver.take() {
            tokio::task::spawn(async move {
                let _ = driver.quit().await;
            });
        }
    }
}
