use anyhow::{Context, Result};
use thirtyfour::prelude::*;
use thirtyfour::{ChromeCapabilities, FirefoxCapabilities};
use std::time::Duration;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub browser_type: BrowserType,
    pub headless: bool,
    pub page_load_timeout: u64,
    pub script_timeout: u64,
    pub implicit_wait: u64,
    pub driver_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrowserType {
    Chrome,
    Firefox,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser_type: BrowserType::Chrome,
            headless: true,
            page_load_timeout: 30,
            script_timeout: 30,
            implicit_wait: 10,
            driver_url: None,
        }
    }
}

pub struct BrowserEngine {
    driver: WebDriver,
    config: BrowserConfig,
}

impl BrowserEngine {
    pub async fn new(config: BrowserConfig) -> Result<Self> {
        let driver_url = config.driver_url.clone()
            .unwrap_or_else(|| "http://localhost:4444".to_string());
        
        let driver = match config.browser_type {
            BrowserType::Chrome => {
                let mut caps = ChromeCapabilities::new();
                if config.headless {
                    caps.add_chrome_arg("--headless")?;
                }
                caps.add_chrome_arg("--no-sandbox")?;
                caps.add_chrome_arg("--disable-dev-shm-usage")?;
                caps.add_chrome_arg("--disable-gpu")?;
                caps.add_chrome_arg("--window-size=1920,1080")?;
                caps.add_chrome_arg("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")?;
                
                WebDriver::new(&driver_url, caps).await
                    .context("Failed to create Chrome WebDriver")?
            }
            BrowserType::Firefox => {
                let mut caps = FirefoxCapabilities::new();
                if config.headless {
                    caps.add_firefox_arg("-headless")?;
                }
                caps.add_firefox_arg("-width=1920")?;
                caps.add_firefox_arg("-height=1080")?;
                
                WebDriver::new(&driver_url, caps).await
                    .context("Failed to create Firefox WebDriver")?
            }
        };
        
        // Set timeouts
        driver.set_page_load_timeout(Duration::from_secs(config.page_load_timeout)).await?;
        driver.set_script_timeout(Duration::from_secs(config.script_timeout)).await?;
        driver.set_implicit_wait_timeout(Duration::from_secs(config.implicit_wait)).await?;
        
        Ok(Self { driver, config })
    }
    
    pub async fn navigate(&self, url: &str) -> Result<()> {
        self.driver.goto(url).await
            .context(format!("Failed to navigate to {}", url))?;
        
        // Wait for page to be ready
        self.wait_for_page_ready().await?;
        
        Ok(())
    }
    
