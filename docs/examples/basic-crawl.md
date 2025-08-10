# Example: Basic Web Crawling

This guide demonstrates various crawling scenarios with Omnivore, from simple to complex use cases.

## Basic Crawl

The simplest way to crawl a website:

```bash
omnivore crawl https://example.com
```

This uses default settings:
- 10 concurrent workers
- Crawl depth of 5
- 100ms delay between requests

## Controlled Crawling

### Shallow Crawl (Homepage Only)

```bash
omnivore crawl https://example.com --depth 0
```

This only fetches the homepage without following any links.

### Limited Depth Crawl

```bash
omnivore crawl https://example.com --depth 2 --workers 5
```

- Crawls only 2 levels deep from the starting page
- Uses 5 concurrent workers for moderate speed

### Polite Crawling

```bash
omnivore crawl https://example.com \
  --workers 2 \
  --delay 1000 \
  --respect-robots \
  --user-agent "MyBot/1.0 (https://mysite.com/bot)"
```

Best practices for production sites:
- Only 2 workers to minimize server load
- 1 second delay between requests
- Respects robots.txt (note: currently only fetches, doesn't parse)
- Custom user agent with contact info

## Advanced Scenarios

### Deep Site Analysis

```bash
omnivore crawl https://example.com \
  --depth 10 \
  --workers 20 \
  --output analysis.json \
  --verbose
```

For thorough site exploration:
- Deep crawl (10 levels)
- Many workers for speed
- Save statistics to file
- Verbose logging for debugging

### Multiple Seed URLs

```bash
# Create a config file with multiple seeds
cat > crawl-config.toml << EOF
[crawler]
seed_urls = [
    "https://example.com",
    "https://example.org",
    "https://example.net"
]
max_workers = 15
max_depth = 3
EOF

omnivore -c crawl-config.toml crawl
```

### Export and Analysis Pipeline

```bash
# 1. Crawl and export stats
omnivore crawl https://example.com \
  --depth 3 \
  --output crawl-stats.json

# 2. Parse the statistics
cat crawl-stats.json | jq '.urls_visited'

# 3. Extract specific metrics
cat crawl-stats.json | jq '{
  total: .urls_visited,
  success_rate: (.success_count / .urls_visited * 100),
  avg_time: .average_response_time_ms
}'
```

## Edge Cases and Troubleshooting

### Handling Large Sites

For sites with thousands of pages:

```bash
omnivore crawl https://large-site.com \
  --workers 5 \
  --depth 3 \
  --delay 500
```

Tips:
- Keep depth low to avoid exponential growth
- Use moderate worker count
- Add delays to prevent overwhelming the server

### Sites with Rate Limiting

```bash
omnivore crawl https://api.example.com \
  --workers 1 \
  --delay 2000
```

Single worker with long delays prevents hitting rate limits.

### JavaScript-Heavy Sites

⚠️ **Limitation**: Omnivore currently only parses static HTML.

For JavaScript sites, you'll see limited content:

```bash
omnivore crawl https://spa-site.com --depth 1
```

The crawler will only see the initial HTML, not dynamically loaded content.

### Handling Redirects

Omnivore automatically follows HTTP redirects:

```bash
omnivore crawl http://example.com  # Redirects to https://
```

### Sites with Authentication

⚠️ **Not Supported**: Omnivore cannot currently handle login-protected areas.

```bash
# This will only crawl public pages
omnivore crawl https://site-with-login.com
```

### Handling Errors

Common issues and solutions:

#### Connection Refused
```bash
# Site may be blocking crawlers
omnivore crawl https://blocking-site.com \
  --user-agent "Mozilla/5.0 (compatible; MyBot/1.0)" \
  --delay 3000
```

#### Timeout Errors
```bash
# Increase timeout in config
cat > config.toml << EOF
[crawler.politeness]
timeout_secs = 60
EOF

omnivore -c config.toml crawl https://slow-site.com
```

#### Memory Issues
```bash
# Reduce concurrency and depth
omnivore crawl https://huge-site.com \
  --workers 2 \
  --depth 2
```

## Monitoring Progress

### Real-time Progress

During crawling, you'll see:

```
Crawling https://example.com...
[00:00:30] ████████████████████ 150/200 URLs | ✓ 145 ✗ 5 | 5.0 pages/s
```

Components:
- Time elapsed: `[00:00:30]`
- Progress bar showing completion
- URL count: `150/200` (visited/queued)
- Success/error counts: `✓ 145 ✗ 5`
- Speed: `5.0 pages/s`

### Final Statistics

After completion:

```json
{
  "urls_visited": 150,
  "urls_queued": 0,
  "success_count": 145,
  "error_count": 5,
  "start_time": "2024-01-15T10:00:00Z",
  "end_time": "2024-01-15T10:05:30Z",
  "duration_secs": 330,
  "pages_per_second": 0.45,
  "average_response_time_ms": 250,
  "total_bytes_downloaded": 15728640
}
```

## Performance Tuning

### Finding Optimal Settings

Start conservative and increase:

```bash
# Test 1: Conservative
omnivore crawl https://target.com --workers 2 --delay 500

# Test 2: Moderate
omnivore crawl https://target.com --workers 5 --delay 200

# Test 3: Aggressive (if server handles it)
omnivore crawl https://target.com --workers 10 --delay 100
```

### Benchmarking

Compare different configurations:

```bash
# Run multiple tests
for workers in 2 5 10; do
  echo "Testing with $workers workers..."
  omnivore crawl https://example.com \
    --workers $workers \
    --depth 2 \
    --output "test-$workers.json"
done

# Compare results
for f in test-*.json; do
  echo "$f: $(jq .pages_per_second $f) pages/sec"
done
```

## Integration Examples

### Bash Script Integration

```bash
#!/bin/bash
# crawl-monitor.sh

URL="https://example.com"
OUTPUT="crawl-$(date +%Y%m%d-%H%M%S).json"

# Run crawl
omnivore crawl "$URL" --depth 3 --output "$OUTPUT"

# Check results
if [ $? -eq 0 ]; then
  PAGES=$(jq .urls_visited "$OUTPUT")
  echo "Successfully crawled $PAGES pages"
  
  # Alert if errors exceed threshold
  ERRORS=$(jq .error_count "$OUTPUT")
  if [ "$ERRORS" -gt 10 ]; then
    echo "Warning: High error count ($ERRORS)"
  fi
else
  echo "Crawl failed"
  exit 1
fi
```

### Python Integration

```python
import subprocess
import json
import time

def crawl_site(url, depth=3, workers=5):
    """Run Omnivore crawler and return statistics"""
    
    output_file = f"crawl-{int(time.time())}.json"
    
    cmd = [
        "omnivore", "crawl", url,
        "--depth", str(depth),
        "--workers", str(workers),
        "--output", output_file
    ]
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode == 0:
        with open(output_file) as f:
            stats = json.load(f)
        return stats
    else:
        raise Exception(f"Crawl failed: {result.stderr}")

# Usage
stats = crawl_site("https://example.com")
print(f"Crawled {stats['urls_visited']} pages")
print(f"Success rate: {stats['success_count']/stats['urls_visited']*100:.1f}%")
```

## Best Practices

### 1. Start Small
Always test with shallow depth first:
```bash
omnivore crawl https://new-site.com --depth 1
```

### 2. Respect Server Resources
Use delays and limited workers:
```bash
omnivore crawl https://small-site.com --workers 3 --delay 500
```

### 3. Monitor Resource Usage
Watch memory and CPU:
```bash
# In another terminal
watch -n 1 'ps aux | grep omnivore'
```

### 4. Save Statistics
Always export stats for analysis:
```bash
omnivore crawl https://example.com --output "$(date +%Y%m%d)-crawl.json"
```

### 5. Use Configuration Files
For repeated crawls, use config files:
```toml
# production-crawl.toml
[crawler]
max_workers = 5
max_depth = 4
user_agent = "ProductionBot/1.0"
respect_robots_txt = true

[crawler.politeness]
default_delay_ms = 1000
max_retries = 2
timeout_secs = 30
```

```bash
omnivore -c production-crawl.toml crawl https://example.com
```