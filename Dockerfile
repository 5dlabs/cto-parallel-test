# Multi-stage Dockerfile for Rust library project
# This builds the library and creates a minimal runtime image

# Build stage
FROM rust:1.83-bookworm AS builder

WORKDIR /build

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src
COPY clippy.toml ./

# Build the library in release mode
RUN cargo build --release --lib

# Runtime stage - minimal Debian image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy built library from builder stage
COPY --from=builder /build/target/release/libcto_parallel_test.rlib /app/
# Note: .so file only exists for cdylib crate-type, we're a regular lib (rlib)

# Set ownership
RUN chown -R app:app /app

USER app

# Add metadata
LABEL org.opencontainers.image.title="cto-parallel-test"
LABEL org.opencontainers.image.description="Product Catalog Module - E-commerce API Library"
LABEL org.opencontainers.image.vendor="5DLabs"

# Note: This is a library, not a standalone service
# Applications using this library should depend on this image or include the library in their builds
CMD ["echo", "This is a library crate. Use it as a dependency in your Rust application."]
