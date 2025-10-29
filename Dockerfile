# Multi-stage build for optimal image size
FROM rust:slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release (without --locked to allow dependency updates)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the library artifacts from builder
# Note: This is a library crate, so we copy the build output for reference
COPY --from=builder /app/target/release/libcto_parallel_test.rlib /app/lib/
COPY --from=builder /app/target/release/deps/ /app/deps/

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD exit 0

LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"
LABEL org.opencontainers.image.description="CTO Parallel Test - Rust authentication library"
LABEL org.opencontainers.image.licenses="MIT"

CMD ["sh", "-c", "echo 'CTO Parallel Test library container ready'"]
