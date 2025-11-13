# Authentication Module

This service provides a production‑grade authentication foundation with:
- JWT creation and validation (configurable TTL; default 24h)
- Argon2id password hashing with cryptographically secure random salt
- User model and request/response DTOs

## Environment Variables
- `JWT_SECRET` (required): HMAC secret for signing/verifying JWTs. Must be a strong, random value of at least 32 bytes. Prefer a 256-bit (32-byte) or longer secret generated from a CSPRNG.
- `JWT_EXP_SECONDS` (optional): Token lifetime in seconds. Defaults to `86400` (24 hours).

## Public API
- `cto_parallel_test::auth::jwt::create_token(user_id: &str) -> Result<String, AuthError>`
- `cto_parallel_test::auth::jwt::validate_token(token: &str) -> Result<Claims, AuthError>`
- `cto_parallel_test::auth::models::User` with:
  - `User::hash_password(password: &str) -> Result<String, String>`
  - `User::verify_password(&self, password: &str) -> bool`

## Security Notes
- No hardcoded secrets: tokens require `JWT_SECRET`.
- Expiration validated by default. Tokens failing validation (expired/tampered) are rejected.
- Password hash is excluded from JSON via `#[serde(skip_serializing)]`.
- Uses `Clock` abstraction internally for time; default clock bridges to system time.
 - JWT algorithm explicitly restricted to HS256 for both signing and validation to prevent algorithm confusion.

## Quick Start (local)
```bash
export JWT_SECRET="$(openssl rand -base64 48)"  # >=32 bytes
# Optional TTL override
# export JWT_EXP_SECONDS=86400

cargo test --workspace --all-features
```

## Integration
This module is stateless and does not perform any DB access. Use it from your routes/handlers to implement `/login` and `/register` endpoints. Store only `password_hash`, never plaintext passwords.

## Hardening
- Rotate `JWT_SECRET` on a schedule and enforce short lived tokens where possible.
- Enforce `JWT_SECRET` length ≥ 32 bytes (already enforced at runtime) and store outside code (env/secret manager).
- Consider key identifiers (kid) and rotation strategy for future tasks.
- Consider `aud`/`iss` claims if multi‑tenant requirements arise.
