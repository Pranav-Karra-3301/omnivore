# omnivore-cli

Command-line interface for the Omnivore web crawler.

## Install

```bash
cargo install omnivore-cli
```

## Usage

```bash
# Basic crawl
omnivore crawl https://example.com --workers 5 --depth 3

# Build knowledge graph
omnivore graph results.json --output knowledge-graph.db
```

## License

MIT OR Apache-2.0
