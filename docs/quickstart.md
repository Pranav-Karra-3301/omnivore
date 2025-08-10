# Quickstart Guide

Get started with Omnivore in 5 minutes.

## Prerequisites

- Rust 1.70+ installed
- Git for cloning the repository
- 4GB RAM minimum
- Internet connection for crawling

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Build and install
cargo build --release
cargo install --path omnivore-cli
cargo install --path omnivore-api
```

### Using Cargo

```bash
cargo install omnivore
```

## Basic Usage

### 1. Simple Web Crawl

Crawl a website with default settings:

```bash
omnivore crawl https://example.com
```

You'll see a progress bar showing:
- URLs visited and queued
- Success/error counts
- Pages per second
- Time elapsed

### 2. Controlled Crawling

Limit depth and workers for controlled crawling:

```bash
omnivore crawl https://example.com \
  --depth 2 \
  --workers 5 \
  --delay 500
```

Parameters:
- `--depth 2`: Only follow links 2 levels deep
- `--workers 5`: Use 5 concurrent workers
- `--delay 500`: Wait 500ms between requests

### 3. Export Statistics

Save crawl statistics to a file:

```bash
omnivore crawl https://example.com \
  --output crawl-stats.json
```

The JSON file contains:
- Total URLs visited
- Success/failure counts
- Timing information
- Average response times

### 4. Parse HTML Files

Extract data from downloaded HTML:

```bash
# Download a page
curl https://example.com > page.html

# Parse and extract data
omnivore parse page.html
```

Output includes:
- Page title
- Meta descriptions
- OpenGraph tags
- Extracted links
- Structured data (JSON-LD)

## API Usage

### Start the API Server

```bash
omnivore-api
# Server starts on http://localhost:3000
```

### Start a Crawl via API

```bash
curl -X POST http://localhost:3000/api/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "max_depth": 3,
    "max_workers": 10
  }'
```

Response:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "started",
  "message": "Crawl started for URL: https://example.com"
}
```

### Check Statistics

```bash
curl http://localhost:3000/api/stats
```

Response:
```json
{
  "status": "completed",
  "stats": {
    "urls_visited": 42,
    "success_count": 40,
    "error_count": 2,
    "duration_secs": 15
  }
}
```

## Configuration File

Create a `config.toml` for persistent settings:

```toml
[crawler]
max_workers = 10
max_depth = 5
user_agent = "Omnivore/0.1.0"
respect_robots_txt = true

[crawler.politeness]
default_delay_ms = 200
max_retries = 3
timeout_secs = 30

[storage]
data_dir = "~/.omnivore/data"
```

Use with:
```bash
omnivore -c config.toml crawl https://example.com
```

## Common Scenarios

### Polite Crawling

For production sites, be respectful:

```bash
omnivore crawl https://example.com \
  --workers 2 \
  --delay 1000 \
  --respect-robots \
  --user-agent "MyBot/1.0 (contact@example.com)"
```

### Deep Analysis

For thorough site analysis:

```bash
omnivore crawl https://example.com \
  --depth 10 \
  --workers 20 \
  --output full-analysis.json
```

### Quick Preview

For a quick site overview:

```bash
omnivore crawl https://example.com \
  --depth 1 \
  --workers 5
```

## What's Working

✅ **Functional Features:**
- Multi-threaded web crawling
- HTML parsing and data extraction
- Metadata extraction (OpenGraph, Twitter Cards)
- Link discovery and following
- Basic statistics tracking
- Progress visualization
- REST API for programmatic access
- Configuration file support

## What's Not Working Yet

⚠️ **Under Development:**
- Knowledge graph construction (`graph` command shows placeholder only)
- Session persistence (`stats` command limited)
- Robots.txt parsing (fetches but doesn't parse rules)
- JavaScript rendering (static HTML only)
- Authentication for protected sites
- Advanced entity extraction

## Next Steps

1. **Explore Configuration**: See [Configuration](configuration.md) for advanced options
2. **Use the API**: Check [REST API](api/rest.md) for programmatic access
3. **Understand Architecture**: Read [Architecture Overview](architecture/overview.md)
4. **Learn Extraction**: See [Writing Extractors](guides/extractors.md)

## Troubleshooting

### "Connection refused" errors
- The site may be blocking crawlers
- Try adding delays: `--delay 1000`
- Use a custom user agent

### High memory usage
- Reduce workers: `--workers 2`
- Limit depth: `--depth 2`

### No data extracted
- Check if the site uses JavaScript rendering
- Verify the URL is accessible
- Try with `--verbose` for debug output

## Getting Help

- [GitHub Issues](https://github.com/Pranav-Karra-3301/omnivore/issues)
- [Documentation](https://github.com/Pranav-Karra-3301/omnivore/docs)
- Run `omnivore --help` for command options