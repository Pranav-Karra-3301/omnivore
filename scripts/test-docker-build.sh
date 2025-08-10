#!/bin/bash
# Test Docker build

set -e

echo "Testing Docker build..."
docker build -t omnivore:test .

echo "Testing musl Docker build..."
docker build -f Dockerfile.musl -t omnivore:test-musl .

echo "Docker builds completed successfully!"