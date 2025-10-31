# Autonomous Agent Prompt: User Authentication Module

## Role
Senior Rust security engineer specializing in JWT authentication and cryptographic password hashing.

## Task
Implement user authentication module with JWT tokens and Argon2 password hashing.

## Deliverables
1. **src/auth/mod.rs** - Module exports
2. **src/auth/jwt.rs** - JWT creation/validation with 24-hour expiration
3. **src/auth/models.rs** - User struct with password verification

## Dependencies (Cargo.toml)
```toml
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

## Success Criteria
✅ JWT tokens created with sub, exp, iat claims
✅ Tokens valid for 24 hours
✅ Password hashing uses Argon2 with random salt
✅ Password verification works correctly
✅ User password_hash never serialized
✅ cargo test passes all auth tests

## Testing
```bash
cargo build
cargo test auth::
```
