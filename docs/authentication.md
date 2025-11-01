# Authentication Module

This module provides production-grade primitives for user authentication:

- JWT token creation and validation
- Argon2 password hashing with random salt
- User model and DTOs for request/response payloads

## Environment Variables

- `JWT_SECRET` (required): Secret key for signing/validating JWTs. Must be strong and kept out of source control.
- `JWT_EXP_HOURS` (optional): Token lifetime in hours. Defaults to `24`.
- `JWT_LEEWAY_SECS` (optional): Validation leeway in seconds to account for minor clock skew (defaults to `60`, clamped to a maximum of `300`).
- `JWT_ISSUER` (optional): Expected token issuer. When set, tokens are issued with and validated against this value.
- `JWT_AUDIENCE` (optional): Expected token audience. When set, tokens are issued with and validated against this value.

## Security Notes

- No hardcoded secrets: tokens require `JWT_SECRET` (min 32 bytes).
- Tokens include `sub`, `iat`, `nbf`, and `exp` claims. Validation enforces `nbf` and `exp` with a small leeway (default 60s, configurable up to 300s) to account for clock skew.
- Password hashes never serialize (`#[serde(skip_serializing)]`).
- Argon2 (v0.5) with unique random salt per password; verification uses constant-time checks.
- Safe expiration math: internally guards against integer overflow when computing `exp`.

## Usage

Add `pub mod auth;` to your crate and use:

```rust
use cto_parallel_test::auth::{create_token, validate_token, User};

let hashed = User::hash_password("password");
let user = User { id: 1, username: "u".into(), email: "e".into(), password_hash: hashed };
assert!(user.verify_password("password"));

std::env::set_var("JWT_SECRET", "a-very-long-strong-secret");
let token = create_token("1").unwrap();
let claims = validate_token(&token).unwrap();
assert_eq!(claims.sub, "1");
```

## Testing

Run the standard gates:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
- `cargo test --workspace --all-features`

Unit tests set a deterministic `JWT_SECRET` for reliability.
