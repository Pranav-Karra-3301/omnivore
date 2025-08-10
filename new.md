# Omnivore v0.4.0 - New Features Documentation

## Overview
Omnivore has been transformed into the most comprehensive universal web scraper and code extractor, with professional-grade features for handling any website or repository.

## New Features

### 1. Fixed Worker Parallelization
- **Issue**: Previous scheduler wasn't enforcing max_workers limit
- **Solution**: Implemented proper semaphore-based concurrency control
- **Impact**: Crawler now respects the `--workers` flag and properly limits concurrent requests

### 2. Smart Redirect Handling
- **Feature**: Automatic handling of HTTP redirects (301/302/307/308)
- **Configuration**: Follows up to 10 redirects by default
- **Logging**: All redirects are logged to `warnings.log` for debugging
- **Example**:
  ```bash
  omnivore crawl https://example.com --workers 5
  # Automatically handles redirects from http to https, www to non-www, etc.
  ```

### 3. Organized Folder Output (--organize)
- **Feature**: Creates structured folder hierarchy for crawled content
- **Structure**: 
  ```
  domain_com_20241210_150000_crawl/
  ├── index.json       # Crawl metadata and summary
  ├── page_0001.json   # Individual page content
  ├── page_0002.json
  └── ...
  ```
- **Usage**:
  ```bash
  omnivore crawl https://example.com --organize
  ```

### 4. Multiple Output Formats (--format)
- **Supported Formats**:
  - `json` (default): Structured JSON output
  - `markdown`: Clean, readable Markdown format
  - `csv`: Tabular format with url, title, text, word_count, links
  - `yaml`: Human-readable YAML structure
  - `text`: Plain text with minimal formatting

- **Examples**:
  ```bash
  # Output as Markdown
  omnivore crawl https://example.com --format markdown
  
  # Output as CSV for data analysis
  omnivore crawl https://example.com --format csv
  
  # Output as YAML for configuration files
  omnivore crawl https://example.com --format yaml
  ```

### 5. ZIP Compression (--zip)
- **Feature**: Automatically compress output to ZIP file
- **Benefits**: Reduces storage space by 60-80% for text content
- **Works with**: Both single files and organized folders
- **Usage**:
  ```bash
  # Compress single output file
  omnivore crawl https://example.com --zip
  
  # Compress organized folder structure
  omnivore crawl https://example.com --organize --zip
  ```

### 6. Comprehensive Error Logging
- **Files Created**:
  - `error.log`: All crawl failures with timestamps
  - `warnings.log`: Non-fatal issues like redirects
- **Format**: `[ISO-8601 Timestamp] Error/Warning Message`
- **Automatic**: No configuration needed, logs are created when errors occur

### 7. URL Exclusion (--exclude-urls)
- **Feature**: Remove all URLs/links from output content
- **Use Case**: When you only need text content without references
- **Example**:
  ```bash
  omnivore crawl https://example.com --exclude-urls
  ```

### 8. Content Filtering
- **Smart Extraction**: Automatically filters out:
  - Navigation menus
  - Headers/footers
  - Scripts and styles
  - Boilerplate content
- **Structured Detection**: Recognizes and preserves:
  - Course listings
  - FAQs
  - Lists and sections
  - Important metadata

## Usage Examples

### Basic Crawl with Clean Output
```bash
omnivore crawl https://example.com
```

### Professional Data Collection
```bash
omnivore crawl https://university.edu \
  --workers 10 \
  --depth 3 \
  --organize \
  --format markdown \
  --zip
```

### Data Analysis Pipeline
```bash
# Crawl and output as CSV
omnivore crawl https://data-source.com --format csv --output data.csv

# Process with standard tools
cat data.csv | grep "important" | cut -d',' -f1,3
```

### Monitoring and Debugging
```bash
# Crawl with full logging
omnivore crawl https://complex-site.com --workers 20

# Check errors
tail -f error.log

# Monitor redirects
tail -f warnings.log
```

## Performance Improvements

1. **Parallel Processing**: Proper worker pool management
2. **Connection Reuse**: HTTP keep-alive for better performance
3. **Smart Retries**: Exponential backoff for failed requests
4. **Memory Efficient**: Streaming processing for large sites

### 9. Enhanced User Experience
- **Purple ASCII Art**: Beautiful purple branding when running `omnivore` or `omnivore --help`
- **Version Display**: Prominent version information below the banner
- **Documentation Command**: `omnivore docs` opens documentation at ov.pranavkarra.me/docs

### 10. Improved Git Command
- **Default File Output**: Automatically saves to `{repo_name}_{timestamp}.txt`
- **Smart Filename Generation**: Extracts repository name from URL
- **Stdout Option**: Use `--stdout` flag to output to terminal
- **Automatic Cleanup**: Temporary clones are deleted unless `--keep` is specified

### 11. Advanced Table Extraction (--extract-tables)
- **Feature**: Automatically detect and extract HTML tables from web pages
- **Output**: Saves tables as CSV files in `tables/` subdirectory
- **Smart Detection**: 
  - Identifies table headers automatically
  - Handles complex tables with merged cells
  - Extracts table titles and captions
  - Preserves footnotes and annotations
- **Usage**:
  ```bash
  # Extract tables from academic pages
  omnivore crawl https://gradschool.psu.edu/program-metrics \
    --organize \
    --extract-tables
  ```
- **Output Structure**:
  ```
  domain_com_timestamp_crawl/
  ├── index.json       # Includes table metadata
  ├── page_0001.json   # Page content with table data
  ├── tables/
  │   ├── page_0001_admissions_data.csv
  │   ├── page_0001_gre_scores.csv
  │   └── page_0001_demographics.csv
  └── ...
  ```

## Coming Soon (Planned Features)

### Content Deduplication (--dedupe)
- Hash-based duplicate detection
- Store unique content only
- Reference duplicates in index

### Resume Capability (--resume)
- Save crawl state to `.omnivore-state.json`
- Resume interrupted crawls
- Incremental crawling support

## Best Practices

1. **Start Small**: Test with `--depth 1` first
2. **Use Workers Wisely**: 5-10 workers for most sites
3. **Respect Robots.txt**: Use `--respect-robots` flag
4. **Monitor Logs**: Check error.log for issues
5. **Organize Large Crawls**: Use `--organize` for sites with >100 pages

## Troubleshooting

### Common Issues

1. **Too Many Redirects**
   - Check `warnings.log` for redirect chains
   - Site might be blocking crawlers

2. **High Memory Usage**
   - Reduce workers with `--workers 3`
   - Use `--organize` to write pages incrementally

3. **Slow Performance**
   - Increase workers for parallel processing
   - Check network connectivity
   - Reduce depth for faster results

## CLI Help
```bash
# View all options
omnivore crawl --help

# View examples
omnivore --help
```

## Version History
- v0.4.0: Enhanced UX with ASCII art, docs command, improved git defaults
- v0.3.0: Added advanced table extraction and CSV export capabilities
- v0.2.0: Major feature update with professional crawling capabilities
- v0.1.1: Initial release with basic crawling

---
Built with Rust for performance, reliability, and modern web crawling needs.