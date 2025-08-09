# Installation

## Homebrew (macOS/Linux)
```bash
brew tap Pranav-Karra-3301/omnivore
brew install omnivore
```

## From Source
```bash
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore
cargo build --release
make install # optional helper from Makefile
```

## Docker
```bash
# Build and run
docker build -t omnivore:latest .
docker run -it --rm -p 3000:3000 omnivore:latest

# Or compose (if provided)
docker-compose up -d
```
