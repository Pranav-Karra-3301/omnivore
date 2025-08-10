#!/bin/sh
# Omnivore installer script
# This script downloads and installs the Omnivore web crawler

set -e

# Configuration
REPO="Pranav-Karra-3301/omnivore"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
CONFIG_DIR="${CONFIG_DIR:-/usr/local/etc/omnivore}"
GITHUB_URL="https://github.com/${REPO}"
GITHUB_API="https://api.github.com/repos/${REPO}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="darwin"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac
    
    # Construct target triple
    if [ "$OS" = "linux" ]; then
        TARGET="${ARCH}-unknown-linux-gnu"
    elif [ "$OS" = "darwin" ]; then
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        else
            TARGET="aarch64-apple-darwin"
        fi
    fi
    
    info "Detected platform: $OS ($ARCH)"
    info "Target: $TARGET"
}

# Get latest release version
get_latest_version() {
    if command_exists curl; then
        VERSION=$(curl -s "${GITHUB_API}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command_exists wget; then
        VERSION=$(wget -qO- "${GITHUB_API}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
    
    if [ -z "$VERSION" ]; then
        warn "Could not determine latest version, using v0.1.0-beta"
        VERSION="v0.1.0-beta"
    fi
    
    info "Latest version: $VERSION"
}

# Download and extract binary
download_and_install() {
    DOWNLOAD_URL="${GITHUB_URL}/releases/download/${VERSION}/omnivore-${VERSION}-${TARGET}.tar.gz"
    TMP_DIR="$(mktemp -d)"
    
    info "Downloading from: $DOWNLOAD_URL"
    
    cd "$TMP_DIR"
    
    if command_exists curl; then
        curl -L -o omnivore.tar.gz "$DOWNLOAD_URL" || error "Failed to download"
    else
        wget -O omnivore.tar.gz "$DOWNLOAD_URL" || error "Failed to download"
    fi
    
    info "Extracting archive..."
    tar -xzf omnivore.tar.gz
    
    # Find the extracted directory
    EXTRACT_DIR=$(find . -type d -name "omnivore-*" | head -1)
    
    if [ -z "$EXTRACT_DIR" ]; then
        error "Failed to find extracted directory"
    fi
    
    cd "$EXTRACT_DIR"
    
    # Check if we need sudo for installation
    if [ -w "$INSTALL_DIR" ]; then
        SUDO=""
    else
        if command_exists sudo; then
            info "Installation requires sudo privileges"
            SUDO="sudo"
        else
            error "Cannot write to $INSTALL_DIR and sudo is not available"
        fi
    fi
    
    # Install binaries
    info "Installing binaries to $INSTALL_DIR..."
    $SUDO install -m 755 omnivore "$INSTALL_DIR/omnivore"
    $SUDO install -m 755 omnivore-api "$INSTALL_DIR/omnivore-api"
    
    # Install config files
    if [ -d "configs" ]; then
        info "Installing configuration files to $CONFIG_DIR..."
        $SUDO mkdir -p "$CONFIG_DIR"
        $SUDO cp -r configs/* "$CONFIG_DIR/" 2>/dev/null || true
    fi
    
    # Install shell completions if available
    if [ -d "completions" ]; then
        install_completions
    fi
    
    # Clean up
    cd /
    rm -rf "$TMP_DIR"
    
    info "Installation complete!"
}

# Install shell completions
install_completions() {
    info "Installing shell completions..."
    
    # Detect shell
    if [ -n "$BASH_VERSION" ]; then
        SHELL_NAME="bash"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_NAME="zsh"
    else
        SHELL_NAME=$(basename "$SHELL")
    fi
    
    case "$SHELL_NAME" in
        bash)
            if [ -d "/usr/local/etc/bash_completion.d" ]; then
                $SUDO cp completions/omnivore.bash "/usr/local/etc/bash_completion.d/omnivore" 2>/dev/null || true
            elif [ -d "/etc/bash_completion.d" ]; then
                $SUDO cp completions/omnivore.bash "/etc/bash_completion.d/omnivore" 2>/dev/null || true
            fi
            ;;
        zsh)
            if [ -d "/usr/local/share/zsh/site-functions" ]; then
                $SUDO cp completions/_omnivore "/usr/local/share/zsh/site-functions/_omnivore" 2>/dev/null || true
            fi
            ;;
        fish)
            if [ -d "$HOME/.config/fish/completions" ]; then
                cp completions/omnivore.fish "$HOME/.config/fish/completions/omnivore.fish" 2>/dev/null || true
            fi
            ;;
    esac
}

# Verify installation
verify_installation() {
    if command_exists omnivore; then
        VERSION_OUTPUT=$(omnivore --version 2>&1 || echo "unknown")
        info "Omnivore installed successfully!"
        info "Version: $VERSION_OUTPUT"
        echo ""
        echo "To get started, run:"
        echo "  omnivore --help"
        echo ""
        echo "To start a crawl:"
        echo "  omnivore crawl https://example.com"
        echo ""
        echo "To start the API server:"
        echo "  omnivore-api"
    else
        warn "Installation may have failed. Please check the output above."
    fi
}

# Main installation flow
main() {
    echo "================================"
    echo " Omnivore Installer"
    echo "================================"
    echo ""
    
    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --version)
                VERSION="$2"
                shift 2
                ;;
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --version VERSION     Install specific version (default: latest)"
                echo "  --install-dir DIR     Installation directory (default: /usr/local/bin)"
                echo "  --help               Show this help message"
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done
    
    # Check prerequisites
    if ! command_exists tar; then
        error "tar is required but not installed"
    fi
    
    detect_platform
    
    if [ -z "$VERSION" ]; then
        get_latest_version
    fi
    
    download_and_install
    verify_installation
    
    echo "================================"
    echo " Installation Complete!"
    echo "================================"
}

# Run main function
main "$@"