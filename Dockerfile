# Multi-stage build for optimized Docker image

# Stage 1: Build the application
FROM rust:1.86-bookworm AS builder

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy over manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml
COPY clippy.toml ./clippy.toml

# Build for release with all optimizations
RUN cargo build --release --all-features

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies (PostgreSQL client libraries)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/libcto_parallel_test.* /app/

# Copy migrations for diesel
COPY migrations /app/migrations
COPY diesel.toml /app/diesel.toml

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

USER appuser

# Set environment variable for database URL (override at runtime)
ENV DATABASE_URL=""

# Health check command (can be customized based on actual service)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["true"]

# Default command (this is a library, so just verify it exists)
CMD ["ls", "-la", "/app/"]
