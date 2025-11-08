FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libpq5 wget --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy the binary from the build stage (expects pre-built binary)
COPY cto-parallel-test /app/cto-parallel-test

# Set ownership and permissions
RUN chmod +x /app/cto-parallel-test && chown -R app:app /app

USER app

EXPOSE 8080

# Health check (placeholder - adjust if you add a health endpoint)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["./cto-parallel-test"]
