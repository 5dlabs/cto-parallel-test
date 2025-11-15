#!/usr/bin/env bash
set -euo pipefail

# Start a local PostgreSQL via Docker and run Diesel setup + migrations
# Parameters via env (secure defaults):
#   DB_USER (default: postgres)
#   DB_PASSWORD (default: postgres)
#   DB_NAME (default: ecommerce_db)
#   DB_PORT (default: 5432)
#   DB_CONTAINER_NAME (default: cto_pg)

DB_USER="${DB_USER:-postgres}"
DB_PASSWORD="${DB_PASSWORD:-postgres}"
DB_NAME="${DB_NAME:-ecommerce_db}"
DB_PORT="${DB_PORT:-5432}"
DB_CONTAINER_NAME="${DB_CONTAINER_NAME:-cto_pg}"

echo "[db] Ensuring postgres container '${DB_CONTAINER_NAME}' on port ${DB_PORT}"
if ! docker ps -a --format '{{.Names}}' | grep -q "^${DB_CONTAINER_NAME}$"; then
  docker run -d \
    --name "${DB_CONTAINER_NAME}" \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_NAME}" \
    -p "${DB_PORT}:5432" \
    postgres:16-alpine >/dev/null
else
  docker start "${DB_CONTAINER_NAME}" >/dev/null || true
fi

echo "[db] Waiting for postgres readiness"
for i in {1..60}; do
  if docker exec "${DB_CONTAINER_NAME}" pg_isready -U "${DB_USER}" >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
echo "[db] DATABASE_URL set to ${DATABASE_URL}"

if ! command -v diesel >/dev/null 2>&1; then
  echo "[db] Installing diesel_cli"
  cargo install diesel_cli --no-default-features --features postgres
fi

echo "[db] Running diesel setup + migration run"
diesel setup
diesel migration run

echo "[db] Done"

