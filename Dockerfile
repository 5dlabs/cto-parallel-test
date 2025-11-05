FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libpq5 wget --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy pre-built binary from build context
COPY ecommerce_api /app/ecommerce_api

# Set permissions
RUN chmod +x /app/ecommerce_api && chown -R app:app /app

# Switch to non-root user
USER app

# Expose application port
EXPOSE 8080

# Health check (will be updated when API endpoints are implemented)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Run the binary
CMD ["./ecommerce_api"]
