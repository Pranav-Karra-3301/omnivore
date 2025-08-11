# Quickstart Guide

Get up and running with Omnivore in minutes. This guide covers the essential commands and common use cases to help you start crawling and analyzing content immediately.

## Table of Contents
- [Initial Setup](#initial-setup)
- [Basic Web Crawling](#basic-web-crawling)
- [Git Repository Analysis](#git-repository-analysis)
- [AI-Powered Extraction](#ai-powered-extraction)
- [Browser Mode](#browser-mode)
- [Output Formats](#output-formats)
- [Common Workflows](#common-workflows)
- [Tips and Best Practices](#tips-and-best-practices)

## Initial Setup

### Step 1: Install Omnivore
```bash
# Quick install (Linux/macOS)
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh

# Or via Homebrew
brew tap Pranav-Karra-3301/omnivore && brew install omnivore

# Or from source
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore
cargo build --release
make install
```

### Step 2: Run Setup Wizard
```bash
omnivore setup
```

The setup wizard will guide you through:
- **OpenAI API Key**: For AI-powered extraction features
- **Default Settings**: Workers, depth, delays
- **Output Preferences**: Default formats and directories
- **Browser Configuration**: ChromeDriver setup for JavaScript rendering

### Step 3: Verify Installation
```bash
# Check version
omnivore --version

# View available commands
omnivore --help

# Test with a simple crawl
omnivore crawl https://example.com --depth 1
```

## Basic Web Crawling

### Simple Crawl
```bash
# Crawl a website with default settings
omnivore crawl https://example.com

# Output:
# 🕸️  Omnivore Web Crawler
# Starting crawl from: https://example.com
# Configuration:
#   Workers: 10
#   Max depth: 5
#   ...
# ✅ Successfully crawled 42 pages
```

### Crawl with Custom Settings
```bash
# Shallow crawl with more workers
omnivore crawl https://news.ycombinator.com \
  --workers 20 \
  --depth 2 \
  --output hn-crawl.json

# Deep crawl with rate limiting
omnivore crawl https://blog.example.com \
  --depth 10 \
  --delay 500 \
  --respect-robots
```

### Organized Output with Tables
```bash
# Extract tables and organize output
omnivore crawl https://en.wikipedia.org/wiki/List_of_countries_by_population \
  --extract-tables \
  --organize \
  --output population-data

# Creates:
# population-data/
# ├── index.json          # Crawl metadata
# ├── page_0001.json      # Page content
# ├── page_0002.json      # ...
# └── tables/
#     ├── page_0001_Population_by_country.csv
#     └── page_0001_Historical_data.csv
```

### Multiple Output Formats
```bash
# JSON output (default)
omnivore crawl https://example.com --output data.json

# Markdown format
omnivore crawl https://example.com --format markdown --output report.md

# CSV format (best for structured data)
omnivore crawl https://example.com --format csv --output data.csv

# Plain text
omnivore crawl https://example.com --format text --output content.txt

# YAML format
omnivore crawl https://example.com --format yaml --output data.yaml
```

## Git Repository Analysis

### Basic Repository Analysis
```bash
# Analyze a GitHub repository
omnivore git https://github.com/rust-lang/rust-clippy --output clippy-analysis.txt

# Output includes:
# - Project type detection (CLI tool, Web app, Library, etc.)
# - Language and framework identification
# - Organized code structure
# - Full source code with line numbers
```

### Filter Specific Files
```bash
# Extract only Rust source files
omnivore git https://github.com/rust-lang/cargo \
  --only "*.rs,*.toml" \
  --output cargo-rust.txt

# Exclude test files
omnivore git ./my-project \
  --exclude "**/tests/**,**/*.test.js" \
  --output src-only.txt

# Include normally excluded directories
omnivore git ./my-project \
  --include "**/node_modules/express/**" \
  --output with-deps.txt
```

### Analyze Local Repository
```bash
# Current directory
omnivore git . --output this-project.txt

# Specific local path
omnivore git ~/projects/my-app --output my-app-analysis.txt

# With JSON output for processing
omnivore git . --json --output codebase.json
```

## AI-Powered Extraction

### Setup AI Features
```bash
# Configure OpenAI API key
omnivore setup
# Enter your OpenAI API key when prompted
```

### Natural Language Queries
```bash
# Extract specific information using AI
omnivore crawl https://shop.example.com \
  --ai "Extract all product names, prices, and availability"

# Extract structured data from news sites
omnivore crawl https://news.site.com \
  --ai "Get article titles, authors, publication dates, and summaries"

# Academic data extraction
omnivore crawl https://university.edu/courses \
  --ai "Extract course codes, titles, credits, prerequisites, and instructor names"
```

### Auto Mode - Intelligent Detection
```bash
# Automatically detect and extract everything
omnivore crawl https://example.com --auto

# Auto mode detects:
# - Tables (exports as CSV)
# - Forms and input fields
# - Dropdowns and filters
# - Pagination
# - Contact information
# - Downloadable files
# - Media content
# - Structured data (JSON-LD, microdata)
```

### Using Templates
```bash
# E-commerce template
omnivore crawl https://shop.com --template ecommerce
# Extracts: products, prices, reviews, categories

# News template
omnivore crawl https://news.com --template news
# Extracts: articles, authors, dates, categories

# Academic template
omnivore crawl https://university.edu --template academic
# Extracts: courses, faculty, departments, research
```

## Browser Mode

### Enable JavaScript Rendering
```bash
# First, ensure ChromeDriver is running
chromedriver --port=9515

# Crawl JavaScript-heavy site
omnivore crawl https://spa-app.com --browser

# With custom wait time for dynamic content
omnivore crawl https://slow-app.com \
  --browser \
  --wait 3000  # Wait 3 seconds for content to load
```

### Interactive Mode
```bash
# Interact with dropdowns and filters
omnivore crawl https://data-portal.com \
  --browser \
  --interact \
  --extract-tables

# This will:
# 1. Load the page with JavaScript
# 2. Click on all dropdowns to reveal options
# 3. Try different filter combinations
# 4. Extract data from each variation
# 5. Export all tables found
```

## Output Formats

### JSON Output (Default)
```json
{
  "stats": {
    "total_urls": 50,
    "successful": 48,
    "failed": 2,
    "elapsed_time": "45.3s"
  },
  "results": [
    {
      "url": "https://example.com",
      "title": "Example Domain",
      "content": "...",
      "links": ["..."],
      "metadata": {}
    }
  ]
}
```

### Markdown Output
```markdown
# Crawl Results: https://docs.example.com

**Date:** 2024-01-15
**Pages:** 25 | **Words:** 15,420

## Page 1: Getting Started
**URL:** https://docs.example.com/getting-started
...
```

### Organized Directory Structure
```
crawl-output/
├── index.json              # Metadata and statistics
├── page_0001.json          # Individual page data
├── page_0002.json
└── tables/                 # Extracted tables as CSV
    ├── table_001.csv
    └── table_002.csv
```

## Common Workflows

### Website Backup
```bash
# Complete website backup with media
omnivore crawl https://my-site.com \
  --depth 20 \
  --organize \
  --include-raw \
  --zip \
  --output backup-$(date +%Y%m%d).zip
```

### Data Mining
```bash
# Extract all tables from a data portal
omnivore crawl https://data.gov/statistics \
  --extract-tables \
  --format csv \
  --organize \
  --output gov-data/

# Mine product data from e-commerce
omnivore crawl https://shop.com/products \
  --ai "Extract product name, price, SKU, stock status" \
  --format json \
  --output products.json
```

### Documentation Scraping
```bash
# Scrape technical documentation
omnivore crawl https://docs.framework.com \
  --depth 10 \
  --format markdown \
  --exclude-urls "*/api/*" \
  --output framework-docs.md
```

### Competitive Analysis
```bash
# Analyze competitor websites
for site in competitor1.com competitor2.com competitor3.com; do
  omnivore crawl https://$site \
    --depth 3 \
    --ai "Extract products, pricing, features, testimonials" \
    --output "analysis-$site.json"
done
```

## Tips and Best Practices

### Performance Optimization
```bash
# For large sites, use shallow depth first
omnivore crawl https://huge-site.com --depth 2 --output preview.json

# Use more workers for faster crawling (if site allows)
omnivore crawl https://fast-site.com --workers 50

# But be polite with rate limiting
omnivore crawl https://small-blog.com --workers 2 --delay 1000
```

### Respectful Crawling
```bash
# Always respect robots.txt
omnivore crawl https://example.com --respect-robots

# Use appropriate delays
omnivore crawl https://example.com --delay 500

# Set a custom user agent
omnivore crawl https://example.com \
  --user-agent "MyBot/1.0 (contact@example.com)"
```

### Error Handling
```bash
# Verbose mode for debugging
omnivore crawl https://problem-site.com \
  --verbose \
  --output debug.json

# Save error log
omnivore crawl https://example.com \
  --output data.json \
  2> errors.log
```

## Next Steps

Now that you're familiar with the basics:

1. **Explore Advanced Features**:
   - Read about [Browser Mode](cli.md#browser-mode) for JavaScript sites
   - Learn about [AI Integration](cli.md#ai-features) for intelligent extraction
   - Check [Git Command](cli-git.md) for code repository analysis

2. **Customize Your Setup**:
   - Configure defaults in [Configuration](configuration.md)
   - Set up API keys for AI features
   - Create custom templates

3. **Scale Your Operations**:
   - Use the [REST API](api/rest.md) for programmatic access
   - Deploy with [Docker](installation.md#docker-installation)
   - Set up monitoring and logging

4. **Get Help**:
   - Run `omnivore help <command>` for detailed command help
   - Check [Troubleshooting](troubleshooting.md) for common issues
   - Visit the [GitHub repository](https://github.com/Pranav-Karra-3301/omnivore) for updates