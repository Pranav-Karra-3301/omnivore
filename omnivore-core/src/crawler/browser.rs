use crate::{CrawlResult, Error, Result};
use crate::extractor::ContentExtractor;
use thirtyfour::prelude::*;
use url::Url;
use tokio::time::sleep;
use std::time::Duration;
use tracing::{info, warn, debug};
use serde::{Serialize, Deserialize};

pub struct BrowserEngine {
    driver: Option<WebDriver>,
    headless: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicContent {
    pub url: String,
    pub main_content: String,
    pub dropdown_contents: Vec<DropdownContent>,
    pub filter_contents: Vec<FilterContent>,
    pub has_infinite_scroll: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropdownContent {
    pub index: usize,
    pub label: Option<String>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterContent {
    pub index: usize,
    pub label: Option<String>,
    pub content: String,
}

impl BrowserEngine {
    pub async fn new() -> Result<Self> {
        Ok(Self { 
            driver: None,
            headless: true,
        })
    }
    
    pub async fn new_with_options(headless: bool) -> Result<Self> {
        Ok(Self {
            driver: None,
            headless,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let mut caps = DesiredCapabilities::chrome();
        
        // Add Chrome options
        let chrome_args = if self.headless {
            vec![
                "--headless",
                "--no-sandbox",
                "--disable-dev-shm-usage",
                "--disable-gpu",
                "--window-size=1920,1080",
                "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            ]
        } else {
            vec![
                "--no-sandbox",
                "--disable-dev-shm-usage",
                "--window-size=1920,1080",
                "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            ]
        };
        
        for arg in chrome_args {
            caps.add_arg(arg).map_err(|e| Error::Browser(format!("Failed to add Chrome arg: {e}")))?;
        }

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

        // Wait for page to be ready
        self.wait_for_page_ready(driver).await?;

        let content = driver
            .source()
            .await
            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;

        let links = self.extract_links_js(driver, &url).await?;

        // Extract clean content
        let extractor = ContentExtractor::new();
        let cleaned_content = Some(extractor.extract_clean_content(&content));

        Ok(CrawlResult {
            url: url.to_string(),
            status_code: 200,
            content,
            cleaned_content,
            headers: std::collections::HashMap::new(),
            extracted_data: serde_json::json!({}),
            links: links.into_iter().map(|u| u.to_string()).collect(),
            crawled_at: chrono::Utc::now(),
        })
    }
    
    pub async fn crawl_with_interactions(&self, url: Url) -> Result<DynamicContent> {
        let driver = self
            .driver
            .as_ref()
            .ok_or_else(|| Error::Browser("Browser not connected".to_string()))?;

        info!("Crawling with interactions: {}", url);
        
        driver
            .goto(url.as_str())
            .await
            .map_err(|e| Error::Browser(format!("Navigation failed: {e}")))?;

        self.wait_for_page_ready(driver).await?;
        
        // Check for infinite scroll
        let has_infinite_scroll = self.detect_infinite_scroll(driver).await?;
        if has_infinite_scroll {
            info!("Detected infinite scroll, loading all content...");
            self.handle_infinite_scroll(driver, 10).await?;
        }
        
        // Get main content
        let main_content = driver
            .source()
            .await
            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;
        
        // Find and interact with dropdowns
        let dropdown_contents = self.interact_with_dropdowns(driver).await?;
        
        // Find and interact with filters
        let filter_contents = self.interact_with_filters(driver).await?;
        
        Ok(DynamicContent {
            url: url.to_string(),
            main_content,
            dropdown_contents,
            filter_contents,
            has_infinite_scroll,
        })
    }
    
    async fn wait_for_page_ready(&self, driver: &WebDriver) -> Result<()> {
        let script = r#"
            return document.readyState === 'complete' && 
                   (typeof jQuery === 'undefined' || jQuery.active === 0) &&
                   (typeof angular === 'undefined' || !angular.element(document).injector() || 
                    angular.element(document).injector().get('$http').pendingRequests.length === 0);
        "#;
        
        let max_wait = Duration::from_secs(30);
        let start = std::time::Instant::now();
        
        loop {
            if start.elapsed() > max_wait {
                warn!("Page load timeout exceeded");
                break;
            }
            
            match driver.execute(script, vec![]).await {
                Ok(ret) => {
                    if let Some(ready) = ret.json().as_bool() {
                        if ready {
                            debug!("Page is ready");
                            break;
                        }
                    }
                }
                Err(_) => {
                    // Try basic readyState check
                    let basic_script = "return document.readyState === 'complete';";
                    if let Ok(ret) = driver.execute(basic_script, vec![]).await {
                        if let Some(ready) = ret.json().as_bool() {
                            if ready {
                                break;
                            }
                        }
                    }
                }
            }
            
            sleep(Duration::from_millis(500)).await;
        }
        
        Ok(())
    }
    
    async fn detect_infinite_scroll(&self, driver: &WebDriver) -> Result<bool> {
        let initial_height = driver
            .execute("return document.body.scrollHeight;", vec![])
            .await
            .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?  
            .json()
            .as_i64()
            .unwrap_or(0);
        
        driver.execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
            .await
            .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?;
        sleep(Duration::from_secs(2)).await;
        
        let new_height = driver
            .execute("return document.body.scrollHeight;", vec![])
            .await
            .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?  
            .json()
            .as_i64()
            .unwrap_or(0);
        
        Ok(new_height > initial_height)
    }
    
    async fn handle_infinite_scroll(&self, driver: &WebDriver, max_scrolls: u32) -> Result<()> {
        let mut last_height: i64 = 0;
        
        for _ in 0..max_scrolls {
            let current_height = driver
                .execute("return document.body.scrollHeight;", vec![])
                .await
                .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?
                .json()
                .as_i64()
                .unwrap_or(0);
            
            if current_height == last_height {
                break;
            }
            
            last_height = current_height;
            driver.execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
            .await
            .map_err(|e| Error::Browser(format!("Script execution failed: {e}")))?;
            sleep(Duration::from_secs(2)).await;
        }
        
        Ok(())
    }
    
    async fn interact_with_dropdowns(&self, driver: &WebDriver) -> Result<Vec<DropdownContent>> {
        let mut contents = Vec::new();
        
        // Find select elements
        let selects = driver.find_all(By::Css("select")).await.unwrap_or_default();
        
        for (idx, select) in selects.iter().enumerate() {
            // Get label if available
            let label = select.attr("aria-label").await.ok().flatten();
            
            // Get all options
            if let Ok(options) = select.find_all(By::Css("option")).await {
                for option in options.iter().skip(1) { // Skip first (usually default) option
                    if let Ok(_) = option.click().await {
                        sleep(Duration::from_millis(1000)).await;
                        self.wait_for_page_ready(driver).await?;
                        
                        let content = driver.source().await
                            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;
                        contents.push(DropdownContent {
                            index: idx,
                            label: label.clone(),
                            content,
                        });
                    }
                }
            }
        }
        
        // Find custom dropdowns
        let custom_dropdowns = driver.find_all(By::Css("[role='combobox'], .dropdown, [data-toggle='dropdown']"))
            .await.unwrap_or_default();
        
        for (idx, dropdown) in custom_dropdowns.iter().enumerate() {
            let label = dropdown.attr("aria-label").await.ok().flatten();
            
            if let Ok(_) = dropdown.click().await {
                sleep(Duration::from_millis(500)).await;
                
                // Look for dropdown items
                if let Ok(items) = driver.find_all(By::Css(".dropdown-item, [role='option'], li")).await {
                    for item in items.iter().take(5) { // Limit to first 5 items
                        if let Ok(_) = item.click().await {
                            sleep(Duration::from_millis(1000)).await;
                            self.wait_for_page_ready(driver).await?;
                            
                            let content = driver.source().await
                            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;
                            contents.push(DropdownContent {
                                index: selects.len() + idx,
                                label: label.clone(),
                                content,
                            });
                            
                            // Re-open dropdown
                            dropdown.click().await.ok();
                            sleep(Duration::from_millis(500)).await;
                        }
                    }
                }
            }
        }
        
        info!("Extracted {} dropdown variations", contents.len());
        Ok(contents)
    }
    
    async fn interact_with_filters(&self, driver: &WebDriver) -> Result<Vec<FilterContent>> {
        let mut contents = Vec::new();
        
        // Find checkboxes and radio buttons
        let filters = driver.find_all(By::Css("input[type='checkbox'], input[type='radio'], [role='checkbox'], [role='radio']"))
            .await.unwrap_or_default();
        
        for (idx, filter) in filters.iter().enumerate().take(10) { // Limit to first 10 filters
            let label = filter.attr("aria-label").await.ok().flatten();
            
            if let Ok(_) = filter.click().await {
                sleep(Duration::from_millis(1000)).await;
                self.wait_for_page_ready(driver).await?;
                
                let content = driver.source().await
                            .map_err(|e| Error::Browser(format!("Failed to get page source: {e}")))?;
                contents.push(FilterContent {
                    index: idx,
                    label,
                    content,
                });
            }
        }
        
        info!("Extracted {} filter variations", contents.len());
        Ok(contents)
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
