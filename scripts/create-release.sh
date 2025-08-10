#!/bin/bash
# Create a release for Omnivore
# Usage: ./scripts/create-release.sh [version]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Default version if not provided
VERSION="${1:-v0.1.0}"

# Ensure version starts with 'v'
if [[ ! "$VERSION" =~ ^v ]]; then
    VERSION="v$VERSION"
fi

echo "Creating release $VERSION for Omnivore..."

cd "$PROJECT_ROOT"

# Verify we're on the main branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    echo "Warning: Not on main/master branch (current: $CURRENT_BRANCH)"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "Error: There are uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Run pre-release checks
echo "Running pre-release checks..."

echo "  - Formatting check..."
cargo fmt --all -- --check

echo "  - Clippy check..."
cargo clippy --all-targets --all-features -- -D warnings

echo "  - Running tests..."
cargo test --all-features

echo "  - Security audit..."
cargo audit || echo "Warning: Security audit failed"

echo "  - Building release..."
cargo build --release

echo "Pre-release checks completed successfully!"

# Update version in Cargo.toml files
echo "Updating version to ${VERSION#v} in Cargo.toml files..."

# Update workspace version
sed -i.bak "s/^version = \".*\"/version = \"${VERSION#v}\"/" Cargo.toml

# Create tag
echo "Creating git tag $VERSION..."
git add .
git commit -m "Release $VERSION" || echo "No changes to commit"
git tag -a "$VERSION" -m "Release $VERSION"

echo "Release $VERSION created successfully!"
echo ""
echo "To publish the release:"
echo "  1. Push the tag: git push origin $VERSION"
echo "  2. GitHub Actions will automatically:"
echo "     - Build binaries for all platforms"
echo "     - Create a GitHub release"
echo "     - Build and push Docker images"
echo "     - Update the Homebrew formula"
echo ""
echo "Manual steps (if needed):"
echo "  - Check the GitHub Actions workflow"
echo "  - Verify the Homebrew formula was updated correctly"
echo "  - Test the installation from all methods"

# Restore backup
if [[ -f Cargo.toml.bak ]]; then
    rm Cargo.toml.bak
fi
