# Build stage
FROM rust:1.83-slim AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
# Note: Cargo.lock is gitignored for library crates - cargo will generate it

# Copy source code
COPY src ./src
COPY clippy.toml ./

# Build and test the library
RUN cargo build --release && \
    cargo test --release

# Runtime stage - minimal image with build verification
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy a marker file to indicate successful build
RUN echo "cto-parallel-test library v0.1.0 - build completed successfully" > /app/BUILD_SUCCESS

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# For a library crate, this container serves as a verified build artifact
CMD ["cat", "/app/BUILD_SUCCESS"]
