# Multi-stage build for Rust application
# Stage 1: Build the application
FROM rust:1.90-slim-bookworm AS builder

# Install required system dependencies for Diesel and PostgreSQL
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application in release mode
RUN cargo build --release --all-features

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Create app directory
WORKDIR /app

# Copy the built library artifacts from the builder stage
# Since this is currently a library crate, copy all artifacts
COPY --from=builder /app/target/release/ /app/lib/

# Copy migrations for runtime use
COPY --from=builder /app/migrations /app/migrations

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port (adjust as needed when API server is added)
EXPOSE 8080

# Health check (to be updated when API endpoints are available)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD echo "Health check placeholder - update when API is available" || exit 1

# Default command (will be updated when binary target is added)
CMD ["echo", "Library built successfully. Add binary target for executable."]
