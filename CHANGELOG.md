# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-beta] - 2024-08-10

### Added
- Initial beta release of Omnivore web crawler
- Multi-threaded web crawling with configurable concurrency (1-100 workers)
- HTML content extraction using CSS selectors and rule-based parsing
- Metadata extraction supporting OpenGraph, Twitter Cards, and JSON-LD
- Politeness engine with rate limiting and configurable delays
- RocksDB storage backend for persistent data storage
- REST API with endpoints for crawl management and statistics
- CLI interface with commands: crawl, parse, graph, stats, generate-completions
- Progress tracking with real-time statistics display
- TOML-based configuration system
- Shell completion generation for bash, zsh, fish, and powershell
- Comprehensive documentation with examples

### Known Issues
- Knowledge graph construction not fully implemented (basic structure only)
- Robots.txt fetched but rules not parsed
- No JavaScript rendering support (static HTML only)
- Session persistence not implemented across restarts
- No authentication support for protected sites
- Docker build failing due to benchmark file issues (Issue #1)
- Cross-compilation for ARM64 failing due to OpenSSL linking (Issue #2)

### Security
- Added cargo-audit configuration to suppress known vulnerabilities without fixes
- RUSTSEC-2023-0071: RSA vulnerability in sqlx-mysql (not using MySQL)
- RUSTSEC-2024-0436: paste crate unmaintained warning

### CI/CD
- Implemented GitHub Actions workflow for:
  - Testing on Ubuntu and macOS
  - Code coverage with cargo-llvm-cov
  - Security auditing with cargo-audit
  - Benchmark execution
  - Dependency checking
- Temporarily disabled Docker and cross-compilation jobs pending fixes

### Dependencies
- Core: tokio, reqwest, scraper, html5ever, rocksdb
- CLI: clap, indicatif, colored
- API: axum, tower, async-graphql
- Testing: mockito, proptest, criterion