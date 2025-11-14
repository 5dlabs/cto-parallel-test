# Security Notes

This service ships with hardened authentication primitives. Operators should review and configure the following environment variables and defaults.

- JWT secret: set `JWT_SECRET` to a high-entropy value (minimum enforced length 32 characters). Optionally set `JWT_SECRET_MIN_LEN` (>= 32) to raise the floor.
- Token TTL: `JWT_TTL_SECS` controls token lifetime. Values are clamped to the range (0, 7 days]; default is 24 hours.
- Algorithm: JWTs are signed and validated with HS256 only; algorithm is pinned to prevent confusion attacks.
- Issuer/Audience (optional):
  - `JWT_ISSUER` and `JWT_AUDIENCE` can be set to embed and validate `iss` and `aud` claims.
  - If configured, validation requires an exact match.
- Password hashing: Argon2 with a cryptographically secure random salt (OsRng). Verification safely handles malformed hashes.

Operational recommendations

- Store secrets in a secret manager or environment management system; never commit to source control.
- Rotate `JWT_SECRET` periodically and on any suspicion of compromise.
- CI security scans:
  - CodeQL static analysis (GitHub code scanning)
  - `cargo audit` for vulnerable dependencies
  - `gitleaks` for secret scanning (tree) to prevent new leaks
- Quality gates in CI: `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test`.

Local verification

- Format: `cargo fmt --all --check`
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- Test: `cargo test --workspace --all-features -q`
- Build (reproducible): `cargo build --workspace --all-features --locked`
- Dependencies: `cargo install cargo-audit --locked && cargo audit --deny warnings`
- Secrets: `gitleaks detect --no-git --source . --redact`
