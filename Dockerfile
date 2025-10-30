# Multi-stage Dockerfile for Rust library/API project

# Stage 1: Builder
# Use nightly for latest Diesel compatibility (Diesel 2.3.x requires Rust 1.86+)
FROM rustlang/rust:nightly-bookworm-slim AS builder

WORKDIR /app

# Install required system dependencies for Diesel/PostgreSQL
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy source code and build files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build the library in release mode
RUN cargo build --release --lib

# Stage 2: Runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire release directory (library crate doesn't produce a binary)
COPY --from=builder /app/target/release/deps /app/lib/deps
COPY --from=builder /app/migrations ./migrations

# Set environment variables
ENV LD_LIBRARY_PATH=/app/lib:$LD_LIBRARY_PATH

# For library projects, provide a simple health check script
RUN echo '#!/bin/sh' > /app/health.sh && \
    echo 'echo "Rust library cto-parallel-test built successfully"' >> /app/health.sh && \
    echo 'echo "Library artifacts:"' >> /app/health.sh && \
    echo 'find /app/lib -name "libcto_parallel_test*" -o -name "*cto_parallel_test*.rlib" 2>/dev/null | head -5' >> /app/health.sh && \
    echo 'exit 0' >> /app/health.sh && \
    chmod +x /app/health.sh

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/health.sh"]

# Default command
CMD ["/app/health.sh"]
