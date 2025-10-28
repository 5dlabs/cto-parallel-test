# Task 3: User Authentication Module

## Overview
Create user authentication and JWT (JSON Web Token) handling functionality for secure API access. This is a Level 0 task with no dependencies that implements authentication infrastructure for the e-commerce application.

## Context
This task establishes the security foundation for the application by implementing JWT-based authentication and password hashing. It runs in parallel with other Level 0 tasks (Tasks 1, 4, 6) and provides authentication capabilities that Task 5 (Shopping Cart API) will depend on.

## Objectives
1. Implement JWT token creation and validation in `src/auth/jwt.rs`
2. Create User model with password hashing in `src/auth/models.rs`
3. Set up auth module exports in `src/auth/mod.rs`
4. Add authentication dependencies (jsonwebtoken, argon2, rand) to `Cargo.toml`

## Dependencies
**None** - This is a Level 0 task that can run independently in parallel with Tasks 1, 4, and 6.

## Files to Create/Modify

### 1. `src/auth/mod.rs`
Module declaration to export authentication components:

```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token};
pub use self::models::User;
```

**Design Pattern**: Re-export commonly used types for ergonomic imports.

### 2. `src/auth/jwt.rs`
JWT token creation and validation:

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user id)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
    };

    // In a real app, this would be a proper secret key from environment
    let secret = b"test_secret_key";
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}
```

**Security Considerations**:
- Tokens expire after 24 hours
- Uses HS256 algorithm (default)
- Test secret key hardcoded (production would use environment variable)
- Claims include subject (user ID), expiration, and issued-at timestamps

### 3. `src/auth/models.rs`
User model with password hashing:

```rust
use serde::{Serialize, Deserialize};
use argon2::{self, Config};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        argon2::verify_encoded(&self.password_hash, password.as_bytes()).unwrap_or(false)
    }

    pub fn hash_password(password: &str) -> String {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }
}
```

**Security Features**:
- **Argon2**: Memory-hard password hashing algorithm (OWASP recommended)
- **Random salt**: 32-byte salt generated per password
- **Password hash never serialized**: `#[serde(skip_serializing)]` prevents leakage
- **Constant-time comparison**: argon2::verify_encoded resistant to timing attacks

### 4. `Cargo.toml` Updates
Add authentication and cryptography dependencies:

```toml
[dependencies]
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

**Dependency Rationale**:
- **jsonwebtoken**: Industry-standard JWT implementation
- **argon2**: Winner of Password Hashing Competition, memory-hard algorithm
- **rand**: Cryptographically secure random number generation

## Implementation Steps

1. **Create Auth Module Structure**
   - Create `src/auth/` directory
   - Add `mod.rs` with module exports
   - Set up public API for authentication

2. **Implement JWT Handling**
   - Create `jwt.rs` with Claims structure
   - Implement token creation with expiration
   - Implement token validation with error handling
   - Use jsonwebtoken crate for standardized JWT

3. **Implement User Model**
   - Create `models.rs` with User struct
   - Implement password hashing using Argon2
   - Implement password verification
   - Ensure password hash never serializes to JSON

4. **Add Dependencies**
   - Update Cargo.toml with crypto dependencies
   - Specify exact versions for security predictability
   - Include serde integration for JWT claims

5. **Validation**
   - Run `cargo check` to verify syntax
   - Ensure proper error handling in JWT functions
   - Validate password hashing produces different outputs for same input (salt verification)

## Technical Considerations

### JWT Token Structure
```
Header: { "alg": "HS256", "typ": "JWT" }
Payload: { "sub": "user_id", "exp": 1234567890, "iat": 1234567890 }
Signature: HMACSHA256(base64UrlEncode(header) + "." + base64UrlEncode(payload), secret)
```

### Password Hashing Flow
```
Password → Random Salt → Argon2 Config → Hash → Encoded String
"password123" → [32 random bytes] → default config → hash → "$argon2i$v=19$m=4096,t=3,p=1$..."
```

### Security Best Practices Implemented
- ✅ JWT expiration (24 hours)
- ✅ Argon2 memory-hard hashing
- ✅ Random salt per password
- ✅ Password hash exclusion from serialization
- ✅ Proper error handling in validation
- ✅ Constant-time password comparison

### Security Limitations (Test Project)
- ⚠️ Hardcoded secret key (production should use env var)
- ⚠️ No token refresh mechanism
- ⚠️ No token revocation
- ⚠️ Basic claims structure
- ⚠️ No rate limiting

## Integration Points

- **Task 5 (Shopping Cart API)**: Will use JWT validation for authentication
- **Task 7 (Integration Tests)**: Will test token creation and validation
- **Future API Endpoints**: Will use User model for authentication

## Risks and Mitigation

**Risk**: Hardcoded secret key in production
- **Mitigation**: Documentation clearly marks as test code
- **Production Fix**: Use environment variable with proper key management

**Risk**: Password hashing performance impact
- **Mitigation**: Argon2 default config balances security and performance
- **Tuning**: Config can be adjusted based on hardware capabilities

**Risk**: JWT without refresh tokens
- **Mitigation**: Acceptable for test project
- **Production**: Implement refresh token mechanism

## Success Criteria

1. ✅ `src/auth/mod.rs` exists with proper exports
2. ✅ `src/auth/jwt.rs` implements token creation and validation
3. ✅ `src/auth/models.rs` implements User with password hashing
4. ✅ JWT tokens contain sub, exp, and iat claims
5. ✅ Password hashing uses Argon2 with random salts
6. ✅ Password hash never serialized to JSON
7. ✅ `Cargo.toml` includes jsonwebtoken, argon2, and rand dependencies
8. ✅ Code compiles with `cargo check`
9. ✅ Token validation correctly decodes valid tokens
10. ✅ Password verification correctly validates passwords

## Estimated Effort
**45 minutes** - Cryptography setup, JWT implementation, and password hashing

## Additional Resources

### JWT Standard
- RFC 7519: JSON Web Token specification
- Uses compact, URL-safe format
- Self-contained with all necessary information

### Argon2 Algorithm
- Winner of Password Hashing Competition (2015)
- Memory-hard to resist GPU/ASIC attacks
- Three variants: Argon2i, Argon2d, Argon2id
- This implementation uses Argon2i (side-channel resistant)

### Testing Suggestions
```rust
// Example test for password hashing
#[test]
fn test_password_hashing() {
    let password = "test_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Different salts produce different hashes
    assert_ne!(hash1, hash2);

    // Both hashes verify successfully
    assert!(argon2::verify_encoded(&hash1, password.as_bytes()).unwrap());
    assert!(argon2::verify_encoded(&hash2, password.as_bytes()).unwrap());
}
```
