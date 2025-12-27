# AGENTS.md - AI Assistant Guidelines for Omnivore

> Configuration for GitHub Copilot, OpenAI Codex, and other AI coding assistants.

---

## Project Overview

**Omnivore** is a Rust-based web crawler and knowledge graph builder. This document helps AI assistants understand the project's patterns, requirements, and constraints to generate appropriate, safe code.

### Repository

- **Language**: Rust (Edition 2021, requires 1.88+)
- **License**: MIT OR Apache-2.0
- **Repository**: https://github.com/Pranav-Karra-3301/omnivore
- **Status**: Experimental/Personal project - not production-ready

### Primary Components

| Package | Description | Location |
|---------|-------------|----------|
| `omnivore-core` | Core crawler library | `/omnivore-core/src/` |
| `omnivore-cli` | Command-line interface | `/omnivore-cli/src/` |

---

## Project Identity & Goals

### What This Project Does

1. **Web Crawling**: High-performance async crawling with 1000+ concurrent connections
2. **Content Extraction**: CSS selectors, XPath, AI-powered natural language queries
3. **Knowledge Graphs**: Build graph structures from extracted web data
4. **Git Analysis**: Extract and analyze code from Git repositories
5. **API Server**: REST and GraphQL interfaces for programmatic access

### Core Values

- **Performance**: Tokio-based async, parallel processing, efficient memory use
- **Ethical Crawling**: robots.txt compliance, rate limiting, politeness controls
- **Extensibility**: Pluggable extractors, storage backends, output formats
- **Safety**: Proper error handling, no unwraps, secure defaults

---

## Safety & Security Guidelines

### CRITICAL: Never Generate Code That...

```rust
// NEVER DO THESE:

// 1. Ignores errors
let data = fetch_url(url).unwrap();  // BAD: Will panic
let _ = fallible_operation();         // BAD: Silently ignores error

// 2. Bypasses safety controls
config.respect_robots_txt = false;    // BAD: Unethical
rate_limiter.disable();               // BAD: Can cause DoS

// 3. Introduces vulnerabilities
let query = format!("SELECT * FROM pages WHERE url = '{}'", user_input);  // SQL injection
Command::new("sh").arg("-c").arg(user_input).spawn();  // Command injection

// 4. Uses unsafe without justification
unsafe { std::ptr::read(ptr) }  // BAD: Needs documentation and review
```

### ALWAYS: Safe Patterns

```rust
// CORRECT: Proper error handling
let data = fetch_url(url)
    .await
    .map_err(|e| CrawlError::Network(e))?;

// CORRECT: Respect crawling ethics
if !self.robots_checker.is_allowed(url).await {
    return Err(CrawlError::DisallowedByRobots);
}

// CORRECT: Parameterized queries
sqlx::query("SELECT * FROM pages WHERE url = ?")
    .bind(&url)
    .fetch_one(&pool)
    .await?;

// CORRECT: Validated input for commands
let validated_path = validate_file_path(user_input)?;
```

---

## Coding Standards

### Error Handling

Use the project's established patterns:

```rust
// In library code (omnivore-core): use thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlError {
    #[error("Failed to fetch URL: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("URL parsing failed: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Rate limit exceeded for {domain}")]
    RateLimited { domain: String },
}

// In application code (omnivore-cli): use anyhow
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_config()
        .context("Failed to load configuration")?;
    Ok(())
}
```

### Async Code

```rust
use tokio::sync::Semaphore;

pub struct Crawler {
    client: reqwest::Client,
    concurrency: Arc<Semaphore>,
}

impl Crawler {
    pub async fn crawl_many(&self, urls: Vec<Url>) -> Vec<Result<Page, CrawlError>> {
        let futures: Vec<_> = urls.into_iter()
            .map(|url| self.crawl_one(url))
            .collect();

        futures::future::join_all(futures).await
    }

    async fn crawl_one(&self, url: Url) -> Result<Page, CrawlError> {
        let _permit = self.concurrency.acquire().await?;
        // Automatic concurrency limiting
        self.fetch_and_parse(url).await
    }
}
```

