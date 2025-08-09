pub mod crawler;
pub mod error;
pub mod graph;
pub mod intelligence;
pub mod parser;
pub mod storage;

pub use error::{Error, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlConfig {
    pub max_workers: usize,
    pub max_depth: u32,
    pub user_agent: String,
    pub respect_robots_txt: bool,
    pub politeness: PolitenessConfig,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolitenessConfig {
    pub default_delay_ms: u64,
    pub max_requests_per_second: f64,
    pub backoff_multiplier: f64,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_workers: 10,
            max_depth: 10,
            user_agent: "Omnivore/1.0".to_string(),
            respect_robots_txt: true,
            politeness: PolitenessConfig::default(),
            timeout_ms: 30000,
            max_retries: 3,
        }
    }
}

impl Default for PolitenessConfig {
    fn default() -> Self {
        Self {
            default_delay_ms: 100,
            max_requests_per_second: 10.0,
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub url: String,
    pub status_code: u16,
    pub content: String,
    pub headers: std::collections::HashMap<String, String>,
    pub extracted_data: serde_json::Value,
    pub links: Vec<String>,
    pub crawled_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlStats {
    pub total_urls: usize,
    pub successful: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub average_response_time_ms: f64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub elapsed_time: std::time::Duration,
}
