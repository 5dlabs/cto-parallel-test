# Build and test stage for library crate
FROM rust:alpine AS builder

WORKDIR /app

# Install build dependencies and Rust components
RUN apk add --no-cache musl-dev openssl-dev && \
    rustup component add rustfmt clippy

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source tree
COPY src ./src
COPY clippy.toml ./

# Run quality checks
RUN cargo fmt --all -- --check
RUN cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
RUN cargo test --workspace --all-features
RUN cargo build --release --all-features

# Final minimal image with just the library artifacts
FROM alpine:latest

WORKDIR /app

# Copy build artifacts (library files)
COPY --from=builder /app/target/release/libcto_parallel_test.rlib /app/lib/
COPY --from=builder /app/Cargo.toml /app/

# Add metadata
LABEL org.opencontainers.image.description="CTO Parallel Test - Rust Authentication Library"
LABEL org.opencontainers.image.source="https://github.com/5dlabs/cto-parallel-test"

# Default command shows library info
CMD ["sh", "-c", "echo 'CTO Parallel Test Library - All quality checks passed' && ls -lh /app/lib/"]
