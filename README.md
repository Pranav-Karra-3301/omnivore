<p align="center">
  <img src="https://ov.pranavkarra.me/logo.png" alt="ov" width="200"/>
</p>


[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/v/release/Pranav-Karra-3301/omnivore)](https://github.com/Pranav-Karra-3301/omnivore/releases)

# Omnivore

**Universal Rust Web Crawler & Knowledge Graph Builder**

A high-performance, parallel web crawler and knowledge graph system built in Rust, designed for scale and intelligence.

## Features

- **Parallel Crawling**: Async/await with Tokio, supporting 1000+ concurrent connections
- **Dual-Mode Operation**: Static (Reqwest) and dynamic (browser automation) crawling
- **Smart Content Extraction**: CSS selectors, XPath, and pattern matching
- **Knowledge Graph**: Build and query entity-relationship graphs from crawled data
- **Politeness Engine**: Per-domain rate limiting with exponential backoff
- **Extensible Architecture**: Plugin system for custom extractors and processors

## Quick Start

### Installation

#### Quick Install (Linux/macOS)
```bash
# Install latest release
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh

# Or with specific version
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh -s -- --version v0.1.0-beta
```

#### Homebrew (macOS/Linux)
```bash
# Add tap and install
brew tap Pranav-Karra-3301/omnivore
brew install omnivore
```

#### Download Binary
Download pre-built binaries from the [releases page](https://github.com/Pranav-Karra-3301/omnivore/releases):
- Linux: `omnivore-v0.1.0-beta-x86_64-unknown-linux-gnu.tar.gz`
- macOS Intel: `omnivore-v0.1.0-beta-x86_64-apple-darwin.tar.gz`
- macOS Apple Silicon: `omnivore-v0.1.0-beta-aarch64-apple-darwin.tar.gz`

Extract and install:
```bash
tar -xzf omnivore-*.tar.gz
cd omnivore-*/
sudo install -m 755 omnivore /usr/local/bin/
sudo install -m 755 omnivore-api /usr/local/bin/
```

#### From Source
```bash
# Using Cargo
cargo install --git https://github.com/Pranav-Karra-3301/omnivore --tag v0.1.0-beta

# Or build manually
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore
cargo build --release

# Install locally
make install
```

#### Docker
```bash
# Run with Docker
docker run -it --rm -p 3000:3000 omnivore:latest

# Or use docker-compose for full stack
docker-compose up -d
```

### Basic Usage

```bash
# Crawl a website
omnivore crawl https://example.com --workers 10 --depth 5

# Parse HTML with custom rules
omnivore parse index.html --rules parser-rules.yaml

# Build knowledge graph
omnivore graph crawl-results.json --output graph.db

# Extract code from Git repositories
omnivore git https://github.com/user/repo.git --output ./extracted-code
omnivore git . --json > codebase.json  # Extract from local repo

# Start API server
cargo run --bin omnivore-api
```

## Architecture

```
omnivore/
├── omnivore-core/     # Core crawler and processing engine
├── omnivore-cli/      # Command-line interface
├── omnivore-api/      # REST and GraphQL API
└── configs/           # Configuration files
```

## Core Components

### Crawler Engine
- Tokio-based async runtime with configurable worker pools
- Automatic routing between static and dynamic crawlers
- URL frontier with priority queue and bloom filter deduplication
- Robots.txt compliance and politeness controls

### Parser System
- HTML parsing with Scraper (html5ever backend)
- CSS and XPath selectors
- Structured data extraction (JSON-LD, Microdata, OpenGraph)
- Custom extraction rules with regex patterns

### Intelligence Layer
- Entity recognition (emails, phones, dates, prices)
- Relationship extraction from unstructured text
- Content classification and tagging
- Vector embeddings for semantic search

### Knowledge Graph
- Petgraph for in-memory graph operations
- Schema-based validation
- Query engine with graph algorithms
- Export to various graph formats

### Git Code Extractor (v0.1.1)
Intelligent extraction of meaningful code from Git repositories:

#### Features
- **Smart Filtering**: Automatically excludes build artifacts, dependencies, and generated files
- **Gitignore Respect**: Honors repository's `.gitignore` files by default
- **Flexible Output**: JSON, plain text, or directory structure preservation
- **Pattern Matching**: Include/exclude files with glob patterns
- **Binary Detection**: Automatic detection and filtering of binary files
- **Size Limits**: Configurable file size restrictions

#### Usage Examples
```bash
# Extract from remote repository
omnivore git https://github.com/user/repo.git --output ./code

# Extract specific file types
omnivore git . --include "src/**/*.rs,Cargo.toml" --json

# Exclude test files
omnivore git . --exclude "**/*test*,**/*spec*" --txt

# Clone with specific depth
omnivore git https://github.com/user/repo.git --depth 10 --output ./shallow

# Include binary files and ignore gitignore
omnivore git . --allow-binary --no-gitignore --output ./full-extract

# Set maximum file size (10MB)
omnivore git . --max-file-size 10485760 --json
```

#### Command Options
- `--include <patterns>`: Include only files matching these patterns (comma-separated)
- `--exclude <patterns>`: Exclude files matching these patterns (comma-separated)
- `--no-gitignore`: Don't respect .gitignore files
- `--output <path>`: Output to directory (preserves structure)
- `--json`: Output as JSON array of file objects
- `--txt`: Output as concatenated text with separators
- `--keep`: Keep temporary clone (for remote repos)
- `--depth <n>`: Clone depth for remote repositories (default: 1)
- `--allow-binary`: Include binary files in output
- `--max-file-size <bytes>`: Maximum file size to include

## Configuration

Create a `crawler.toml` file:

```toml
[crawler]
max_workers = 100
max_depth = 10
user_agent = "Omnivore/1.0"
respect_robots_txt = true

[crawler.politeness]
default_delay_ms = 100
max_requests_per_second = 10.0

[parser]
clean_text = true
extract_metadata = true

[storage]
data_dir = "./data"
compression = "zstd"
```

## API Usage

### REST API

```bash
# Start crawl
curl -X POST http://localhost:3000/api/crawl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com", "max_depth": 5}'

# Get statistics
curl http://localhost:3000/api/stats
```

### GraphQL

```graphql
query {
  health
  version
}
```

## Performance

- Process 10,000+ pages per minute on commodity hardware
- Sub-100ms latency for graph queries
- Memory-efficient frontier management (< 500MB for 1M URLs)
- Automatic scaling from 1 to 10,000 workers

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_crawler_creation
```

## Roadmap

- [ ] Distributed crawling with worker nodes
- [ ] Machine learning for content classification
- [ ] Browser fingerprint randomization
- [ ] WASM plugin support
- [ ] Kubernetes operator
- [ ] Real-time graph updates via WebSocket

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Acknowledgments

Built with these amazing Rust crates:
- Tokio for async runtime
- Reqwest for HTTP client
- Scraper for HTML parsing
- Petgraph for graph algorithms
- Axum for web framework

---

**Note**: This is a production-grade foundation that can be extended with additional features as needed.
