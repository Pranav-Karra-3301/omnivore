# Omnivore Homebrew Formula

This directory contains the Homebrew formula for installing Omnivore on macOS and Linux systems.

## Installation

### From this tap (once published):
```bash
brew tap Pranav-Karra-3301/omnivore
brew install omnivore
```

### From source:
```bash
brew install --build-from-source omnivore
```

## Usage

After installation, you can use Omnivore with:

```bash
# Start crawling a website
omnivore crawl https://example.com --workers 10 --depth 5

# Start the API server
omnivore-api

# Or use brew services to manage the API server
brew services start omnivore
brew services stop omnivore
```

## Development

### Testing the formula locally:

```bash
# Install from local formula
brew install --build-from-source ./omnivore.rb

# Test the formula
brew test omnivore

# Audit the formula
brew audit --strict omnivore.rb
```

### Updating the formula:

1. Update the `url` and `sha256` in the formula
2. Test the installation
3. Submit a PR to the tap repository

## Formula Structure

- `omnivore.rb` - Main Homebrew formula
- Installs both `omnivore` CLI and `omnivore-api` server
- Includes a launchd service for the API server
- Comprehensive test suite

## Requirements

- Rust toolchain (automatically installed as dependency)
- macOS 10.14+ or Linux with glibc 2.17+