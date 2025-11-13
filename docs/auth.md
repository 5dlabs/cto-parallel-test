# Authentication Module

This crate provides a foundational, secure authentication module:

- JWT token creation and validation (24h expiry)
- Argon2 password hashing with random salt
- `User` model with password verification
- Request/Response DTOs for auth endpoints

## Configuration

- `JWT_SECRET` (required): Secret key used to sign and validate JWTs.
  - Must be a strong, random value (≥32 chars REQUIRED).
  - Example:
    ```bash
    export JWT_SECRET="your_secure_random_secret_key_minimum_32_characters"
    ```

## Usage

```rust
use cto_parallel_test::auth::{create_token, validate_token, User};

// Hash a password when creating a user
let password_hash = User::hash_password("super_secret_password");

// Verify a login attempt
let user = User { id: 1, username: "alice".into(), email: "a@example.com".into(), password_hash };
assert!(user.verify_password("super_secret_password"));

// Create and validate a token
std::env::set_var("JWT_SECRET", "dev_only_secret_min_32_chars________________");
let token = create_token("1").expect("token creation");
let claims = validate_token(&token).expect("token validation");
assert_eq!(claims.sub, "1");
```

## Security Notes

- No hardcoded secrets. Tokens require `JWT_SECRET` at runtime.
- Password hashes are never serialized (`#[serde(skip_serializing)]`).
- Argon2 defaults are used; consider tuning parameters per deployment.
- A `Clock` abstraction is used to bound wall-clock time usage.

## Testing

Run the standard quality gates before PR:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```
