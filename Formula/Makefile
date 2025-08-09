# Homebrew Formula Management for Omnivore

.PHONY: audit install test uninstall update clean help

# Formula name
FORMULA_NAME = omnivore
FORMULA_FILE = omnivore.rb

# GitHub repository details
GITHUB_USER = Pranav-Karra-3301
GITHUB_REPO = omnivore

help: ## Show this help message
	@echo "Omnivore Homebrew Formula Management"
	@echo "Usage: make <target>"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

audit: ## Audit the Homebrew formula
	brew audit --strict $(FORMULA_FILE)

install: ## Install Omnivore using the local formula
	brew install --build-from-source ./$(FORMULA_FILE)

install-head: ## Install Omnivore from HEAD
	brew install --HEAD ./$(FORMULA_FILE)

test: ## Test the installed formula
	brew test $(FORMULA_NAME)

uninstall: ## Uninstall Omnivore
	brew uninstall $(FORMULA_NAME)

reinstall: uninstall install ## Reinstall Omnivore

update-sha: ## Update SHA256 hashes for release (requires VERSION env var)
	@echo "Updating SHA256 hashes for version $(VERSION)"
	@if [ -z "$(VERSION)" ]; then echo "Please set VERSION environment variable"; exit 1; fi
	@echo "Download release files and update SHA256 hashes in $(FORMULA_FILE)"

validate: audit ## Validate the formula
	brew install --dry-run ./$(FORMULA_FILE)

clean: ## Clean up temporary files
	brew cleanup $(FORMULA_NAME) || true

info: ## Show formula information
	brew info ./$(FORMULA_FILE)

create-tap: ## Create a new Homebrew tap repository
	@echo "Creating Homebrew tap at homebrew-$(FORMULA_NAME)"
	@if [ ! -d "../homebrew-$(FORMULA_NAME)" ]; then \
		mkdir -p ../homebrew-$(FORMULA_NAME)/Formula; \
		cp $(FORMULA_FILE) ../homebrew-$(FORMULA_NAME)/Formula/; \
		cp README.md ../homebrew-$(FORMULA_NAME)/; \
		echo "Tap created at ../homebrew-$(FORMULA_NAME)"; \
	else \
		echo "Tap directory already exists"; \
	fi

sync-tap: ## Sync formula to tap repository  
	@if [ -d "../homebrew-$(FORMULA_NAME)" ]; then \
		cp $(FORMULA_FILE) ../homebrew-$(FORMULA_NAME)/Formula/; \
		cp README.md ../homebrew-$(FORMULA_NAME)/; \
		echo "Formula synced to tap"; \
	else \
		echo "Tap directory does not exist. Run 'make create-tap' first"; \
	fi

# Development helpers
dev-install: ## Install development version
	cargo build --release
	ln -sf $(PWD)/target/release/omnivore /usr/local/bin/omnivore-dev
	ln -sf $(PWD)/target/release/omnivore-api /usr/local/bin/omnivore-api-dev

dev-uninstall: ## Remove development version
	rm -f /usr/local/bin/omnivore-dev /usr/local/bin/omnivore-api-dev

# Release helpers
prepare-release: ## Prepare for a new release (requires VERSION env var)
	@echo "Preparing release $(VERSION)"
	@if [ -z "$(VERSION)" ]; then echo "Please set VERSION environment variable"; exit 1; fi
	sed -i.bak 's/version ".*"/version "$(VERSION)"/' $(FORMULA_FILE)
	@echo "Updated version to $(VERSION) in $(FORMULA_FILE)"
	@echo "Don't forget to:"
	@echo "1. Build release binaries"
	@echo "2. Update SHA256 hashes with 'make update-sha VERSION=$(VERSION)'"
	@echo "3. Test with 'make install test'"
	@echo "4. Commit and push changes"

style-check: ## Check Ruby style (requires rubocop)
	rubocop $(FORMULA_FILE) || true

# CI/Testing
ci-test: audit validate ## Run CI tests
	@echo "All CI tests passed!"

.DEFAULT_GOAL := help