FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 wget --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* && apt-get clean
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app
WORKDIR /app
# Note: This is a library crate, so binary would be provided by the application that uses it
# For now, we create a placeholder structure
RUN echo "This is a library crate for the e-commerce catalog module" > /app/README.txt
USER app
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD echo "Library crate - no health endpoint" || exit 0
CMD ["tail", "-f", "/dev/null"]
