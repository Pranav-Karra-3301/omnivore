# CLAUDE.md - Claude Code Configuration for Omnivore

> This file provides context and guidelines for Claude Code when working on the Omnivore project.

---

## Project Identity

**Omnivore** is a high-performance, parallel web crawler and knowledge graph system built in Rust. It's designed for scale, intelligence, and respectable crawling practices.

### What Omnivore Is

- A **universal web crawler** with async/parallel processing capabilities
- A **knowledge graph builder** that extracts and structures information from web data
- A **Git repository analyzer** for intelligent code extraction and analysis
- A **CLI tool** with REST/GraphQL API capabilities
- An **AI-powered content extractor** using natural language queries

### What Omnivore Is NOT

- This is **not production-ready software** - it's a vibecoded personal project with experimental features
- Not a replacement for established crawling frameworks - it's a learning/research tool
- Not a malicious scraping tool - it respects robots.txt and implements politeness controls

### Project Goals

1. **Performance**: Process 10,000+ pages per minute with sub-100ms graph query latency
2. **Intelligence**: AI-powered extraction with natural language queries
3. **Respectability**: robots.txt compliance, rate limiting, and ethical crawling
4. **Extensibility**: Pluggable architecture for custom extractors and storage backends
5. **Developer Experience**: Clear CLI interface with comprehensive documentation

---

## Philosophy & Guidelines

### Safety First

When generating or modifying code in this project:

1. **Never introduce security vulnerabilities**
   - No command injection, XSS, SQL injection, or OWASP Top 10 issues
   - Validate all external inputs at system boundaries
   - Use parameterized queries, proper escaping, and input sanitization

2. **Respect rate limiting and politeness**
   - Always honor robots.txt directives
   - Implement exponential backoff for retries
   - Never bypass rate limiting controls

3. **Handle errors gracefully**
   - Use `Result<T, E>` types appropriately
   - Propagate errors with context using `anyhow` or `thiserror`
   - Never unwrap in production code paths - use `?` operator or explicit handling

4. **Memory and resource safety**
   - Avoid unbounded allocations
   - Use streaming for large data processing
   - Clean up resources with proper drop semantics

### Code Quality Standards

```rust
// GOOD: Clear, safe, documented
/// Fetches a URL with proper error handling and rate limiting.
///
/// # Errors
/// Returns `CrawlError::RateLimited` if the domain's rate limit is exceeded.
/// Returns `CrawlError::NetworkError` for connection failures.
pub async fn fetch_url(url: &Url, config: &CrawlConfig) -> Result<Response, CrawlError> {
    self.politeness.wait_for_permission(url.domain()).await?;
    let response = self.client.get(url.as_str()).send().await?;
    Ok(response)
}

// BAD: Unsafe, unclear, no error handling
pub async fn fetch_url(url: &str) -> Response {
    reqwest::get(url).await.unwrap()  // Never do this
}
```

### Explanation Over Brevity

When writing code:
- **Add doc comments** for public APIs explaining purpose, parameters, errors, and examples
- **Use descriptive variable names** - `crawl_depth` not `d`
- **Comment complex algorithms** - explain the "why" not just the "what"
- **Include usage examples** in documentation

---

## Tech Stack

### Core Technologies

| Technology | Purpose | Version |
|------------|---------|---------|
| **Rust** | Primary language | Edition 2021, 1.88+ |
| **Tokio** | Async runtime | 1.43+ (full features) |
| **Reqwest** | HTTP client | 0.12 |
| **Scraper** | HTML parsing | 0.22 |
| **Petgraph** | Graph algorithms | 0.6 |
| **Clap** | CLI framework | 4.5 |
| **Serde** | Serialization | 1.0 |
| **Axum** | Web framework | 0.8 |

### Workspace Structure

