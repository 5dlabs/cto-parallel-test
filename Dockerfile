# Multi-stage Dockerfile for Rust library
# Stage 1: Build
FROM rust:latest as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY clippy.toml ./

# Build the library and run tests
RUN cargo build --release --all-features && \
    cargo test --release --all-features

# Stage 2: Runtime (minimal image with library artifacts)
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy built artifacts from builder
COPY --from=builder /app/target/release/libcto_parallel_test.rlib /app/lib/
COPY --from=builder /app/target/release/deps/*.rlib /app/lib/deps/

# Set library path
ENV LD_LIBRARY_PATH=/app/lib:$LD_LIBRARY_PATH

# Metadata
LABEL org.opencontainers.image.title="cto-parallel-test"
LABEL org.opencontainers.image.description="Authentication library with JWT and Argon2 support"
LABEL org.opencontainers.image.vendor="5dlabs"

CMD ["/bin/bash"]
