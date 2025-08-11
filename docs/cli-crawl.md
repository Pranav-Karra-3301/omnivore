# Omnivore Crawl Command

The `omnivore crawl` command is the core web crawling functionality, offering powerful features for extracting content from websites with support for AI-powered extraction, browser rendering, and multiple output formats.

## Table of Contents
- [Basic Usage](#basic-usage)
- [Command Options](#command-options)
- [AI-Powered Features](#ai-powered-features)
- [Browser Mode](#browser-mode)
- [Output Formats](#output-formats)
- [Advanced Features](#advanced-features)
- [Examples](#examples)
- [Performance Tuning](#performance-tuning)
- [Best Practices](#best-practices)

## Basic Usage

```bash
omnivore crawl <URL> [OPTIONS]
```

### Simple Examples
```bash
# Basic crawl with defaults
omnivore crawl https://example.com

# Save results to file
omnivore crawl https://example.com --output results.json

# Limit depth and workers
omnivore crawl https://example.com --depth 3 --workers 5
```

## Command Options

### Core Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--workers` | `-w` | 10 | Number of concurrent workers (1-100) |
| `--depth` | `-d` | 5 | Maximum crawl depth (1-20) |
| `--output` | `-o` | - | Output file path |
| `--format` | - | json | Output format (json, markdown, csv, yaml, text) |
| `--verbose` | `-v` | false | Enable verbose logging |

### Crawling Control

| Option | Default | Description |
|--------|---------|-------------|
| `--delay` | 100 | Delay between requests in milliseconds |
| `--respect-robots` | false | Honor robots.txt directives |
| `--user-agent` | Omnivore/X.X | Custom User-Agent string |
| `--timeout` | 30000 | Request timeout in milliseconds |
| `--max-retries` | 3 | Maximum retry attempts for failed requests |
| `--follow-redirects` | true | Follow HTTP redirects |
| `--max-redirects` | 10 | Maximum number of redirects to follow |

### Content Filtering

| Option | Description |
|--------|-------------|
| `--include-urls` | URL patterns to include (comma-separated) |
| `--exclude-urls` | URL patterns to exclude (comma-separated) |
| `--content-types` | Content types to accept (default: text/html) |
| `--max-page-size` | Maximum page size in bytes |
| `--min-content-length` | Minimum content length to save |

### Output Options

| Option | Description |
|--------|-------------|
| `--organize` | Create organized folder structure |
| `--extract-tables` | Extract HTML tables as CSV files |
| `--include-raw` | Include raw HTML in output |
| `--exclude-urls` | Don't include URLs/links in output |
| `--zip` | Compress output to ZIP file |
| `--stdout` | Output to stdout instead of file |

## AI-Powered Features

### Natural Language Extraction

Use the `--ai` flag with a natural language query to extract specific information:

```bash
# Extract product information
omnivore crawl https://shop.com \
  --ai "Extract product names, prices, descriptions, and availability"

# Extract article metadata
omnivore crawl https://news.site.com \
  --ai "Get article titles, authors, publication dates, and summaries"

# Extract contact information
omnivore crawl https://company.com \
  --ai "Find all email addresses, phone numbers, and physical addresses"
```

### Auto Mode

The `--auto` flag enables intelligent automatic extraction:

```bash
omnivore crawl https://example.com --auto
```

Auto mode automatically detects and extracts:
- **Tables**: Converted to CSV files
- **Forms**: Input fields and their attributes
- **Dropdowns**: All options and values
- **Pagination**: Next/previous links
- **Contact Info**: Emails, phones, addresses
- **Downloads**: Links to downloadable files
- **Media**: Images and videos with metadata
- **Structured Data**: JSON-LD, Microdata, OpenGraph

### Templates

Use pre-configured templates for common website types:

```bash
# E-commerce sites
omnivore crawl https://shop.com --template ecommerce

# News and blogs
omnivore crawl https://news.com --template news

# Academic sites
omnivore crawl https://university.edu --template academic

# Real estate
omnivore crawl https://realestate.com --template realestate

# Job boards
omnivore crawl https://jobs.com --template jobs

# Social media
omnivore crawl https://social.com --template social
```

#### Available Templates

| Template | Extracts |
|----------|----------|
| `ecommerce` | Products, prices, reviews, categories, stock status |
| `news` | Articles, authors, dates, categories, tags |
| `academic` | Courses, faculty, departments, research papers |
| `realestate` | Listings, prices, features, locations, agents |
| `jobs` | Job titles, companies, salaries, requirements |
| `social` | Posts, users, comments, likes, shares |
| `forum` | Threads, posts, users, timestamps |
| `documentation` | Sections, code examples, API references |

## Browser Mode

Enable JavaScript rendering for dynamic websites:

### Basic Browser Mode
```bash
# Ensure ChromeDriver is running
chromedriver --port=9515

# Crawl with browser
omnivore crawl https://spa-app.com --browser
```

### Browser Options

| Option | Description |
|--------|-------------|
| `--browser` | Enable browser mode with JavaScript rendering |
| `--interact` | Interact with dropdowns and dynamic elements |
| `--wait` | Wait time in ms for content to load (default: 2000) |
| `--screenshot` | Take screenshots of each page |
| `--scroll` | Scroll to load lazy content |
| `--max-scroll` | Maximum scroll attempts for infinite scroll |

### Interactive Mode

```bash
# Interact with all dynamic elements
omnivore crawl https://data-portal.com \
  --browser \
  --interact \
  --extract-tables

# Handle infinite scroll
omnivore crawl https://feed.com \
  --browser \
  --scroll \
  --max-scroll 20
```

## Output Formats

### JSON (Default)
```bash
omnivore crawl https://example.com --format json --output data.json
```

Output structure:
```json
{
  "crawler": "omnivore",
  "version": "0.4.0",
  "start_url": "https://example.com",
  "timestamp": "2024-01-15T10:30:00Z",
  "stats": {
    "total_urls": 100,
    "successful": 95,
    "failed": 5,
    "elapsed_time": "120.5s"
  },
  "results": [
    {
      "url": "https://example.com",
      "status_code": 200,
      "title": "Example Domain",
      "content": "...",
      "cleaned_content": {
        "title": "Example Domain",
        "content": "Main text content...",
        "word_count": 500,
        "links": ["..."],
        "tables": []
      },
      "metadata": {
        "description": "...",
        "keywords": ["..."],
        "og:title": "..."
      },
      "extracted_data": {}
    }
  ]
}
```

### Markdown
```bash
omnivore crawl https://docs.site.com --format markdown --output docs.md
```

### CSV
```bash
omnivore crawl https://data.site.com --format csv --output data.csv
```

CSV columns:
- url
- title
- content
- word_count
- links (comma-separated)
- status_code
- crawled_at

### YAML
```bash
omnivore crawl https://example.com --format yaml --output data.yaml
```

### Plain Text
```bash
omnivore crawl https://example.com --format text --output content.txt
```

### Organized Output
```bash
omnivore crawl https://example.com --organize --output site-backup/
```

Creates structure:
```
site-backup/
├── index.json                 # Crawl metadata and statistics
├── page_0001.json             # First page content
├── page_0002.json             # Second page content
├── ...
├── tables/                    # Extracted tables (if --extract-tables)
│   ├── page_0001_table_1.csv
│   ├── page_0001_table_2.csv
│   └── ...
├── media/                     # Downloaded media (if --download-media)
│   ├── images/
│   └── videos/
└── screenshots/               # Screenshots (if --screenshot in browser mode)
    ├── page_0001.png
    └── ...
```

## Advanced Features

### Table Extraction
```bash
# Extract all tables as CSV files
omnivore crawl https://data.gov/statistics \
  --extract-tables \
  --organize \
  --output gov-data/
```

Tables are saved as:
- Individual CSV files in `tables/` directory
- Embedded in JSON output with structure
- Formatted in Markdown output

### URL Filtering
```bash
# Include only specific paths
omnivore crawl https://example.com \
  --include-urls "/blog/*,/news/*"

# Exclude admin and private areas
omnivore crawl https://example.com \
  --exclude-urls "/admin/*,/private/*,*.pdf"

# Combine include and exclude
omnivore crawl https://example.com \
  --include-urls "/products/*" \
  --exclude-urls "*/reviews/*"
```

### Content Type Filtering
```bash
# Only HTML pages
omnivore crawl https://example.com \
  --content-types "text/html"

# HTML and JSON
omnivore crawl https://api.example.com \
  --content-types "text/html,application/json"

# Everything except images
omnivore crawl https://example.com \
  --exclude-content-types "image/*"
```

### Custom Headers
```bash
# Add authentication
omnivore crawl https://api.example.com \
  --header "Authorization: Bearer TOKEN" \
  --header "X-API-Key: KEY"

# Custom cookies
omnivore crawl https://example.com \
  --header "Cookie: session=abc123; user=john"
```

### Session Management
```bash
# Save session for resuming
omnivore crawl https://large-site.com \
  --session my-crawl \
  --output partial.json

# Resume interrupted crawl
omnivore crawl --resume my-crawl \
  --output complete.json

# List saved sessions
omnivore sessions list

# Delete session
omnivore sessions delete my-crawl
```

## Examples

### Complete Website Backup
```bash
omnivore crawl https://my-site.com \
  --depth 20 \
  --workers 20 \
  --organize \
  --extract-tables \
  --include-raw \
  --zip \
  --output "backup-$(date +%Y%m%d).zip"
```

### E-commerce Product Extraction
```bash
omnivore crawl https://shop.com/products \
  --ai "Extract product name, price, SKU, description, images, stock" \
  --format json \
  --include-urls "*/product/*" \
  --exclude-urls "*/reviews/*" \
  --output products.json
```

### News Article Collection
```bash
omnivore crawl https://news.site.com \
  --template news \
  --depth 5 \
  --format markdown \
  --organize \
  --output news-archive/
```

### Academic Research
```bash
omnivore crawl https://journal.edu \
  --ai "Extract paper titles, authors, abstracts, DOIs, citations" \
  --extract-tables \
  --include-urls "*/papers/*,*/articles/*" \
  --output research.json
```

### API Documentation Scraping
```bash
omnivore crawl https://api.service.com/docs \
  --ai "Extract endpoints, methods, parameters, examples, responses" \
  --format markdown \
  --depth 10 \
  --output api-docs.md
```

### Multi-site Comparison
```bash
#!/bin/bash
sites=("competitor1.com" "competitor2.com" "competitor3.com")

for site in "${sites[@]}"; do
  omnivore crawl "https://$site" \
    --depth 3 \
    --ai "Extract products, pricing, features" \
    --format json \
    --output "analysis-$site.json" &
done
wait

# Combine results
jq -s '.' analysis-*.json > combined-analysis.json
```

### Dynamic Site with Infinite Scroll
```bash
omnivore crawl https://social-feed.com \
  --browser \
  --interact \
  --scroll \
  --max-scroll 50 \
  --wait 3000 \
  --output feed-content.json
```

### Monitoring Price Changes
```bash
# Initial crawl
omnivore crawl https://shop.com/sale \
  --ai "Extract product names and prices" \
  --output prices-baseline.json

# Later crawl
omnivore crawl https://shop.com/sale \
  --ai "Extract product names and prices" \
  --output prices-current.json

# Compare
diff <(jq '.results[].extracted_data' prices-baseline.json) \
     <(jq '.results[].extracted_data' prices-current.json)
```

## Performance Tuning

### For Large Sites
```bash
# Start with shallow crawl
omnivore crawl https://huge-site.com \
  --depth 2 \
  --workers 5 \
  --output preview.json

# Then deep crawl specific sections
omnivore crawl https://huge-site.com/important-section \
  --depth 10 \
  --workers 20
```

### For Slow Sites
```bash
omnivore crawl https://slow-site.com \
  --workers 2 \
  --delay 2000 \
  --timeout 60000 \
  --max-retries 5
```

### For Fast Sites
```bash
omnivore crawl https://fast-site.com \
  --workers 50 \
  --delay 50 \
  --timeout 10000
```

### Memory Management
```bash
# Limit memory usage
omnivore crawl https://example.com \
  --max-queue-size 1000 \
  --max-page-size 10485760 \
  --stream-mode  # Don't store in memory
```

## Best Practices

### 1. Always Be Respectful
```bash
omnivore crawl https://example.com \
  --respect-robots \
  --delay 500 \
  --workers 5 \
  --user-agent "MyBot/1.0 (contact@me.com)"
```

### 2. Start Small
- Begin with `--depth 1` or `--depth 2`
- Use fewer workers initially
- Test on a small section first

### 3. Use Appropriate Delays
- Small sites: 500-1000ms delay
- Medium sites: 200-500ms delay
- Large sites: 50-200ms delay (if allowed)

### 4. Handle Errors Gracefully
```bash
omnivore crawl https://unstable-site.com \
  --max-retries 5 \
  --timeout 30000 \
  --continue-on-error \
  --verbose 2> errors.log
```

### 5. Optimize Output
- Use `--organize` for large crawls
- Use `--zip` to save space
- Use appropriate format for your use case
- Exclude unnecessary data with flags

### 6. Monitor Resource Usage
```bash
# Monitor with system tools
omnivore crawl https://example.com &
PID=$!
top -p $PID
```

## Troubleshooting

### Common Issues

**429 Too Many Requests**
- Increase delay: `--delay 1000`
- Reduce workers: `--workers 2`
- Add exponential backoff

**JavaScript Content Not Loading**
- Use browser mode: `--browser`
- Increase wait time: `--wait 5000`
- Enable interaction: `--interact`

**Memory Issues**
- Reduce workers
- Limit queue size
- Use streaming mode
- Process in batches

**Incomplete Crawls**
- Check robots.txt compliance
- Verify URL patterns
- Check for session requirements
- Look for rate limiting

## See Also

- [Git Command](cli-git.md) - Analyze code repositories
- [Configuration](configuration.md) - Set defaults
- [API Reference](api/rest.md) - Programmatic access
- [Browser Mode Guide](guides/browser.md) - Advanced browser usage