    pub async fn wait_for_page_ready(&self) -> Result<()> {
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
            
            match self.driver.execute(script, vec![]).await {
                Ok(ret) => {
                    if let Ok(ready) = ret.value().as_bool() {
                        if ready {
                            debug!("Page is ready");
                            break;
                        }
                    }
                }
                Err(_) => {
                    // Script execution failed, page might not have jQuery/Angular
                    // Check basic readyState
                    let basic_script = "return document.readyState === 'complete';";
                    if let Ok(ret) = self.driver.execute(basic_script, vec![]).await {
                        if let Ok(ready) = ret.value().as_bool() {
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
    
    pub async fn find_dropdowns(&self) -> Result<Vec<WebElement>> {
        let selectors = vec![
            "select",
            "div[role='combobox']",
            "div[role='listbox']",
            ".dropdown",
            ".select-wrapper",
            "[data-toggle='dropdown']",
        ];
        
        let mut dropdowns = Vec::new();
        
        for selector in selectors {
            match self.driver.find_all(By::Css(selector)).await {
                Ok(elements) => dropdowns.extend(elements),
                Err(_) => continue,
            }
        }
        
        Ok(dropdowns)
    }
    
    pub async fn find_filters(&self) -> Result<Vec<WebElement>> {
        let selectors = vec![
            "input[type='checkbox']",
            "input[type='radio']",
            ".filter",
            ".filter-option",
            "[data-filter]",
            "[role='checkbox']",
            "[role='radio']",
        ];
        
        let mut filters = Vec::new();
        
        for selector in selectors {
            match self.driver.find_all(By::Css(selector)).await {
                Ok(elements) => filters.extend(elements),
                Err(_) => continue,
            }
        }
        
        Ok(filters)
    }
    
    pub async fn interact_with_dropdown(&self, dropdown: &WebElement) -> Result<Vec<String>> {
        let mut contents = Vec::new();
        
        // Check if it's a select element
        if dropdown.tag_name().await?.to_lowercase() == "select" {
            let options = dropdown.find_all(By::Css("option")).await?;
            
            for option in options {
                // Click the option
                if let Ok(_) = option.click().await {
                    sleep(Duration::from_millis(1000)).await;
                    self.wait_for_page_ready().await?;
                    
                    // Get page content
                    let content = self.get_page_content().await?;
                    contents.push(content);
                }
            }
        } else {
            // Handle custom dropdowns
            if let Ok(_) = dropdown.click().await {
                sleep(Duration::from_millis(500)).await;
                
                // Look for dropdown items
                let item_selectors = vec![
                    "li",
                    ".dropdown-item",
                    "[role='option']",
                    ".option",
                ];
                
                for selector in item_selectors {
                    if let Ok(items) = self.driver.find_all(By::Css(selector)).await {
                        for item in items {
                            if let Ok(_) = item.click().await {
                                sleep(Duration::from_millis(1000)).await;
                                self.wait_for_page_ready().await?;
                                
                                let content = self.get_page_content().await?;
                                contents.push(content);
                                
                                // Re-open dropdown for next item
                                dropdown.click().await.ok();
                                sleep(Duration::from_millis(500)).await;
                            }
                        }
                        break;
                    }
                }
            }
        }
        
        Ok(contents)
    }
    
    pub async fn interact_with_filter(&self, filter: &WebElement) -> Result<String> {
        filter.click().await?;
        sleep(Duration::from_millis(1000)).await;
        self.wait_for_page_ready().await?;
        
        self.get_page_content().await
    }
    
    pub async fn get_page_content(&self) -> Result<String> {
        let html = self.driver.source().await
            .context("Failed to get page source")?;
        Ok(html)
    }
    
    pub async fn scroll_to_bottom(&self) -> Result<()> {
        let script = "window.scrollTo(0, document.body.scrollHeight);";
        self.driver.execute(script, vec![]).await?;
        sleep(Duration::from_millis(1000)).await;
        Ok(())
    }
    
    pub async fn infinite_scroll(&self, max_scrolls: u32) -> Result<()> {
        let mut last_height: i64 = 0;
        
        for _ in 0..max_scrolls {
            // Get current scroll height
            let script = "return document.body.scrollHeight;";
            let height_result = self.driver.execute(script, vec![]).await?;
            let current_height = height_result.value().as_i64().unwrap_or(0);
            
            if current_height == last_height {
                // No more content to load
                break;
            }
            
            last_height = current_height;
            
            // Scroll to bottom
            self.scroll_to_bottom().await?;
            
            // Wait for new content to load
            sleep(Duration::from_secs(2)).await;
        }
        
        Ok(())
    }
    
    pub async fn extract_dynamic_content(&self, url: &str) -> Result<DynamicContent> {
        info!("Extracting dynamic content from: {}", url);
        
        self.navigate(url).await?;
        
        // Check for infinite scroll
        let initial_height = self.driver
            .execute("return document.body.scrollHeight;", vec![])
            .await?
            .value()
            .as_i64()
            .unwrap_or(0);
        
        self.scroll_to_bottom().await?;
        sleep(Duration::from_secs(2)).await;
        
        let new_height = self.driver
            .execute("return document.body.scrollHeight;", vec![])
            .await?
            .value()
            .as_i64()
            .unwrap_or(0);
        
        let has_infinite_scroll = new_height > initial_height;
        
        if has_infinite_scroll {
            info!("Detected infinite scroll, loading all content...");
            self.infinite_scroll(10).await?;
        }
        
        // Get main content
        let main_content = self.get_page_content().await?;
        
        // Find interactive elements
        let dropdowns = self.find_dropdowns().await?;
        let filters = self.find_filters().await?;
        
        info!("Found {} dropdowns and {} filters", dropdowns.len(), filters.len());
        
        let mut dropdown_contents = Vec::new();
        let mut filter_contents = Vec::new();
        
        // Interact with dropdowns
        for (idx, dropdown) in dropdowns.iter().enumerate() {
            info!("Processing dropdown {}/{}", idx + 1, dropdowns.len());
            match self.interact_with_dropdown(dropdown).await {
                Ok(contents) => {
                    for content in contents {
                        dropdown_contents.push(DropdownContent {
                            index: idx,
                            content,
                        });
                    }
                }
                Err(e) => warn!("Failed to interact with dropdown {}: {}", idx, e),
            }
        }
        
        // Interact with filters
        for (idx, filter) in filters.iter().enumerate() {
            info!("Processing filter {}/{}", idx + 1, filters.len());
            match self.interact_with_filter(filter).await {
                Ok(content) => {
                    filter_contents.push(FilterContent {
                        index: idx,
                        content,
                    });
                }
                Err(e) => warn!("Failed to interact with filter {}: {}", idx, e),
            }
        }
        
        Ok(DynamicContent {
            url: url.to_string(),
            main_content,
            dropdown_contents,
            filter_contents,
            has_infinite_scroll,
        })
    }
    
    pub async fn quit(self) -> Result<()> {
        self.driver.quit().await
            .context("Failed to quit WebDriver")?;
        Ok(())
    }
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
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterContent {
    pub index: usize,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_browser_config() {
        let config = BrowserConfig::default();
        assert!(config.headless);
        assert_eq!(config.page_load_timeout, 30);
    }
}