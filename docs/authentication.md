# Authentication Module

This module provides production-grade primitives for user authentication:

- JWT token creation and validation
- Argon2 password hashing with random salt
- User model and DTOs for request/response payloads

## Environment Variables

- `JWT_SECRET` (required): Secret key used to sign and validate JWTs. Must be at least 32 bytes. Do not commit real secrets to source control.
- `JWT_EXP_HOURS` (optional): Token lifetime in hours. Defaults to `24`.
- `JWT_LEEWAY_SECS` (optional): Leeway in seconds for token validation to tolerate minor clock skew (default `60`, maximum `300`).

## Usage

```rust
use cto_parallel_test::auth::{create_token, validate_token, User};

// Ensure a strong secret is set at runtime
std::env::set_var("JWT_SECRET", "your_random_secret_at_least_32_bytes");

// Hash passwords for storage
let hashed = User::hash_password("correct horse battery staple").unwrap();
let user = User { id: 1, username: "alice".into(), email: "alice@example.com".into(), password_hash: hashed };
assert!(user.verify_password("correct horse battery staple"));

// Issue and validate a token
let token = create_token("1").unwrap();
let claims = validate_token(&token).unwrap();
assert_eq!(claims.sub, "1");
```

## Security Notes

- No hardcoded secrets: `JWT_SECRET` is required in all environments.
- Tokens include standard claims: `sub`, `iat`, `exp`. Expiration defaults to 24 hours.
- Password hashes never serialize to JSON (guarded by `#[serde(skip_serializing)]`).
- Argon2 is intentionally slow with unique salts for each password.

See also: `docs/SECURITY.md` and the repository coding and GitHub guidelines.

