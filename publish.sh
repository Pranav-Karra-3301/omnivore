#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Omnivore Publishing Script${NC}"
echo ""

# Check if cargo is logged in
if ! cargo owner --list omnivore-core 2>/dev/null | grep -q .; then
    echo -e "${YELLOW}⚠️  You need to login to crates.io first${NC}"
    echo "Run: cargo login"
    echo "Get your token from: https://crates.io/me"
    exit 1
fi

echo -e "${GREEN}📦 Publishing omnivore-core v0.1.1...${NC}"
cd omnivore-core

# Verify it builds
echo "Building omnivore-core..."
cargo build --release

# Publish
cargo publish

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to publish omnivore-core${NC}"
    exit 1
fi

cd ..

echo -e "${YELLOW}⏳ Waiting 30 seconds for crates.io to index omnivore-core...${NC}"
sleep 30

echo -e "${GREEN}🔧 Updating omnivore-cli dependency...${NC}"
# Update the dependency to use the published version
sed -i '' 's/omnivore-core = { version = "0.1", path = "..\/omnivore-core" }/omnivore-core = "0.1.1"/' omnivore-cli/Cargo.toml

echo -e "${GREEN}📦 Publishing omnivore-cli v0.2.0...${NC}"
cd omnivore-cli

# Verify it builds
echo "Building omnivore-cli..."
cargo build --release

# Publish
cargo publish

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to publish omnivore-cli${NC}"
    echo -e "${YELLOW}Reverting Cargo.toml changes...${NC}"
    git checkout Cargo.toml
    exit 1
fi

cd ..

echo ""
echo -e "${GREEN}✅ Successfully published both crates!${NC}"
echo ""
echo "📚 Links:"
echo "  - omnivore-core: https://crates.io/crates/omnivore-core"
echo "  - omnivore-cli: https://crates.io/crates/omnivore-cli"
echo "  - Documentation: https://ov.pranavkarra.me/docs"
echo "  - Homepage: https://ov.pranavkarra.me"
echo ""
echo -e "${YELLOW}💡 Users can now install with: cargo install omnivore-cli${NC}"