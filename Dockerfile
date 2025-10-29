# Multi-stage Dockerfile for Rust library
# Stage 1: Builder - compile the Rust code
FROM rust:1.85-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY clippy.toml ./

# Copy source code
COPY src ./src

# Build the release binary
RUN cargo build --release --all-features

# Run tests during build to ensure quality
RUN cargo test --release --all-features

# Stage 2: Runtime - create minimal runtime image
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

# Copy built artifacts from builder
COPY --from=builder /app/target/release/libcto_parallel_test.rlib /app/lib/

# Switch to non-root user
USER appuser

# Add metadata
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"
LABEL org.opencontainers.image.description="CTO Parallel Test - E-commerce Product Catalog Library"
LABEL org.opencontainers.image.licenses="MIT"

# Default command (can be overridden)
CMD ["/bin/bash"]
