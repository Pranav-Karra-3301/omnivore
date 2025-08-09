# CLI Reference

The CLI is powered by `clap` and exposes several subcommands.

## Global options
- `-v, --verbose`: verbose logging
- `-c, --config <FILE>`: path to configuration file

## `crawl`
Crawl starting from a URL.

```bash
omnivore crawl <url> [--workers N] [--depth N] [--output FILE] [--respect-robots] [--delay MS]
```
- **url**: starting URL
- **--workers**: number of concurrent workers (default 10)
- **--depth**: maximum crawl depth (default 5)
- **--output**: write final stats to a file (JSON)
- **--respect-robots**: honor robots.txt
- **--delay**: delay between requests in ms (default 100)

## `parse`
Parse an HTML file with optional rules.

```bash
omnivore parse <file> [--rules FILE]
```

## `graph`
Build a knowledge graph from crawl results.

```bash
omnivore graph <input> [--output FILE]
```

## `stats`
Display crawl statistics for a session.

```bash
omnivore stats [session]
```

## `generate-completions`
Generate shell completions.

```bash
omnivore generate-completions <bash|zsh|fish|powershell>
```