```
omnivore/
├── omnivore-core/     # Core library (crawler, parser, intelligence, storage)
├── omnivore-cli/      # CLI application
├── docs/              # MkDocs documentation
├── configs/           # Configuration examples
├── scripts/           # Build and utility scripts
└── Cargo.toml         # Workspace manifest
```

### Key Dependencies to Know

- **Error handling**: `anyhow` (application errors), `thiserror` (library errors)
- **Async traits**: `async-trait` for async fn in traits
- **Concurrent data**: `dashmap` for concurrent hashmaps, `parking_lot` for locks
- **Testing**: `mockito` (HTTP mocking), `proptest` (property testing), `criterion` (benchmarks)
- **Git integration**: `git2` with vendored OpenSSL

---

## Workflows & Commands

### Development Commands

```bash
# Setup development environment
make setup              # Install clippy, rustfmt, cargo-audit, cargo-watch

# Build commands
make build              # Release build
make build-debug        # Debug build
cargo build -p omnivore-core   # Build specific package

# Run the CLI
cargo run --bin omnivore -- crawl https://example.com
cargo run --bin omnivore -- git https://github.com/user/repo

# Watch mode for development
make watch              # Rebuild on file changes
```

### Testing Commands

```bash
# Run all tests
make test               # or: cargo test --workspace

# Specific test types
make test-unit          # Unit tests only
make test-integration   # Integration tests
cargo test --doc        # Documentation tests

# Run specific test
cargo test test_crawler_creation

# With coverage
make ci-coverage        # Generates LCOV report
```

### Quality Checks

```bash
# Full CI suite (run before committing)
make ci                 # fmt-check → clippy → test → audit

# Individual checks
make fmt                # Format code
make fmt-check          # Check formatting without modifying
make clippy             # Run linter (warnings as errors)
make audit              # Security vulnerability check

# Pre-release checks
make release-prep       # Full validation suite
```

### Documentation

```bash
make docs               # Generate and open rustdoc
make docs-build         # Build docs only
```

### Benchmarking

```bash
make bench              # Run criterion benchmarks
cargo bench --bench crawler_bench   # Specific benchmark
```

---

## Git Workflow

### Branch Naming

```
feature/description     # New features
fix/issue-description   # Bug fixes
refactor/what-changed   # Code refactoring
docs/what-documented    # Documentation updates
```

### Commit Message Format

```
<type>: <short description>

[Optional longer description]

[Optional: Fixes #123]
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`

Example:
```
feat: Add table extraction for HTML crawling

Implements automatic detection of HTML tables during crawling
and exports them as CSV files. Tables are identified using
semantic HTML5 tags and heuristics for data tables.

Fixes #45
```

### Before Committing

1. Run `make ci` to ensure all checks pass
2. Write meaningful commit messages
3. Keep commits atomic and focused
4. Update documentation if changing public APIs

---

## Releases & Publishing

### Version Numbers

- Follow Semantic Versioning: `MAJOR.MINOR.PATCH`
- Core library and CLI may have independent versions
- Update `Cargo.toml` workspace version for releases

### Publishing to crates.io

```bash
# 1. Publish core library first
cd omnivore-core && cargo publish

# 2. Wait for crates.io indexing (~2-5 minutes)

# 3. Update omnivore-cli to use published version (not path)

# 4. Publish CLI
cd omnivore-cli && cargo publish
```

### Creating a Release

```bash
# 1. Update version numbers in Cargo.toml files
# 2. Update CHANGELOG.md
# 3. Create git tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# 4. GitHub Actions will build binaries and create release
```

---

## Things to NEVER Do

### Code Safety

- **NEVER** use `.unwrap()` or `.expect()` in production code paths
- **NEVER** ignore error results with `let _ = fallible_operation()`
- **NEVER** use `unsafe` without extensive documentation and justification
- **NEVER** introduce panicking code in library functions
- **NEVER** hardcode credentials, API keys, or secrets

### Crawling Ethics

