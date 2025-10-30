# Build stage
FROM rust:1.83-slim as builder

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

# Runtime stage - minimal image with build artifacts
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled library artifacts
COPY --from=builder /app/target/release/*.rlib /app/lib/ || true
COPY --from=builder /app/target/release/deps /app/deps/ || true

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# For a library crate, this container serves as a verified build artifact
CMD ["echo", "cto-parallel-test library build completed successfully"]
