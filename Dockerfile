# Multi-stage build for efficient Docker image
# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY clippy.toml ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release --all-features

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/cto-parallel-test /usr/local/bin/cto-parallel-test

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /usr/local/bin/cto-parallel-test

USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "/usr/local/bin/cto-parallel-test", "--version" ] || exit 1

ENTRYPOINT ["/usr/local/bin/cto-parallel-test"]
