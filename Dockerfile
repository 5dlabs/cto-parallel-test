# Multi-stage Dockerfile for Rust e-commerce API
# Stage 1: Build the library/application
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the library (for now - will be updated when binary is added)
RUN cargo build --release --lib

# Stage 2: Runtime (minimal)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 appuser

WORKDIR /app

# Copy build artifacts (for future binary)
# For now, this is a library-only crate, so we'll just validate the build succeeded
COPY --from=builder /app/target/release/libcto_parallel_test.rlib ./

USER appuser

# Placeholder CMD - will be updated when API endpoints are added
CMD ["echo", "Library built successfully. Add binary target to run application."]
