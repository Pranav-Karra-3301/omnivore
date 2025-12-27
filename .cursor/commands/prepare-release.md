# Prepare Release

Steps for preparing a new Omnivore release.

## Pre-Release Checklist

### 1. Code Quality

```bash
# Run full CI checks
make ci

# Run full test suite with all features
cargo test --workspace --all-features

# Check for outdated dependencies
cargo outdated
```

### 2. Update Version Numbers

Update versions in:

- [ ] `Cargo.toml` (workspace version)
- [ ] `omnivore-core/Cargo.toml`
- [ ] `omnivore-cli/Cargo.toml`

```toml
# Follow semver:
# MAJOR: breaking API changes
# MINOR: new features, backward compatible
# PATCH: bug fixes, backward compatible
version = "0.2.0"
```

### 3. Update CHANGELOG.md

Add release notes:

```markdown
## [0.2.0] - YYYY-MM-DD

### Added
- New feature X that does Y

### Changed
- Improved performance of Z

### Fixed
- Bug where A caused B

### Security
- Fixed vulnerability in C
```

### 4. Update Documentation

- [ ] README.md reflects current features
- [ ] CLI help text is accurate
- [ ] API documentation is current
- [ ] Examples work with new version

### 5. Final Verification

```bash
# Build release binaries
cargo build --release --workspace

# Test release binary
./target/release/omnivore --version
./target/release/omnivore --help

# Run release binary tests
./target/release/omnivore crawl https://example.com --depth 1
```

## Release Process

### 1. Create Git Tag

```bash
# Commit version changes
git add -A
git commit -m "chore: Prepare release v0.2.0"

# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push changes and tag
git push origin main
git push origin v0.2.0
```

### 2. GitHub Release

GitHub Actions will automatically:
- Create a GitHub release
- Build binaries for supported platforms
- Attach binaries to the release
- Update Homebrew formula

### 3. Publish to crates.io

```bash
# Publish core library first
cd omnivore-core
cargo publish

# Wait for indexing (2-5 minutes)
sleep 300

# Update omnivore-cli to use published version
# Change: omnivore-core = { path = "../omnivore-core" }
# To: omnivore-core = "0.2.0"

# Publish CLI
cd ../omnivore-cli
cargo publish
```

## Post-Release

- [ ] Verify GitHub release is published
- [ ] Verify crates.io packages are available
- [ ] Verify Homebrew formula is updated
- [ ] Announce release (if applicable)
- [ ] Update any dependent projects

## Rollback Plan

If issues are discovered:

```bash
# Delete the tag locally and remotely
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Yank the crate (prevents new downloads, existing continue working)
cargo yank --version 0.2.0 omnivore-core
cargo yank --version 0.2.0 omnivore

# Fix the issue, then re-release as v0.2.1
```
