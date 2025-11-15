# Authentication Module

This crate provides a foundational, secure authentication module:

- JWT token creation and validation (24h expiry)
- Argon2 password hashing with random salt
- `User` model with password verification
- Request/Response DTOs for auth endpoints

## Configuration

- `JWT_SECRET` (required in production): Secret key used to sign and validate JWTs.
  - Must be a strong, random value (≥32 chars by default; see `JWT_SECRET_MIN_LEN`).
  - Example:
    ```bash
    export JWT_SECRET="your_secure_random_secret_key_minimum_32_characters"
    ```
  - Development-only fallback: in debug builds only, a cryptographically random key is
    generated once per-process and cached. This is provided strictly for local development
    convenience; always set `JWT_SECRET` in CI and production.
- `JWT_TTL_SECS` (optional): Token lifetime in seconds. Defaults to `86400` (24 hours).
  - Example: `export JWT_TTL_SECS=7200`  # 2 hours
- `JWT_SECRET_MIN_LEN` (optional): Minimum secret length to enforce; cannot be lower than 32.
  - Example: `export JWT_SECRET_MIN_LEN=48`
 - Argon2 tuning (optional, bounded for safety):
   - `ARGON2_M_COST_KIB` (memory in KiB). Default `65536` (64 MiB). Bounds: 8192–1048576.
   - `ARGON2_T_COST` (iterations). Default `3`. Bounds: 1–10.
   - `ARGON2_P_COST` (parallelism). Default `1`. Bounds: 1–8.

## Usage

```rust
use cto_parallel_test::auth::{create_token, validate_token, User};

// Hash a password when creating a user
let password_hash = User::hash_password("super_secret_password");

// Verify a login attempt
let user = User {
    id: 1,
    username: "alice".into(),
    email: "a@example.com".into(),
    password_hash,
};
assert!(user.verify_password("super_secret_password"));

// Create and validate a token (JWT_SECRET is required at runtime)
std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");
let token = create_token("1").expect("token creation");
let claims = validate_token(&token).expect("token validation");
assert_eq!(claims.sub, "1");
```

## Security Notes

- No hardcoded secrets. Tokens require `JWT_SECRET` at runtime (missing or weak keys are rejected).
- Password hashes are never serialized and never accepted from input (`#[serde(skip_serializing, skip_deserializing)]`).
- `Debug` for `User` redacts the `password_hash` field to avoid leaking sensitive material in logs.
- All inbound auth DTOs (`LoginRequest`, `RegisterRequest`, and `User` if ever deserialized) use `#[serde(deny_unknown_fields)]` to prevent mass-assignment or silent acceptance of unexpected fields.
- Argon2 defaults are used; consider tuning parameters per deployment (increase memory cost and iterations where feasible).
- JWT is strictly validated with `HS256` only to avoid algorithm confusion attacks; 30s leeway is allowed for clock skew.
- A minimum secret length of 32 bytes is enforced for HMAC keys to reduce risk of weak keys.
- Token lifetime (`JWT_TTL_SECS`) defaults to 24h and is capped at 24 hours to limit exposure of leaked tokens.
- If `JWT_ISSUER` and/or `JWT_AUDIENCE` are set, tokens include `iss`/`aud` claims and validation enforces matching issuer/audience.

## Cryptography Backend

- The crate uses `jsonwebtoken` 10.x with the `aws_lc_rs` backend enabled and default features disabled. This configuration avoids `ring` 0.16.x and the `rsa` crate entirely to eliminate known advisories surfaced by `cargo audit`.
- Only HMAC-SHA2 (HS256) is compiled and used by this module. RSA/ECDSA algorithms are not required and not included in the build.

## Testing

Run the standard quality gates before PR:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

## GitHub Code Scanning

When a pull request is open, fetch open security alerts for the PR using GitHub CLI:

- Create PR from `feature/task-3-implementation` to `main`:
  `gh pr create --title "Task 3: Secure Auth Module (JWT + Argon2)" --body-file PR_BODY.md --base main --head feature/task-3-implementation --label task-3 --label service-cto-parallel-test --label run-play-task-3-hfxlh`
- List alerts for the PR (replace `<PR_NUMBER>`):
  `gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&per_page=100&pr=<PR_NUMBER>" -H "Accept: application/sarif+json, application/json" | jq .`
