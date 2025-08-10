use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Rate limit exceeded for domain: {0}")]
    RateLimitExceeded(String),

    #[error("Robots.txt disallows crawling: {0}")]
    RobotsDisallowed(String),

    #[error("Maximum depth reached: {0}")]
    MaxDepthReached(u32),

    #[error("Timeout after {0} ms")]
    Timeout(u64),

    #[error("Browser automation error: {0}")]
    Browser(String),
    
    #[error("WebDriver error: {0}")]
    WebDriver(#[from] thirtyfour::error::WebDriverError),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Graph error: {0}")]
    Graph(String),

    #[error("Intelligence processing error: {0}")]
    Intelligence(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
