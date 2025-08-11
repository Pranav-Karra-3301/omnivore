# Omnivore

A high-performance web crawler and content extraction framework written in Rust.

## Features

### ✅ Implemented
- **Parallel web crawling** with configurable concurrency
- **HTML content extraction** using CSS selectors and rules
- **Git repository analysis** with intelligent code extraction
- **Metadata extraction** (OpenGraph, Twitter Cards, JSON-LD)
- **Politeness controls** with rate limiting and delays
- **RocksDB storage** for crawled content
- **REST API** for programmatic access
- **CLI interface** with progress tracking
- **TOML configuration** for flexible setup
- **Smart code filtering** for repository analysis

### 🚧 In Development
- Knowledge graph construction
- Advanced browser automation
- Vector embeddings and search
- Entity and relation extraction
- Robots.txt rule parsing

## Quick Links

- [Installation](installation.md) - Get Omnivore running on your system
- [Quickstart](quickstart.md) - Start crawling in 5 minutes
- [CLI Reference](cli.md) - Command-line interface documentation
- [Git Command](cli-git.md) - Extract and analyze code from repositories
- [API Documentation](api/rest.md) - REST API endpoints
- [Configuration](configuration.md) - Customize crawler behavior

## Use Cases

Omnivore is ideal for:
- **Web scraping** - Extract structured data from websites
- **Code analysis** - Extract and analyze source code from Git repositories
- **Content archival** - Save website content locally
- **Data mining** - Collect data for analysis
- **Site monitoring** - Track changes over time
- **Research** - Gather information systematically
- **Codebase documentation** - Generate reports of repository structure

## Architecture Overview

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│     CLI     │────▶│    Core     │────▶│   Storage   │
└─────────────┘     │   Crawler   │     │  (RocksDB)  │
                    └─────────────┘     └─────────────┘
┌─────────────┐            │
│   REST API  │────────────┘
└─────────────┘
```

## Getting Started

1. **Install Omnivore** using Cargo:
   ```bash
   cargo install omnivore
   ```

2. **Run your first crawl**:
   ```bash
   omnivore crawl https://example.com --depth 2
   ```

3. **Start the API server**:
   ```bash
   omnivore-api
   ```

## Project Status

Omnivore is actively developed with a focus on stability and performance. The core crawling functionality is production-ready, while advanced features like knowledge graphs are under development.

See our [GitHub repository](https://github.com/Pranav-Karra-3301/omnivore) for the latest updates.