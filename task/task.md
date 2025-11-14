# Task 3: User Authentication Module

## Overview
Implement a secure user authentication system with JWT token handling and Argon2 password hashing for the e-commerce Rust API.

## Context
This is a **Level 0 task** (no dependencies) that provides:
- JWT-based stateless authentication
- Secure password hashing using Argon2
- Token creation and validation
- User model with password verification

This module is independent and can be developed in parallel with Tasks 1, 4, and 6.

## Objectives
1. Implement JWT token creation and validation
2. Set up Argon2 password hashing
3. Create user models with password verification
4. Establish authentication middleware foundation
5. Configure secure token management

## Dependencies
**None** - This is a foundational task that can run in parallel with Tasks 1, 4, and 6.

## Architecture Context
Refer to `.taskmaster/docs/architecture.md` sections:
- **User Authentication Module** (lines 203-230): JWT and password handling
- **Authentication Flow** (lines 398-440): Token-based auth sequence
- **Security Considerations** (lines 514-532): Best practices

## Implementation Plan

Security note: The secure implementation already exists in `src/auth/` and follows best practices (Argon2id v0x13 with strong params, HS256 with explicit header, env-only secrets with minimum length, TTL clamped via `JWT_TTL_SECS`). The steps below reference those modules rather than duplicating insecure samples.

### Step 1: Add Authentication Dependencies
Update `Cargo.toml` with required authentication libraries (already present and pinned):
```toml
[dependencies]
jsonwebtoken = "9.3.0"      # HS256, explicit header, issuer/audience support
argon2 = "0.5.0"            # Argon2id v0x13, secure params
serde = { version = "1.0", features = ["derive"] }
rand_core = { version = "0.6", features = ["getrandom"] } # CSPRNG for salts
```

**Validation:** Run `cargo check` to verify dependency resolution.

### Step 2: Create Authentication Module Structure
Create `src/auth/mod.rs` to export module components:
```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
```

### Step 3: Implement JWT Token Handling
Use the provided `src/auth/jwt.rs` which:
- Enforces `JWT_SECRET` from env with a minimum length (default ≥32, configurable via `JWT_SECRET_MIN_LEN` with a hard floor of 32)
- Clamps TTL via `JWT_TTL_SECS` with a safe maximum (7 days)
- Uses HS256 only with explicit `typ=JWT` header to prevent algorithm confusion
- Optionally validates `JWT_ISSUER` and `JWT_AUDIENCE` when set

Public API:
```rust
use cto_parallel_test::auth::{create_token, validate_token, Claims};
```

### Step 4: Implement User Model with Password Hashing
Use the provided `src/auth/models.rs` which:
- Uses Argon2id v0x13 with parameters t=3, m=64MiB, p=1 and CSPRNG (`OsRng`) salt
- Denies unknown fields on inbound DTOs to prevent mass assignment
- Never serializes or deserializes the password hash from untrusted input
- Redacts the password hash in `Debug` output

Public API:
```rust
use cto_parallel_test::auth::User;
```

### Step 5: Register Authentication Module
Update `src/main.rs` or `src/lib.rs`:
```rust
pub mod auth;
```

### Step 6: Add Environment Variable (Optional)
Update `.env` file with JWT secret:
```
JWT_SECRET=your_secure_random_secret_key_here
```

## Testing Strategy
1. **Unit Tests for JWT:**
   - Test token creation with valid user ID
   - Test token validation with valid token
   - Test token expiration handling
   - Test invalid token rejection

2. **Unit Tests for Password Hashing:**
   - Test password hashing generates unique hashes
   - Test password verification with correct password
   - Test password verification fails with wrong password
   - Test hash format is valid Argon2

3. **Integration Tests:**
   - Test complete auth flow (hash → verify → create token → validate token)
   - Test token expiration after 24 hours
   - Test multiple users don't interfere

## Security Considerations

### Password Security
- **Never store plaintext passwords** - Always use `hash_password()` before storage
- **Use random salt** - Argon2 generates unique salt for each password
- **Skip serialization** - `#[serde(skip_serializing)]` prevents password hash in JSON

### JWT Security
- **24-hour expiration** - Tokens expire after 1 day
- **Secure secret key** - Load from environment variable in production
- **Stateless tokens** - No server-side session storage required
- **Claims validation** - Always validate expiration and signature

### Best Practices
- Use HTTPS in production to protect tokens in transit
- Store tokens securely on client side (httpOnly cookies recommended)
- Implement token refresh mechanism for better UX
- Rate limit authentication endpoints to prevent brute force
- Log authentication attempts for security monitoring

## Risks and Considerations
- **Secret Key Management:** JWT_SECRET must be securely stored and rotated periodically
- **Token Expiration:** 24-hour expiration may need adjustment based on requirements
- **Password Complexity:** Consider enforcing password strength requirements
- **Timing Attacks:** Argon2 includes protection, but verify_password should use constant-time comparison
- **Concurrent Logins:** Stateless JWT allows multiple device logins

## Success Criteria
- [ ] All authentication dependencies added to `Cargo.toml`
- [ ] `src/auth/mod.rs` created with module exports
- [ ] `src/auth/jwt.rs` implements token creation and validation
- [ ] `src/auth/models.rs` implements User model with password methods
- [ ] JWT tokens expire after 24 hours
- [ ] Password hashing uses Argon2 with random salt
- [ ] Password hash never appears in serialized User
- [ ] `cargo check` passes without errors
- [ ] Unit tests verify JWT functionality
- [ ] Unit tests verify password hashing and verification
- [ ] Token validation rejects expired tokens
- [ ] Password verification fails for incorrect passwords

## Files Modified/Created
- `Cargo.toml` - Add authentication dependencies
- `src/auth/mod.rs` - Module exports
- `src/auth/jwt.rs` - JWT token handling
- `src/auth/models.rs` - User model and auth DTOs
- `.env` (optional) - JWT secret configuration

## Next Steps
After completion, this authentication module will be used by:
- **Task 5:** Shopping Cart API (requires JWT validation)
- **Task 7:** Integration Tests (tests auth flows)
- **Task 2:** API Endpoints (will add auth routes)

## Code Examples

### Creating a Token
```rust
use crate::auth::jwt::create_token;

let user_id = "123";
let token = create_token(user_id).expect("Failed to create token");
println!("JWT: {}", token);
```

### Validating a Token
```rust
use crate::auth::jwt::validate_token;

// Example token placeholder for docs; not a real JWT
let token = "example.jwt.token";
match validate_token(token) {
    Ok(claims) => println!("Valid token for user: {}", claims.sub),
    Err(e) => println!("Invalid token: {}", e),
}
```

### Hashing and Verifying Password
```rust
use crate::auth::models::User;

// Hash a password
let password = "secure_password_123";
let hash = User::hash_password(password);

// Create user with hash
let user = User {
    id: 1,
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    password_hash: hash,
};

// Verify password
assert!(user.verify_password(password));
assert!(!user.verify_password("wrong_password"));
```

## Performance Notes
- **Argon2 is intentionally slow** - This is a security feature to resist brute force attacks
- **Token validation is fast** - JWT validation is cryptographically efficient
- **No database queries needed** - Stateless authentication reduces server load
- **Consider caching** - For high-traffic apps, cache validated tokens briefly

## Compliance and Standards
- **Password Hashing:** Follows OWASP recommendations using Argon2
- **JWT Standard:** Implements RFC 7519 (JSON Web Tokens)
- **Token Claims:** Uses standard claims (sub, exp, iat)
- **Cryptographic Standards:** Uses industry-standard algorithms
