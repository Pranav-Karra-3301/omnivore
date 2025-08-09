# Omnivore Web Crawler - Development Makefile

.PHONY: build test clean install dev fmt clippy audit completions help

# Default target
.DEFAULT_GOAL := help

# Build configuration
CARGO_FLAGS ?= --release
TARGET_DIR = target/release

help: ## Show this help message
	@echo "Omnivore Web Crawler - Development Commands"
	@echo "Usage: make <target>"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

build: ## Build all binaries in release mode
	cargo build $(CARGO_FLAGS)

build-debug: ## Build all binaries in debug mode
	cargo build

test: ## Run all tests
	cargo test --all --all-features

test-unit: ## Run unit tests only
	cargo test --lib --all

test-integration: ## Run integration tests only
	cargo test --test '*' --all

bench: ## Run benchmarks
	cargo bench

clippy: ## Run Clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code with rustfmt
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

audit: ## Audit dependencies for security vulnerabilities
	cargo audit

clean: ## Clean build artifacts
	cargo clean
	rm -rf completions/

install: build completions ## Install omnivore locally
	cargo install --path omnivore-cli --force
	cargo install --path omnivore-api --force
	@echo "Omnivore installed! Add shell completions with:"
	@echo "  bash: source completions/omnivore.bash"
	@echo "  zsh: source completions/_omnivore"  
	@echo "  fish: source completions/omnivore.fish"

dev-install: build-debug ## Install development version
	cargo install --path omnivore-cli --force --debug
	cargo install --path omnivore-api --force --debug

uninstall: ## Uninstall omnivore
	cargo uninstall omnivore || true
	cargo uninstall omnivore-api || true

completions: build ## Generate shell completions
	./scripts/build-completions.sh

dev: ## Start development environment
	@echo "Starting Omnivore development environment..."
	@echo "Building project..."
	@make build-debug
	@echo "Running tests..."
	@make test-unit
	@echo "Development environment ready!"

# Docker targets
docker-build: ## Build Docker image
	docker build -t omnivore:latest .

docker-run: ## Run Docker container
	docker run -it --rm -p 3000:3000 omnivore:latest

# Release targets  
release-dry-run: ## Dry run release build
	cargo build --release --all-targets
	@echo "Release build successful!"

release-prep: fmt clippy test audit ## Prepare for release
	@echo "Pre-release checks passed!"

# CI targets
ci: fmt-check clippy test audit ## Run all CI checks
	@echo "All CI checks passed!"

ci-coverage: ## Generate test coverage report
	cargo install cargo-tarpaulin --locked
	cargo tarpaulin --out html --output-dir coverage/

# Documentation
docs: ## Generate documentation
	cargo doc --all-features --no-deps --open

docs-build: ## Build documentation without opening
	cargo doc --all-features --no-deps

# Profiling and analysis
profile: ## Profile the application (requires cargo-flamegraph)
	cargo install flamegraph
	cargo flamegraph --bin omnivore -- crawl https://example.com --workers 1 --depth 1

bloat: ## Analyze binary size (requires cargo-bloat)
	cargo install cargo-bloat
	cargo bloat --release -n 20

# Database and storage
clean-data: ## Clean up data directory
	rm -rf data/ || true
	mkdir -p data

reset-db: ## Reset database (for development)
	rm -rf data/*.db || true

# Homebrew formula
homebrew-audit: ## Audit Homebrew formula
	cd Formula && make audit

homebrew-test: ## Test Homebrew formula
	cd Formula && make test

# Examples and demo
demo: build ## Run a demo crawl
	@echo "Running Omnivore demo..."
	./target/release/omnivore crawl https://httpbin.org/html --workers 2 --depth 1
	
example-api: build ## Start API server for demo
	@echo "Starting Omnivore API server..."
	@echo "Visit http://localhost:3000 for API documentation"
	./target/release/omnivore-api

# Performance testing
perf-test: build ## Run performance tests
	@echo "Running performance tests..."
	time ./target/release/omnivore crawl https://httpbin.org/html --workers 10 --depth 2

load-test: ## Run load testing (requires wrk)
	wrk -t12 -c400 -d30s http://localhost:3000/health

# Development tools
watch: ## Watch for changes and rebuild
	cargo install cargo-watch
	cargo watch -x 'build --bin omnivore' -x 'test --lib'

watch-run: ## Watch and run CLI
	cargo install cargo-watch  
	cargo watch -x 'run --bin omnivore -- --help'

# Environment setup
setup: ## Set up development environment
	rustup component add clippy rustfmt
	cargo install cargo-audit cargo-watch
	@echo "Development environment setup complete!"

# Statistics
stats: ## Show project statistics
	@echo "Omnivore Project Statistics:"
	@echo "Lines of code:"
	find . -name "*.rs" -not -path "./target/*" | xargs wc -l | tail -1
	@echo ""
	@echo "Test coverage:"
	cargo test 2>&1 | grep "test result" || echo "Run 'make test' first"

check-deps: ## Check for outdated dependencies
	cargo outdated

update-deps: ## Update dependencies
	cargo update

# Security
security-audit: audit ## Alias for audit

security-scan: ## Run security scan (requires cargo-deny)
	cargo install cargo-deny
	cargo deny check

verify: fmt-check clippy test audit ## Verify everything is ready
	@echo "✅ All verification checks passed!"