# Multi-stage build for optimized Rust library container
# Stage 1: Build
FROM rust:1.86-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations

# Build the actual library
RUN touch src/lib.rs && \
    cargo build --release --lib

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built library artifacts (only .rlib exists for this library crate)
COPY --from=builder /build/target/release/libcto_parallel_test.rlib /app/
COPY --from=builder /build/migrations ./migrations

# Add metadata
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"
LABEL org.opencontainers.image.description="E-commerce API database schema library with Diesel ORM"
LABEL org.opencontainers.image.licenses="MIT"

# Default command (for demonstration)
CMD ["echo", "E-commerce API Schema Library - Use this image as a base for services"]
