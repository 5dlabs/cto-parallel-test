# Multi-stage build for optimized Docker image
# Stage 1: Builder
FROM rust:latest as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source tree
COPY src ./src
COPY migrations ./migrations
COPY clippy.toml ./

# Build for release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/cto_parallel_test /app/cto_parallel_test

# Copy migrations for runtime
COPY --from=builder /app/migrations /app/migrations

# Set the binary as the entrypoint
ENTRYPOINT ["/app/cto_parallel_test"]
