# Multi-stage Dockerfile for Rust binary deployment
# Note: This project is currently a library crate. Update when binary target is added.

FROM rust:1.91-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifest
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary (when binary target exists)
# RUN cargo build --release --bin <binary-name>

# Runtime stage - optimized for deployment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy binary from builder (when binary target exists)
# COPY --from=builder /build/target/release/<binary-name> /app/<binary-name>
# RUN chmod +x /app/<binary-name> && chown -R app:app /app

USER app

# Health check (update when binary target exists)
# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#     CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Default command (update when binary target exists)
# CMD ["/app/<binary-name>"]
