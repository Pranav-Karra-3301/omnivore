# Configuration

Omnivore reads crawler configuration values corresponding to `CrawlConfig` and `PolitenessConfig`.

## Example `crawler.toml`
```toml
[crawler]
max_workers = 100
max_depth = 10
user_agent = "Omnivore/1.0"
respect_robots_txt = true

[crawler.politeness]
default_delay_ms = 100
max_requests_per_second = 10.0
backoff_multiplier = 2.0
```

## Fields
- **max_workers**: maximum concurrent workers
- **max_depth**: maximum traversal depth
- **user_agent**: HTTP user agent string
- **respect_robots_txt**: enable robots.txt compliance
- **politeness.default_delay_ms**: base delay between requests
- **politeness.max_requests_per_second**: throttle ceiling
- **politeness.backoff_multiplier**: exponential backoff factor
- **timeout_ms**: request timeout (ms)
- **max_retries**: number of retries
