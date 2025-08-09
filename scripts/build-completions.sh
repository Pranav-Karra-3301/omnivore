#!/bin/bash
# Build shell completions for Omnivore CLI

set -euo pipefail

# Build the CLI first
echo "Building omnivore CLI..."
cargo build --release --bin omnivore

# Create completions directory
mkdir -p completions

# Generate completions for different shells
echo "Generating completions..."

echo "  - bash completion..."
./target/release/omnivore generate-completions bash > completions/omnivore.bash

echo "  - zsh completion..."  
./target/release/omnivore generate-completions zsh > completions/_omnivore

echo "  - fish completion..."
./target/release/omnivore generate-completions fish > completions/omnivore.fish

echo "  - powershell completion..."
./target/release/omnivore generate-completions powershell > completions/_omnivore.ps1

echo "Shell completions generated in completions/"
ls -la completions/