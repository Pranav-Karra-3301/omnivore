# omnivore-cli

🕷️ **Omnivore** - The Universal Web Scraper & Code Extractor

[![Crates.io](https://img.shields.io/crates/v/omnivore-cli.svg)](https://crates.io/crates/omnivore-cli)
[![Documentation](https://docs.rs/omnivore-cli/badge.svg)](https://docs.rs/omnivore-cli)
[![License](https://img.shields.io/crates/l/omnivore-cli.svg)](https://github.com/Pranav-Karra-3301/omnivore/blob/master/LICENSE)

A powerful command-line tool for web scraping, code analysis, and knowledge extraction. Omnivore can crawl websites, extract code from Git repositories, and build comprehensive knowledge graphs from the data it collects.

📚 **Full Documentation**: [ov.pranavkarra.me/docs](https://ov.pranavkarra.me/docs)  
🌐 **Website**: [ov.pranavkarra.me](https://ov.pranavkarra.me)

## Features

- 🌐 **Advanced Web Crawling**: Multi-threaded crawling with configurable depth, politeness delays, and smart content extraction
- 📊 **Knowledge Graph Generation**: Automatically build knowledge graphs from crawled content
- 🔍 **Code Repository Analysis**: Extract and analyze code from any Git repository or local directory
- 🤖 **AI-Powered Extraction**: Intelligent content extraction using OpenAI GPT models
- 🎯 **Smart Filtering**: Automatically detect project types and apply intelligent file filtering
- 📁 **Multiple Output Formats**: JSON, Markdown, HTML, CSV, and plain text
- 🔧 **Highly Configurable**: Extensive configuration options via CLI flags or config files

## Installation

### From crates.io

```bash
cargo install omnivore-cli
```

### From source

```bash
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore
cargo install --path omnivore-cli
```

## Quick Start

### Web Crawling

```bash
# Basic crawl with 5 workers and depth of 3
omnivore crawl https://example.com --workers 5 --depth 3

# Crawl with AI extraction
omnivore crawl https://example.com --ai-extract --ai-model gpt-4o-mini

# JavaScript-rendered content with browser mode
omnivore crawl https://example.com --browser --wait 2000
```

### Code Repository Analysis

```bash
# Analyze a GitHub repository
omnivore git https://github.com/user/repo --output repo-analysis.txt

# Analyze local directory (even non-git directories)
omnivore git ./my-project --output project-code.txt

# Filter by file types
omnivore git ./my-project --only rs,toml,md --output rust-project.txt

# Exclude patterns
omnivore git ./my-project --exclude "test,*.log,tmp/*" --output filtered-code.txt
```

### Configuration

```bash
# Interactive setup wizard
omnivore setup

# Set OpenAI API key for AI features
omnivore config set openai_api_key YOUR_API_KEY

# View current configuration
omnivore config show
```

## Key Commands

### `omnivore crawl`
Crawl websites with advanced options:
- Multi-threaded crawling with configurable workers
- Respect robots.txt and politeness delays
- Extract structured data, metadata, and content
- Support for JavaScript-rendered pages (browser mode)
- AI-powered intelligent extraction

### `omnivore git`
Extract and analyze code repositories:
- Smart project type detection (Next.js, Python, Rust, etc.)
- Intelligent file filtering (skip node_modules, build artifacts, etc.)
- Organized output with project structure and metadata
- Support for both Git repositories and regular directories
- Multiple output formats (text, JSON)

### `omnivore config`
Manage configuration:
- Set API keys for AI features
- Configure default settings
- Export/import configurations

## Advanced Usage

### AI-Powered Extraction

```bash
# Use GPT-4 for intelligent content extraction
omnivore crawl https://docs.example.com \
  --ai-extract \
  --ai-model gpt-4o \
  --ai-prompt "Extract API endpoints and their descriptions"
```

### Browser Mode for JavaScript Sites

```bash
# Crawl JavaScript-heavy sites
omnivore crawl https://app.example.com \
  --browser \
  --wait 3000 \
  --screenshot \
  --interactive
```

### Building Knowledge Graphs

```bash
# Generate a knowledge graph from crawled data
omnivore crawl https://wiki.example.com --output wiki.json
omnivore graph wiki.json --output knowledge-graph.db
```

### Code Repository Analysis with Smart Defaults

```bash
# Omnivore automatically detects project type and applies smart filters
omnivore git https://github.com/vercel/next.js --output nextjs.txt

# Override smart defaults
omnivore git ./my-project --no-smart-filter --include "*.js,*.ts"
```

## Configuration File

Create a `.omnivore.toml` file in your project or home directory:

```toml
[crawl]
default_workers = 10
default_depth = 3
respect_robots_txt = true
user_agent = "Omnivore/1.0"

[git]
smart_filter = true
max_file_size = 10485760  # 10MB
exclude_binary = true

[ai]
default_model = "gpt-4o-mini"
max_tokens = 2000
```

## Examples

### Crawl documentation site and extract API references
```bash
omnivore crawl https://docs.rust-lang.org \
  --depth 3 \
  --include-pattern "/std/*" \
  --ai-extract \
  --ai-prompt "Extract function signatures and descriptions" \
  --output rust-std-api.json
```

### Analyze a TypeScript project
```bash
omnivore git ./my-typescript-project \
  --only ts,tsx,json \
  --exclude "node_modules,dist,*.test.ts" \
  --output project-analysis.txt
```

### Create a knowledge graph from a website
```bash
# Step 1: Crawl the website
omnivore crawl https://en.wikipedia.org/wiki/Artificial_intelligence \
  --depth 2 \
  --max-pages 100 \
  --output ai-wiki.json

# Step 2: Generate knowledge graph
omnivore graph ai-wiki.json \
  --output ai-knowledge.db \
  --extract-entities \
  --extract-relationships
```

## Environment Variables

- `OMNIVORE_CONFIG_DIR`: Custom configuration directory
- `OPENAI_API_KEY`: OpenAI API key for AI features
- `OMNIVORE_USER_AGENT`: Custom user agent for crawling
- `OMNIVORE_LOG_LEVEL`: Logging level (debug, info, warn, error)

## Documentation

For comprehensive documentation, examples, and guides, visit:
- 📚 [Full Documentation](https://ov.pranavkarra.me/docs)
- 🌐 [Project Website](https://ov.pranavkarra.me)
- 📖 [API Reference](https://docs.rs/omnivore-cli)

## Contributing

Contributions are welcome! Please check out our [contributing guidelines](https://github.com/Pranav-Karra-3301/omnivore/blob/master/CONTRIBUTING.md).

## License

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- [Homepage](https://ov.pranavkarra.me)
- [Documentation](https://ov.pranavkarra.me/docs)
- [GitHub Repository](https://github.com/Pranav-Karra-3301/omnivore)
- [Crates.io](https://crates.io/crates/omnivore-cli)
- [Issue Tracker](https://github.com/Pranav-Karra-3301/omnivore/issues)