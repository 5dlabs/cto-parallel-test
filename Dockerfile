# Multi-stage Docker build for Rust library
# This Dockerfile is designed for testing and CI/CD purposes
# Note: This is a library crate without a binary target

FROM rust:1.82-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY clippy.toml ./

# Copy source code
COPY src ./src

# Build and test the library
RUN cargo build --release --locked && \
    cargo test --release --locked

# Stage 2: Test runtime environment
FROM rust:1.82-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire project for testing
COPY --from=builder /app ./

# Set non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Default command: run tests
CMD ["cargo", "test", "--release"]
