# CLI Reference

The Omnivore CLI provides a command-line interface for web crawling and content extraction.

## Installation

```bash
cargo install omnivore
```

## Global Options

- `-v, --verbose`: Enable verbose logging
- `-c, --config <FILE>`: Path to TOML configuration file
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Commands

### `crawl` - Web Crawling

Start a web crawl from one or more seed URLs.

```bash
omnivore crawl <URL> [OPTIONS]
```

#### Arguments
- `<URL>`: Starting URL(s) to crawl

#### Options
- `--workers <N>`: Number of concurrent workers (default: 10, range: 1-100)
- `--depth <N>`: Maximum crawl depth (default: 5, range: 1-20)
- `--delay <MS>`: Delay between requests in milliseconds (default: 100)
- `--output <FILE>`: Export crawl statistics to JSON file
- `--respect-robots`: Honor robots.txt directives (currently fetches but doesn't parse)
- `--user-agent <STRING>`: Custom User-Agent string

#### Examples

Basic crawl:
```bash
omnivore crawl https://example.com
```

Deep crawl with more workers:
```bash
omnivore crawl https://example.com --depth 10 --workers 20
```

Polite crawl with delays:
```bash
omnivore crawl https://example.com --delay 1000 --respect-robots
```

Export statistics:
```bash
omnivore crawl https://example.com --output stats.json
```

### `parse` - HTML Parsing

Parse HTML content and extract structured data.

```bash
omnivore parse <FILE> [OPTIONS]
```

#### Arguments
- `<FILE>`: Path to HTML file

#### Options
- `--rules <FILE>`: Path to extraction rules file (JSON)
- `--output <FILE>`: Write extracted data to file

#### Examples

Basic parsing:
```bash
omnivore parse page.html
```

With extraction rules:
```bash
omnivore parse page.html --rules rules.json
```

### `git` - Git Repository Analysis

Extract and analyze code from Git repositories with intelligent filtering.

```bash
omnivore git <SOURCE> [OPTIONS]
```

#### Arguments
- `<SOURCE>`: Repository URL or local path

#### Options
- `--output <PATH>`: Output file path (.txt or .json)
- `--only <PATTERNS>`: Include only matching files (comma-separated)
- `--include <PATTERNS>`: Include matching files
- `--exclude <PATTERNS>`: Exclude matching files
- `--keep`: Keep cloned repository after completion
- `--json`: Output in JSON format
- `--stdout`: Output to stdout

#### Examples

Analyze a GitHub repository:
```bash
omnivore git https://github.com/rust-lang/cargo --output cargo-analysis.txt
```

Extract specific file types:
```bash
omnivore git ./my-project --only "*.rs,*.toml"
```

See [Git Command Documentation](cli-git.md) for detailed usage.

### `graph` - Graph Operations (⚠️ Not Implemented)

Build knowledge graphs from crawled data.

```bash
omnivore graph <INPUT> [OPTIONS]
```

**Note**: This command currently only prints placeholder messages. Graph functionality is under development.

### `stats` - Statistics (⚠️ Limited Implementation)

Display crawl session statistics.

```bash
omnivore stats [SESSION]
```

**Note**: This command has limited functionality. Session tracking is not fully implemented.

### `generate-completions` - Shell Completions

Generate shell completion scripts.

```bash
omnivore generate-completions <SHELL>
```

#### Supported Shells
- `bash`
- `zsh`
- `fish`
- `powershell`

#### Installation Examples

Bash:
```bash
omnivore generate-completions bash > /usr/local/etc/bash_completion.d/omnivore
```

Zsh:
```bash
omnivore generate-completions zsh > ~/.zsh/completions/_omnivore
```

Fish:
```bash
omnivore generate-completions fish > ~/.config/fish/completions/omnivore.fish
```

## Configuration File

Use a TOML configuration file to set default values:

```bash
omnivore -c config.toml crawl https://example.com
```

See [Configuration](configuration.md) for file format details.

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `3`: Configuration error
- `4`: Network error

## Environment Variables

- `OMNIVORE_CONFIG`: Default configuration file path
- `OMNIVORE_DATA_DIR`: Data storage directory (default: `~/.omnivore`)
- `RUST_LOG`: Logging level (trace, debug, info, warn, error)

## Common Use Cases

### 1. Quick Site Snapshot
```bash
omnivore crawl https://example.com --depth 2 --output snapshot.json
```

### 2. Respectful Crawling
```bash
omnivore crawl https://example.com --workers 2 --delay 2000 --respect-robots
```

### 3. Deep Site Analysis
```bash
omnivore crawl https://example.com --depth 10 --workers 20 --output analysis.json
```

### 4. Parse Downloaded Content
```bash
# First download
curl https://example.com > page.html
# Then parse
omnivore parse page.html
```

## Troubleshooting

### Issue: Crawl seems stuck
- Reduce worker count: `--workers 2`
- Increase delay: `--delay 1000`
- Check network connectivity

### Issue: Too many requests error
- Increase delay between requests
- Reduce worker count
- Check if site has rate limiting

### Issue: Memory usage high
- Reduce worker count
- Decrease crawl depth
- Use configuration file to limit queue size

## Performance Tips

1. **Worker Count**: Start with 5-10 workers and adjust based on site response
2. **Delays**: Use at least 100ms delay for polite crawling
3. **Depth**: Keep depth low (2-3) for initial exploration
4. **Output**: Use `--output` to save results for later analysis

## Limitations

- Robots.txt parsing not fully implemented (fetches but doesn't parse rules)
- No JavaScript rendering (static HTML only)
- Session management not persistent across runs
- Graph and stats commands have limited functionality