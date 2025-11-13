# Authentication Module

This service provides a production-grade, stateless authentication module with:

- JWT creation and validation using HS256
- 24h expiration by default (configurable via env)
- Argon2id password hashing with per-password random salt
- User model and request/response DTOs

## Security Design

- No hardcoded secrets. `JWT_SECRET` must be provided and must be at least 32 bytes.
- Token TTL is configurable via `JWT_EXP_SECONDS`; default is 86,400 seconds.
- Claims include `sub`, `iat`, and `exp`. Validation restricts the algorithm to HS256.
- Password hashes use Argon2id with OS CSPRNG salt; the `password_hash` field is excluded from serialization.

## Configuration

Environment variables:

- `JWT_SECRET` (required): HMAC secret for signing tokens.
- `JWT_EXP_SECONDS` (optional): Token lifetime in seconds. Default: `86400`.

## Public API

- `cto_parallel_test::auth::create_token(user_id: &str) -> Result<String, AuthError>`
- `cto_parallel_test::auth::validate_token(token: &str) -> Result<Claims, AuthError>`
- `cto_parallel_test::auth::User` with `hash_password` and `verify_password` helpers.

See `src/auth/tests.rs` for usage examples and end-to-end tests.

## Notes

- Wall-clock time is abstracted behind a `Clock` trait for testability and to comply with project coding guidelines.
- Clippy pedantic and formatting checks pass; tests cover hashing and JWT flows.

