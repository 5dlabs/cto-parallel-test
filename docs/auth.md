Authentication Module

- JWT: 24-hour tokens with claims `sub`, `exp`, `iat`.
- Hashing: Argon2 with random salt, `#[serde(skip_serializing)]` on `password_hash`.

Environment
- Set `JWT_SECRET` to a strong, random key (≥32 chars).
- There is no fallback in code; missing secret causes token ops to fail.

Usage
- Create token: `create_token(user_id)`
- Validate token: `validate_token(token)`
- Hash password: `User::hash_password(password)`
- Verify password: `user.verify_password(password)`

Testing
- Tests and doc examples set `JWT_SECRET` explicitly.