### Documentation

Document all public APIs:

```rust
/// Extracts structured data from an HTML document.
///
/// # Arguments
///
/// * `html` - The HTML content to parse
/// * `selectors` - CSS selectors for data extraction
///
/// # Returns
///
/// A `Vec<ExtractedData>` containing all matched elements.
///
/// # Errors
///
/// Returns `ParseError::InvalidSelector` if a selector is malformed.
///
/// # Examples
///
/// ```
/// use omnivore_core::parser::extract;
///
/// let html = "<div class='title'>Hello</div>";
/// let selectors = vec![".title"];
/// let data = extract(html, &selectors)?;
/// assert_eq!(data[0].text, "Hello");
/// ```
pub fn extract(html: &str, selectors: &[&str]) -> Result<Vec<ExtractedData>, ParseError> {
    // Implementation
}
```

---

## Key Dependencies

When generating code, use these established crates:

| Purpose | Crate | Usage Example |
|---------|-------|---------------|
| HTTP client | `reqwest` | `reqwest::Client::new().get(url).send().await?` |
| HTML parsing | `scraper` | `Html::parse_document(html)` |
| Async runtime | `tokio` | `#[tokio::main]` or `#[tokio::test]` |
| Serialization | `serde` | `#[derive(Serialize, Deserialize)]` |
| CLI args | `clap` | `#[derive(Parser)]` |
| Logging | `tracing` | `tracing::info!("message")` |
| Error types | `thiserror` | `#[derive(Error)]` |
| App errors | `anyhow` | `anyhow::Result<T>` |

### Import Patterns

```rust
// Standard imports for omnivore-core
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::{debug, error, info, warn};
use url::Url;

// For async code
use futures::stream::{self, StreamExt};
use tokio::task::JoinHandle;
```

---

## Project Structure

```
omnivore-core/src/
├── lib.rs              # Public exports
├── config.rs           # Configuration structs
├── error.rs            # Error types
├── crawler/            # Web crawling engine
│   ├── mod.rs          # Main crawler struct
│   ├── frontier.rs     # URL queue management
│   ├── politeness.rs   # Rate limiting
│   └── robots.rs       # robots.txt handling
├── parser/             # HTML/content parsing
│   ├── mod.rs
│   └── extractors.rs   # Data extractors
├── intelligence/       # NLP and entity extraction
│   └── entity.rs       # Email, phone, date detection
├── storage/            # Data persistence
│   ├── kv.rs           # Key-value store
│   └── graph_db.rs     # Graph database
└── graph/              # Knowledge graph

omnivore-cli/src/
├── main.rs             # CLI entry point
├── lib.rs              # Library exports
├── setup.rs            # Interactive setup wizard
└── git/                # Git repository analysis
    ├── mod.rs
    ├── detector.rs     # Project type detection
    └── filter.rs       # Smart file filtering
```

---

## Common Tasks

### Adding a New Extractor

```rust
// In omnivore-core/src/parser/extractors.rs

use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProductData {
    pub name: String,
    pub price: Option<f64>,
    pub description: Option<String>,
}

pub fn extract_products(html: &Html) -> Vec<ProductData> {
    let product_selector = Selector::parse(".product").unwrap();
    let name_selector = Selector::parse(".product-name").unwrap();
    let price_selector = Selector::parse(".price").unwrap();

    html.select(&product_selector)
        .map(|el| {
            let name = el.select(&name_selector)
                .next()
                .map(|n| n.text().collect())
                .unwrap_or_default();

            let price = el.select(&price_selector)
                .next()
                .and_then(|p| parse_price(&p.text().collect::<String>()));

            ProductData { name, price, description: None }
        })
        .collect()
}

fn parse_price(text: &str) -> Option<f64> {
    text.chars()
        .filter(|c| c.is_numeric() || *c == '.')
        .collect::<String>()
        .parse()
        .ok()
}
```

