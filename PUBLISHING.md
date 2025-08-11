# Publishing Omnivore to crates.io

## Prerequisites

1. **Crates.io Account**: Create an account at [crates.io](https://crates.io)
2. **API Token**: Get your API token from https://crates.io/me
3. **Login**: Run `cargo login` and paste your API token

## Publishing Steps

### Step 1: Publish omnivore-core

First, publish the core library:

```bash
cd omnivore-core
cargo publish
```

Wait for it to be available on crates.io (usually takes a few minutes).

### Step 2: Update omnivore-cli

Once omnivore-core 0.1.1 is published, update the dependency in `omnivore-cli/Cargo.toml`:

```toml
omnivore-core = "0.1.1"  # Remove the path dependency
```

### Step 3: Publish omnivore-cli

```bash
cd omnivore-cli
cargo publish
```

## Version Management

Current versions:
- omnivore-core: 0.1.1 (ready to publish)
- omnivore-cli: 0.2.0 (ready to publish after core)

## Features Included

The published crates include:

### omnivore-core
- Web crawling engine
- HTML parsing and extraction
- Browser automation support
- Knowledge graph building
- AI-powered extraction
- Rate limiting and politeness

### omnivore-cli
- `omnivore crawl` - Advanced web crawling
- `omnivore git` - Code repository analysis
- `omnivore config` - Configuration management
- `omnivore setup` - Interactive setup wizard
- Smart file filtering and project detection
- Multiple output formats

## Important Links

- Homepage: https://ov.pranavkarra.me
- Documentation: https://ov.pranavkarra.me/docs
- GitHub: https://github.com/Pranav-Karra-3301/omnivore
- Crates.io: https://crates.io/crates/omnivore-cli

## Troubleshooting

If you encounter issues:

1. **Version conflicts**: Ensure omnivore-core is published and indexed before publishing omnivore-cli
2. **Missing files**: Make sure README.md and LICENSE files are present
3. **Build errors**: Run `cargo build --release` locally first to verify everything compiles

## Quick Publish Script

Save this as `publish.sh`:

```bash
#!/bin/bash
set -e

echo "Publishing omnivore-core..."
cd omnivore-core
cargo publish
cd ..

echo "Waiting for crates.io to index omnivore-core..."
sleep 30

echo "Updating omnivore-cli dependency..."
sed -i '' 's/omnivore-core = { version = "0.1", path = "..\/omnivore-core" }/omnivore-core = "0.1.1"/' omnivore-cli/Cargo.toml

echo "Publishing omnivore-cli..."
cd omnivore-cli
cargo publish
cd ..

echo "✅ Successfully published both crates!"
echo "Visit https://crates.io/crates/omnivore-cli to see your published crate"
```

Make it executable with `chmod +x publish.sh` and run with `./publish.sh`.