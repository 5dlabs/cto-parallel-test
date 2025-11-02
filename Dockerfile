# Multi-stage build for Rust library project
# Stage 1: Build the library
FROM rust:1.75-bookworm as builder

WORKDIR /build

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build the library in release mode
RUN cargo build --release --lib

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    wget \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy the built library from builder stage
COPY --from=builder /build/target/release/libcto_parallel_test.* /app/lib/
COPY --from=builder /build/migrations /app/migrations

# Set ownership
RUN chown -R app:app /app

USER app

# Health check endpoint (will be used by future API server)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Note: This is a library crate, so no CMD/ENTRYPOINT yet
# Binary targets will be added in future tasks (Task 2: API Endpoints)
CMD ["echo", "Library container - waiting for API server implementation"]
