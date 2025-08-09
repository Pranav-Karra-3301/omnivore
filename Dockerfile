# Multi-stage build for Omnivore
FROM rust:1.88-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY omnivore-core/Cargo.toml ./omnivore-core/
COPY omnivore-cli/Cargo.toml ./omnivore-cli/
COPY omnivore-api/Cargo.toml ./omnivore-api/

# Create dummy source files to cache dependencies
RUN mkdir -p omnivore-core/src omnivore-cli/src omnivore-api/src \
    && echo "fn main() {}" > omnivore-cli/src/main.rs \
    && echo "fn main() {}" > omnivore-api/src/main.rs \
    && echo "pub fn dummy() {}" > omnivore-core/src/lib.rs

# Build dependencies
RUN cargo build --release --workspace
RUN rm -rf omnivore-*/src

# Copy actual source code
COPY omnivore-core/src ./omnivore-core/src
COPY omnivore-cli/src ./omnivore-cli/src  
COPY omnivore-api/src ./omnivore-api/src
COPY configs ./configs

# Build the actual application
RUN touch omnivore-*/src/lib.rs omnivore-*/src/main.rs \
    && cargo build --release --workspace

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create omnivore user
RUN useradd -m -u 1001 omnivore

# Copy binaries and config
COPY --from=builder /app/target/release/omnivore /usr/local/bin/
COPY --from=builder /app/target/release/omnivore-api /usr/local/bin/
COPY --from=builder /app/configs /etc/omnivore/configs

# Create data directory
RUN mkdir -p /var/lib/omnivore && chown omnivore:omnivore /var/lib/omnivore

# Switch to non-root user
USER omnivore
WORKDIR /home/omnivore

# Set environment variables
ENV OMNIVORE_DATA_DIR=/var/lib/omnivore
ENV OMNIVORE_CONFIG=/etc/omnivore/configs/crawler.toml

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Expose API port
EXPOSE 3000

# Default command
CMD ["omnivore-api"]