# Build stage
FROM rust:1.83-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY clippy.toml ./

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/libcto_parallel_test.* /app/

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Metadata
LABEL org.opencontainers.image.title="CTO Parallel Test"
LABEL org.opencontainers.image.description="E-commerce API test project - Product Catalog Module"
LABEL org.opencontainers.image.vendor="5dlabs"
LABEL org.opencontainers.image.licenses="UNLICENSED"

# This is a library, so we'll use a simple command to keep container running
CMD ["tail", "-f", "/dev/null"]
