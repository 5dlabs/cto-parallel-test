Security Posture and Decisions

- Diesel ORM uses prepared statements by default, preventing SQL injection.
- No hardcoded secrets are present; configuration uses environment variables loaded via `dotenvy` in development only.
- Database URL is read from `DATABASE_URL`; `.env` is gitignored. Use `.env.example` as a template.
- `password_hash` is stored as an opaque string and is excluded from serialization via `#[serde(skip_serializing)]` (src/models.rs:12).
- Monetary values use PostgreSQL `NUMERIC` mapped to `bigdecimal::BigDecimal` (src/schema.rs:17, src/models.rs:20) to avoid precision loss.
- Connection pooling is provided by `r2d2`; pool limits and timeouts are configurable via env vars with secure defaults (src/config/db.rs:1).
- CI integrates CodeQL and cargo-audit for continuous code scanning (.github/workflows/security.yml:1).

Operational Notes

- To run migrations, install Diesel CLI and set a real `DATABASE_URL`.
- Code avoids panics in runtime paths; initialization uses `.expect()` with clear messages for early failure.
