# Authentication Module

This crate provides a foundational, secure authentication module:

- JWT token creation and validation (24h expiry)
- Argon2 password hashing with random salt
- `User` model with password verification
- Request/Response DTOs for auth endpoints

## Configuration

- `JWT_SECRET` (required): Secret key used to sign and validate JWTs.
  - Must be a strong, random value (≥32 chars by default; see `JWT_SECRET_MIN_LEN`).
  - Example:
    ```bash
    export JWT_SECRET="your_secure_random_secret_key_minimum_32_characters"
    ```
- `JWT_TTL_SECS` (optional): Token lifetime in seconds. Defaults to `86400` (24 hours).
  - Example: `export JWT_TTL_SECS=7200`  # 2 hours
- `JWT_SECRET_MIN_LEN` (optional): Minimum secret length to enforce; cannot be lower than 32.
  - Example: `export JWT_SECRET_MIN_LEN=48`

## Usage

```rust
use cto_parallel_test::auth::{create_token, validate_token, User};

// Hash a password when creating a user
let password_hash = User::hash_password("super_secret_password")
    .expect("failed to hash password");

// Verify a login attempt
let user = User {
    id: 1,
    username: "alice".into(),
    email: "a@example.com".into(),
    password_hash,
};
assert!(user.verify_password("super_secret_password"));

// Create and validate a token
// Example only: set a development secret via env. Do not hardcode secrets.
std::env::set_var("JWT_SECRET", "example-secret-min-32-characters-change-me-please");
let token = create_token("1").expect("token creation");
let claims = validate_token(&token).expect("token validation");
assert_eq!(claims.sub, "1");
```

## Security Notes

- No hardcoded secrets. Tokens require `JWT_SECRET` at runtime.
- Password hashes are never serialized (`#[serde(skip_serializing)]`).
- Password hashes are never serialized and never accepted from input (`#[serde(skip_serializing, skip_deserializing)]`).
- `Debug` for `User` redacts the `password_hash` field to avoid leaking sensitive material in logs.
- All inbound auth DTOs (`LoginRequest`, `RegisterRequest`, and `User` if ever deserialized) use `#[serde(deny_unknown_fields)]` to prevent mass-assignment or silent acceptance of unexpected fields.
- Argon2 is configured as Argon2id v0x13 with t=3, m=64 MiB, p=1. Tune upwards if feasible.
- JWT is strictly validated with `HS256` only to avoid algorithm confusion attacks; 30s leeway is allowed for clock skew.
- A minimum secret length of 32 bytes is enforced for HMAC keys to reduce risk of weak keys.
- A `Clock` abstraction is used to bound wall-clock time usage and enable deterministic tests.

## Testing

Run the standard quality gates before PR:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

## References

- See `coding-guidelines.md` for clippy, testing, and security patterns enforced project-wide (Clock abstraction, deny_unknown_fields, no hardcoded secrets).
- See `github-guidelines.md` for mandatory PR workflow, branch policy, and required pre-PR quality gates.
