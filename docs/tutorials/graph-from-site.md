# Tutorial: Build a Graph from a Site

1. Crawl a site and save results:
```bash
omnivore crawl https://example.com --depth 5 --output results.json
```
2. Build the graph:
```bash
omnivore graph results.json --output graph.db
```
3. Explore via API or custom tools.
