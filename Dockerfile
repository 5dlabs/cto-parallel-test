# Rust Dockerfile - Multi-stage build for optimal image size
# Stage 1: Builder - Build the Rust project
FROM rust:1.83-bookworm AS builder

WORKDIR /build

# Copy dependency manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies (cache optimization)
RUN mkdir -p src && \
    echo "pub mod catalog;" > src/lib.rs && \
    mkdir -p src/catalog && \
    echo "// dummy" > src/catalog/mod.rs && \
    echo "// dummy" > src/catalog/models.rs && \
    echo "// dummy" > src/catalog/service.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual project
# Touch lib.rs to ensure rebuild
RUN touch src/lib.rs && \
    cargo build --release

# Run tests during build to ensure correctness
RUN cargo test --release

# Stage 2: Runtime - Minimal image with just the artifacts
FROM debian:bookworm-slim

# Install runtime dependencies (CA certificates for HTTPS)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser

WORKDIR /app

# For a library crate, there are no binaries to run
# Copy metadata to show build succeeded
COPY --from=builder /build/Cargo.toml /app/Cargo.toml

# Set ownership to non-root user
RUN chown -R appuser:appuser /app

USER appuser

# Health check placeholder (customize if you add a web service)
# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#   CMD curl -f http://localhost:8080/health || exit 1

# Default command (customize based on your binary)
# For now, just show that the build succeeded
CMD ["echo", "cto-parallel-test library built successfully"]

# Labels for metadata
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"
LABEL org.opencontainers.image.description="Product Catalog Module - E-commerce Library"
LABEL org.opencontainers.image.licenses="MIT"