### Adding a CLI Command

```rust
// In omnivore-cli/src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "omnivore")]
#[command(about = "Universal web crawler and knowledge graph builder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Crawl a website
    Crawl(CrawlArgs),

    /// Analyze a Git repository
    Git(GitArgs),

    /// Your new command
    NewCommand(NewCommandArgs),
}

#[derive(Parser)]
pub struct NewCommandArgs {
    /// Description of the argument
    #[arg(short, long)]
    pub flag: bool,

    /// Required positional argument
    pub input: String,
}
```

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_normalization() {
        let result = normalize_url("HTTPS://Example.COM/Path?b=2&a=1");
        assert_eq!(result.unwrap().as_str(), "https://example.com/path?a=1&b=2");
    }

    #[tokio::test]
    async fn test_concurrent_crawling() {
        let crawler = Crawler::new(CrawlConfig {
            max_workers: 5,
            ..Default::default()
        });

        let urls = vec!["https://a.com", "https://b.com", "https://c.com"];
        let results = crawler.crawl_all(urls).await;

        assert_eq!(results.len(), 3);
    }
}
```

---

## DO NOT

### Never Generate

1. **Unwrap calls in production code**
   ```rust
   // BAD
   let data = some_result.unwrap();

   // GOOD
   let data = some_result?;
   ```

2. **Blocking code in async contexts**
   ```rust
   // BAD
   std::fs::read_to_string(path)  // Blocks the async runtime

   // GOOD
   tokio::fs::read_to_string(path).await
   ```

3. **Hardcoded secrets**
   ```rust
   // BAD
   let api_key = "sk-1234567890";

   // GOOD
   let api_key = std::env::var("API_KEY")?;
   ```

4. **Unvalidated user input in commands**
   ```rust
   // BAD
   Command::new("sh").arg("-c").arg(&user_input);

   // GOOD
   // Validate and sanitize first, or avoid shell entirely
   ```

5. **Ignoring robots.txt**
   ```rust
   // BAD - bypassing crawl restrictions
   crawl_url_directly(url)

   // GOOD - checking permissions
   if robots_checker.is_allowed(url) {
       crawl_url(url)
   }
   ```

---

## Testing Requirements

Before suggesting code is complete:

1. Run `cargo fmt --all -- --check`
2. Run `cargo clippy --all-targets --all-features -- -D warnings`
3. Run `cargo test --workspace`
4. For new features, ensure tests are included

### Test Patterns

```rust
// Unit test
#[test]
fn test_feature() {
    assert_eq!(expected, actual);
}

// Async test
#[tokio::test]
async fn test_async_feature() {
    let result = async_function().await;
    assert!(result.is_ok());
}

// With HTTP mocking
#[tokio::test]
async fn test_with_mock_server() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/api")
        .with_status(200)
        .with_body(r#"{"status": "ok"}"#)
        .create();

    let client = Client::new(&server.url());
    let response = client.get("/api").await.unwrap();

    mock.assert();
    assert_eq!(response.status, "ok");
}
```

---

## Quick Commands

```bash
# Development
cargo build                    # Debug build
cargo build --release          # Release build
cargo run --bin omnivore       # Run CLI

# Testing
cargo test                     # All tests
cargo test --lib               # Unit tests only
cargo test test_name           # Specific test

# Quality
cargo fmt                      # Format code
cargo clippy                   # Lint
cargo audit                    # Security check

# Documentation
cargo doc --open               # Generate and view docs
```

---

## File Conventions

| Pattern | Convention |
|---------|------------|
| Module files | `snake_case.rs` |
| Type names | `PascalCase` |
| Functions | `snake_case` |
| Constants | `SCREAMING_SNAKE_CASE` |
| Feature flags | `kebab-case` in Cargo.toml |

---

*This file configures AI coding assistants. For human developers, see [README.md](README.md).*
