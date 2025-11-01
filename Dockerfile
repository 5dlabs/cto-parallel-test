FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libpq5 wget --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy the pre-built binary
COPY ecommerce_api /app/ecommerce_api

RUN chmod +x /app/ecommerce_api && chown -R app:app /app

USER app

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["./ecommerce_api"]
