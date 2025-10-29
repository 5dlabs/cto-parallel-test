# Multi-stage Dockerfile for Rust authentication library
# Stage 1: Builder
# Use latest Rust nightly to support edition2024 dependencies
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY clippy.toml ./

# Build the library and run tests to verify
RUN cargo build --release && \
    cargo test --release

# Stage 2: Runtime (minimal image for library artifacts)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built library artifacts from builder
WORKDIR /app
COPY --from=builder /app/target/release/libcto_parallel_test.rlib /app/lib/
COPY --from=builder /app/target/release/deps/ /app/deps/

# Copy source for reference (useful for documentation)
COPY --from=builder /app/src /app/src
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

# Set metadata labels
LABEL org.opencontainers.image.title="CTO Parallel Test - Authentication Library"
LABEL org.opencontainers.image.description="Rust authentication library with JWT and Argon2 password hashing"
LABEL org.opencontainers.image.vendor="5DLabs"

# Default command shows library info
CMD ["echo", "CTO Parallel Test Authentication Library - Use as a dependency in your Rust projects"]
