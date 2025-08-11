# Installation Guide

Omnivore can be installed through multiple methods depending on your platform and preferences. This guide covers all installation options in detail.

## Table of Contents
- [System Requirements](#system-requirements)
- [Quick Install Script](#quick-install-script)
- [Homebrew Installation](#homebrew-installation)
- [Binary Installation](#binary-installation)
- [Building from Source](#building-from-source)
- [Docker Installation](#docker-installation)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements
- **Operating System**: Linux, macOS, or Windows (WSL2 recommended)
- **Memory**: 2GB RAM (4GB+ recommended for large crawls)
- **Disk Space**: 500MB for binaries + space for crawled data
- **Network**: Stable internet connection

### Required Dependencies
- **Git**: For repository analysis features
- **OpenSSL**: For HTTPS support (usually pre-installed)

### Optional Dependencies
- **ChromeDriver**: For browser mode (JavaScript rendering)
- **Docker**: For containerized deployment

## Quick Install Script

The fastest way to install Omnivore on Linux and macOS:

```bash
# Install latest stable release
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh

# Install specific version
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh -s -- --version v0.4.0

# Install to custom directory
curl -sSfL https://raw.githubusercontent.com/Pranav-Karra-3301/omnivore/master/install.sh | sh -s -- --install-dir ~/bin
```

## Homebrew Installation

For macOS and Linux users with Homebrew:

```bash
# Add the Omnivore tap
brew tap Pranav-Karra-3301/omnivore

# Install Omnivore
brew install omnivore

# Upgrade to latest version
brew upgrade omnivore
```

## Binary Installation

### Download Pre-built Binaries

1. Visit the [releases page](https://github.com/Pranav-Karra-3301/omnivore/releases)
2. Download the appropriate archive for your platform:

| Platform | Architecture | File |
|----------|-------------|------|
| Linux | x86_64 | `omnivore-vX.X.X-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | ARM64 | `omnivore-vX.X.X-aarch64-unknown-linux-gnu.tar.gz` |
| macOS | Intel | `omnivore-vX.X.X-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `omnivore-vX.X.X-aarch64-apple-darwin.tar.gz` |
| Windows | x86_64 | `omnivore-vX.X.X-x86_64-pc-windows-msvc.zip` |

### Manual Installation Steps

```bash
# Download (replace VERSION and PLATFORM)
wget https://github.com/Pranav-Karra-3301/omnivore/releases/download/vVERSION/omnivore-vVERSION-PLATFORM.tar.gz

# Extract
tar -xzf omnivore-*.tar.gz

# Navigate to extracted directory
cd omnivore-*/

# Install binaries
sudo install -m 755 omnivore /usr/local/bin/
sudo install -m 755 omnivore-api /usr/local/bin/

# Or install to user directory (no sudo required)
mkdir -p ~/.local/bin
install -m 755 omnivore ~/.local/bin/
# Add ~/.local/bin to PATH in your shell config
```

## Building from Source

### Prerequisites
- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Git
- C compiler (gcc, clang, or MSVC)
- pkg-config (Linux/macOS)
- OpenSSL development headers

### Install Rust
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest stable
rustup update stable
```

### Clone and Build

```bash
# Clone repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Build in release mode
cargo build --release

# Install using make
make install

# Or manually copy binaries
sudo cp target/release/omnivore /usr/local/bin/
sudo cp target/release/omnivore-api /usr/local/bin/
```

### Build with Features
```bash
# Build with all features
cargo build --release --all-features

# Build with specific features
cargo build --release --features "browser,ai"
```

## Docker Installation

### Using Pre-built Images
```bash
# Pull latest image
docker pull ghcr.io/pranav-karra-3301/omnivore:latest

# Run interactive CLI
docker run -it --rm ghcr.io/pranav-karra-3301/omnivore:latest

# Run with volume mount for persistent data
docker run -it --rm \
  -v $(pwd)/data:/data \
  ghcr.io/pranav-karra-3301/omnivore:latest \
  crawl https://example.com --output /data/crawl.json
```

### Building Docker Image
```bash
# Build standard image
docker build -t omnivore:local .

# Build multi-stage optimized image
docker build -f Dockerfile.musl -t omnivore:minimal .
```

### Docker Compose Setup
```yaml
# docker-compose.yml
version: '3.8'

services:
  omnivore:
    image: ghcr.io/pranav-karra-3301/omnivore:latest
    volumes:
      - ./data:/data
      - ./config:/config
    environment:
      - RUST_LOG=info
      - OMNIVORE_CONFIG=/config/omnivore.toml

  omnivore-api:
    image: ghcr.io/pranav-karra-3301/omnivore:latest
    ports:
      - "3000:3000"
    volumes:
      - ./data:/data
    command: ["omnivore-api"]
```

## Verification

### Check Installation
```bash
# Check version
omnivore --version

# Check help
omnivore --help

# Run setup wizard
omnivore setup

# Test basic crawl
omnivore crawl https://example.com --depth 1
```

## Troubleshooting

### Common Issues

#### Permission Denied
```bash
# Fix permissions
chmod +x /usr/local/bin/omnivore
```

#### Command Not Found
```bash
# Add to PATH
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### OpenSSL Errors
```bash
# Linux (Debian/Ubuntu)
sudo apt-get install libssl-dev pkg-config

# macOS
brew install openssl
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

## Next Steps

After installation:
1. Run `omnivore setup` to configure API keys and defaults
2. Read the [Quickstart Guide](quickstart.md) for your first crawl
3. Check [CLI Reference](cli.md) for all commands
