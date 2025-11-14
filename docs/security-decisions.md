Security Decisions and Practices

- Uses Diesel ORM which issues parameterized queries by default to mitigate SQL injection.
- No hardcoded secrets. Runtime configuration is read from environment (`DATABASE_URL`, pool tuning vars). `.env` is not tracked and is gitignored; `.env.example` with redacted placeholders is provided.
- `.env.example` template uses `REDACTED` in the connection string to avoid secret-like patterns while still documenting expected format.
- Password hashes are present in the database model but excluded from serialization and deserialization using `#[serde(skip_serializing, skip_deserializing)]`.
- Insertable models that accept user input (e.g., `NewUser`, `NewProduct`) do not derive `Deserialize`; requests map to explicit DTOs and perform validation and password hashing (where applicable) before DB insert to prevent mass-assignment.
- Monetary values use PostgreSQL `NUMERIC` mapped to `bigdecimal::BigDecimal` for precision safety.
- Foreign keys enforce referential integrity with `ON DELETE CASCADE` where appropriate.
- Connection pooling (r2d2) is parameterized via env with secure defaults (timeouts, sizes) and optional `test_on_check_out`.
- CI includes CodeQL and `cargo-audit` security scans in `.github/workflows/security.yml`.
- Crate forbids `unsafe` Rust at the root (`#![forbid(unsafe_code)]`) to eliminate classes of memory-unsafe issues.
- CI additionally runs Gitleaks secrets scanning to prevent credential leaks.
- Pre-PR quality gates enforced locally: fmt, clippy (pedantic, deny warnings), and tests.

Manual Scanning During PR Review
- Query open alerts for the current PR via GitHub CLI:
  `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>"`

References
- See `coding-guidelines.md` and `github-guidelines.md` for required gates and PR workflow.

Local Scan Results (this run)
- Timestamp (UTC): 2025-11-14T15:36:22Z
- fmt/clippy/tests: all passing
- cargo audit: no advisories found (`vulnerabilities.found=false`)
- gitleaks: no leaks found (`[]`)