- **NEVER** bypass robots.txt directives
- **NEVER** remove or disable rate limiting
- **NEVER** spoof user agents to impersonate browsers maliciously
- **NEVER** store or expose personal data without explicit consent
- **NEVER** crawl private or authenticated pages without authorization

### Project Standards

- **NEVER** commit code that fails `cargo clippy`
- **NEVER** skip tests to merge faster
- **NEVER** remove existing test coverage
- **NEVER** force push to main/master branches
- **NEVER** commit `.env` files or credentials

### Architecture

- **NEVER** add dependencies without justification
- **NEVER** break public API without version bump
- **NEVER** mix async and blocking code without proper handling
- **NEVER** create circular dependencies between modules

---

## Common Patterns

### Error Handling Pattern

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Rate limit exceeded for domain: {domain}")]
    RateLimited { domain: String },

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

// Usage: propagate with context
fn crawl_page(url: &Url) -> Result<Page, CrawlError> {
    let response = fetch(url)?;  // Automatic conversion
    // ...
}
```

### Async Pattern

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct Crawler {
    client: reqwest::Client,
    semaphore: Arc<Semaphore>,  // Limit concurrent requests
}

impl Crawler {
    pub async fn crawl(&self, url: Url) -> Result<Page, CrawlError> {
        let _permit = self.semaphore.acquire().await?;
        // Request is rate-limited by semaphore
        self.fetch_and_parse(url).await
    }
}
```

### Configuration Pattern

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CrawlConfig {
    #[serde(default = "default_workers")]
    pub max_workers: usize,

    #[serde(default)]
    pub respect_robots_txt: bool,
}

fn default_workers() -> usize { 10 }
```

---

## File Locations

| What | Where |
|------|-------|
| Core crawler logic | `omnivore-core/src/crawler/` |
| HTML parsing | `omnivore-core/src/parser/` |
| Entity extraction | `omnivore-core/src/intelligence/` |
| Storage backends | `omnivore-core/src/storage/` |
| CLI entry point | `omnivore-cli/src/main.rs` |
| Git analysis | `omnivore-cli/src/git/` |
| Configuration | `omnivore-core/src/config.rs` |
| Error types | `omnivore-core/src/error.rs` |

---

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_normalization() {
        let url = normalize_url("https://Example.COM/Path").unwrap();
        assert_eq!(url.as_str(), "https://example.com/path");
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks() {
        let limiter = RateLimiter::new(1, Duration::from_secs(1));
        limiter.acquire().await;

        // Second acquire should wait
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_millis(900));
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/crawler_integration.rs
use omnivore_core::Crawler;

#[tokio::test]
async fn test_full_crawl_workflow() {
    let mut server = mockito::Server::new();
    let mock = server.mock("GET", "/")
        .with_body("<html><body>Test</body></html>")
        .create();

    let crawler = Crawler::new(Default::default());
    let result = crawler.crawl(&server.url()).await;

    mock.assert();
    assert!(result.is_ok());
}
```

---

## Debugging Tips

### Enable Detailed Logging

```bash
RUST_LOG=debug cargo run --bin omnivore -- crawl https://example.com
RUST_LOG=omnivore_core::crawler=trace cargo run --bin omnivore -- crawl https://example.com
```

### Common Issues

1. **"Too many open files"**: Increase ulimit or reduce `max_workers`
2. **Slow crawling**: Check if rate limiting is too aggressive
3. **Memory growth**: Ensure crawl frontier is bounded
4. **SSL errors**: Update CA certificates or use `rustls`

---

## Quick Reference

```bash
# Build & Test
make build          # Release build
make test           # All tests
make ci             # Full CI checks

# Development
make watch          # Watch mode
cargo run --bin omnivore -- --help

# Quality
make fmt            # Format
make clippy         # Lint
make audit          # Security

# Docs
make docs           # Generate and view
```

---

*This document is for AI assistants. For human documentation, see [README.md](README.md) and [docs/](docs/).*
