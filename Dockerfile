# syntax=docker/dockerfile:1
# Multi-stage build for Rust library
# Stage 1: Build the library
FROM rust:1.83-slim AS builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY clippy.toml ./clippy.toml

# Build the library in release mode
RUN cargo build --release --all-features

# Run tests to ensure the build is correct
RUN cargo test --release --all-features

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built library artifacts
COPY --from=builder /build/target/release/libcto_parallel_test.rlib /usr/local/lib/
COPY --from=builder /build/target/release/deps/ /usr/local/lib/deps/

# Set library path
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Add metadata
LABEL org.opencontainers.image.title="CTO Parallel Test"
LABEL org.opencontainers.image.description="E-commerce API Library - Product Catalog"
LABEL org.opencontainers.image.vendor="5dlabs"
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"

# Default command (this is a library, so we just provide a shell)
CMD ["/bin/bash"]
