Database Setup and Configuration

- Requires PostgreSQL 16+ accessible at the `DATABASE_URL`.
- Migrations live under `migrations/` and are applied with Diesel CLI.

Environment variables
- `DATABASE_URL` (required): postgres connection string.
- `DB_POOL_MAX_SIZE` (optional, default 15): r2d2 max pool size.
- `DB_POOL_MIN_IDLE` (optional): r2d2 min idle connections.
- `DB_POOL_CONNECTION_TIMEOUT_SECS` (optional, default 30): wait time when checking out a connection.
- `DB_POOL_MAX_LIFETIME_SECS` (optional): maximum time a connection is kept open before recycling.
- `DB_POOL_IDLE_TIMEOUT_SECS` (optional): idle timeout before a connection is dropped.
- `DB_POOL_TEST_ON_CHECK_OUT` (optional): set to `true`/`false` to enable r2d2 connection validation on checkout.

Local development
- Start Postgres (example via Docker):
  `docker run -d --name cto_pg -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=<your-password> -e POSTGRES_DB=ecommerce_db -p 5432:5432 postgres:16-alpine`
- Copy `.env.example` to `.env` and set a real password.
  Example: `DATABASE_URL=postgres://postgres:<your-password>@localhost:5432/ecommerce_db`.
- Install Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`.
- Apply migrations: `diesel migration run`.
- Regenerate schema (if needed): `diesel print-schema > src/schema.rs`.

Security
- No secrets are committed; `.env` is gitignored. Use `.env.example` as a template.
- Diesel uses parameterized queries by default for SQL safety.
- Password hashes are excluded from serialization.
