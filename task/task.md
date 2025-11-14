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

### Step 1: Add Authentication Dependencies
Update `Cargo.toml` with required authentication libraries (versions aligned with this repo):
```toml
[dependencies]
jsonwebtoken = "9.3.0"
argon2 = "0.5.0"
serde = { version = "1.0", features = ["derive"] }
rand_core = { version = "0.6", features = ["getrandom"] }
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
Create `src/auth/jwt.rs` for JWT operations with secure defaults:
```rust
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // Subject (user id)
    pub exp: u64,     // Expiration time (seconds since epoch)
    pub iat: u64,     // Issued at (seconds since epoch)
}

pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();
    let exp = now + 24 * 3600; // 24h

    let claims = Claims { sub: user_id.to_owned(), exp: exp, iat: now };

    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set and strong (>=32 chars)");

    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some("JWT".to_string());
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set and strong (>=32 chars)");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 30; // seconds of clock skew leeway

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)?;
    Ok(token_data.claims)
}
```
Security note: never hardcode or fallback to default secrets; fail fast if `JWT_SECRET` is not set.

### Step 4: Implement User Model with Password Hashing
Create `src/auth/models.rs` for user authentication logic with Argon2id and secure defaults:
```rust
use serde::{Deserialize, Serialize};
use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
             Argon2, Algorithm, Params, Version};
use rand_core::OsRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    pub fn verify_password(&self, password: &str) -> bool {
        match PasswordHash::new(&self.password_hash) {
            Ok(parsed) => Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok(),
            Err(_) => false,
        }
    }

    /// Hash a password using Argon2id v0x13 with t=3, m=64 MiB, p=1 and random salt
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(65_536, 3, 1, None).expect("invalid Argon2 params");
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        argon2.hash_password(password.as_bytes(), &salt).expect("hash failed").to_string()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}
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

// Example placeholder token for documentation; not a real secret.
let token = "<example-jwt-token>";
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
