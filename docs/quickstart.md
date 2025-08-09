# Quickstart

## CLI
```bash
# Crawl a website
omnivore crawl https://example.com --workers 10 --depth 5 --respect-robots --delay 100

# Parse an HTML file with optional rules
omnivore parse ./index.html --rules ./parser-rules.yaml

# Build a knowledge graph from crawl results
omnivore graph ./crawl-results.json --output ./graph.db

# Show crawl stats
omnivore stats
```

## API
```bash
# Start API server
cargo run --bin omnivore-api

# REST: start a crawl
curl -X POST http://localhost:3000/api/crawl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com", "max_depth": 5, "max_workers": 10}'

# REST: get stats
curl http://localhost:3000/api/stats
```
