# Multi-stage build for Rust authentication library
FROM rust:1.90-slim AS builder

WORKDIR /usr/src/app

# Install system dependencies (including PostgreSQL for Diesel)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./
# Copy Cargo.lock if it exists (for reproducible builds)
COPY Cargo.lock* ./

# Copy source code
COPY src ./src
COPY clippy.toml ./

# Build the library and run tests
RUN cargo build --release --all-features && \
    cargo test --release --all-features

# Runtime stage - minimal image with the library artifacts
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (including PostgreSQL client library for Diesel)
RUN apt-get update && \
    apt-get install -y ca-certificates libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy the built library from builder
COPY --from=builder /usr/src/app/target/release/libcto_parallel_test.rlib /app/

# Copy source for documentation reference
COPY --from=builder /usr/src/app/src /app/src
COPY --from=builder /usr/src/app/Cargo.toml /app/

# Set metadata
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"
LABEL org.opencontainers.image.description="User authentication library with JWT and Argon2 password hashing"
LABEL org.opencontainers.image.licenses="MIT"

# Default command shows library info
CMD ["sh", "-c", "echo 'cto-parallel-test authentication library'; ls -lh /app/"]
