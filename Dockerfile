# Multi-stage build for efficient Rust Docker image
# Stage 1: Build the application
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release --all-features

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/libcto_parallel_test.* /app/

# Create a non-root user for security
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose default port (adjust as needed for future services)
EXPOSE 8080

# Health check (placeholder - adjust when HTTP service is added)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD test -f /app/libcto_parallel_test.rlib || exit 1

# Default command (placeholder - library crate doesn't have a binary)
CMD ["sh", "-c", "echo 'Library crate successfully built. Add a binary target to run a service.'"]
