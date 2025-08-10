# Installation Guide

This guide covers multiple ways to install Omnivore, a universal web crawler and knowledge graph builder.

## Table of Contents

- [Homebrew (Recommended for macOS/Linux)](#homebrew)
- [Docker](#docker)
- [Pre-built Binaries](#pre-built-binaries)
- [Build from Source](#build-from-source)
- [Verification](#verification)

## Homebrew

### Prerequisites
- macOS or Linux
- [Homebrew](https://brew.sh/) installed

### Install from Tap

```bash
# Add the Omnivore tap
brew tap Pranav-Karra-3301/omnivore

# Install Omnivore
brew install omnivore
```

### Alternative Installation Methods

#### Install from Source
```bash
# Install dependencies and build from source
brew install --build-from-source omnivore
```

#### Install HEAD Version
```bash
# Install the latest development version
brew install --HEAD omnivore
```

### What Gets Installed

The Homebrew formula installs:
- `omnivore` - CLI tool for web crawling
- `omnivore-api` - REST/GraphQL API server
- Configuration files in `/usr/local/etc/omnivore/`
- Shell completions for bash, zsh, and fish
- Data directory at `/usr/local/var/omnivore/`

### Configuration

Configuration file location:
- **macOS**: `/usr/local/etc/omnivore/crawler.toml`
- **Linux**: `/home/linuxbrew/.linuxbrew/etc/omnivore/crawler.toml`

Data storage location:
- **macOS**: `/usr/local/var/omnivore/`
- **Linux**: `/home/linuxbrew/.linuxbrew/var/omnivore/`

### Running as a Service

```bash
# Start the API server as a service
brew services start omnivore

# Check service status
brew services list | grep omnivore

# Stop the service
brew services stop omnivore
```

### Manual Usage

```bash
# Basic crawl
omnivore crawl https://example.com

# Start API server manually
omnivore-api

# Check API health
curl http://localhost:3000/health
```

---

## Docker

### Prerequisites
- [Docker](https://docs.docker.com/get-docker/) installed
- [Docker Compose](https://docs.docker.com/compose/install/) (for full stack)

### Quick Start

#### Single Container
```bash
# Pull and run the latest image
docker run -p 3000:3000 ghcr.io/pranav-karra-3301/omnivore:latest

# Access the API
curl http://localhost:3000/health
```

#### Full Stack with Docker Compose

```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Start all services
docker-compose up -d

# Check service status
docker-compose ps

# View logs
docker-compose logs -f omnivore-api
```

### Available Services

| Service | Port | Description |
|---------|------|-------------|
| omnivore-api | 3000 | Main API server |
| postgres | 5432 | Database |
| redis | 6379 | Cache/session storage |
| selenium | 4444 | Browser automation (optional) |
| prometheus | 9090 | Metrics (optional) |
| grafana | 3001 | Dashboards (optional) |

### Optional Profiles

```bash
# Start with browser automation
docker-compose --profile browser up -d

# Start with monitoring stack
docker-compose --profile monitoring up -d

# Start with both
docker-compose --profile browser --profile monitoring up -d
```

### Production Setup

```bash
# Copy environment file
cp env.example .env
# Edit .env with your configuration

# Use production compose file
docker-compose -f docker-compose.prod.yml up -d
```

### Docker Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level |
| `OMNIVORE_DATA_DIR` | `/var/lib/omnivore` | Data directory |
| `DATABASE_URL` | `postgresql://...` | Database connection |
| `REDIS_URL` | `redis://redis:6379` | Redis connection |
| `SELENIUM_URL` | `http://selenium:4444` | Browser automation |

---

## Pre-built Binaries

### Download

Visit the [Releases page](https://github.com/Pranav-Karra-3301/omnivore/releases) and download the appropriate binary for your platform:

- **macOS (Intel)**: `omnivore-v0.1.0-x86_64-apple-darwin.tar.gz`
- **macOS (Apple Silicon)**: `omnivore-v0.1.0-aarch64-apple-darwin.tar.gz`
- **Linux (x86_64)**: `omnivore-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
- **Linux (ARM64)**: `omnivore-v0.1.0-aarch64-unknown-linux-gnu.tar.gz`

### Install

```bash
# Download and extract (replace with your platform)
curl -L https://github.com/Pranav-Karra-3301/omnivore/releases/download/v0.1.0/omnivore-v0.1.0-x86_64-apple-darwin.tar.gz | tar -xz

# Move binaries to PATH
sudo mv omnivore-v0.1.0-x86_64-apple-darwin/omnivore /usr/local/bin/
sudo mv omnivore-v0.1.0-x86_64-apple-darwin/omnivore-api /usr/local/bin/

# Make executable
sudo chmod +x /usr/local/bin/omnivore
sudo chmod +x /usr/local/bin/omnivore-api

# Verify installation
omnivore --version
```

### Shell Completions

```bash
# Extract completions from the archive
cp omnivore-v0.1.0-*/completions/* ~/.local/share/

# Add to your shell profile
echo 'source ~/.local/share/omnivore.bash' >> ~/.bashrc  # Bash
echo 'source ~/.local/share/_omnivore' >> ~/.zshrc      # Zsh
```

---

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.88 or later
- `pkg-config`
- `openssl` development libraries

#### Platform-specific Dependencies

**macOS (with Homebrew):**
```bash
brew install pkg-config openssl
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**CentOS/RHEL/Fedora:**
```bash
sudo dnf install gcc pkg-config openssl-devel  # Fedora
sudo yum install gcc pkg-config openssl-devel  # CentOS/RHEL
```

### Build and Install

```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Build in release mode
cargo build --release

# Install locally
make install

# Or install using cargo
cargo install --path omnivore-cli
cargo install --path omnivore-api
```

### Generate Shell Completions

```bash
# Generate completions
make completions

# Install completions
source completions/omnivore.bash      # Bash
source completions/_omnivore          # Zsh
source completions/omnivore.fish      # Fish
```

### Development Build

```bash
# Quick development build
make dev

# Run tests
make test

# Run with hot reload
make watch
```

---

## Verification

### Test CLI Installation

```bash
# Check version
omnivore --version

# Test basic functionality
omnivore --help

# Test configuration
omnivore stats
```

### Test API Installation

```bash
# Start API server
omnivore-api &

# Test health endpoint
curl http://localhost:3000/health

# Test API version
curl http://localhost:3000/api/version

# Kill the background process
kill %1
```

### Test Full Functionality

```bash
# Basic crawl test
omnivore crawl https://httpbin.org/html --workers 2 --depth 1

# Check crawl statistics
omnivore stats

# Start API and test GraphQL
omnivore-api &
curl -X POST http://localhost:3000/graphql -H "Content-Type: application/json" -d '{"query":"{ health }"}'
```

---

## Troubleshooting

### Common Issues

**Build from source fails with SSL errors:**
```bash
# macOS
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"

# Linux
sudo apt install libssl-dev pkg-config
```

**Permission denied when installing binaries:**
```bash
# Use sudo for system-wide installation
sudo mv omnivore /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
mv omnivore ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

**Docker compose fails to start:**
```bash
# Check Docker daemon is running
docker info

# Check available ports
lsof -i :3000

# Check logs
docker-compose logs omnivore-api
```

### Getting Help

- **Documentation**: Visit our [docs](https://github.com/Pranav-Karra-3301/omnivore/tree/main/docs)
- **Issues**: Report bugs on [GitHub Issues](https://github.com/Pranav-Karra-3301/omnivore/issues)
- **Discussions**: Join [GitHub Discussions](https://github.com/Pranav-Karra-3301/omnivore/discussions)

---

## Next Steps

After installation, check out:

- [Quickstart Guide](docs/quickstart.md) - Get started with basic crawling
- [Configuration Guide](docs/configuration.md) - Configure Omnivore for your needs
- [API Documentation](docs/api/) - Learn about the REST and GraphQL APIs
- [Examples](docs/examples/) - See practical usage examples
