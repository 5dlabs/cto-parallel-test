# Task 3: User Authentication Module

## Overview
Create user authentication and JWT (JSON Web Token) handling functionality for the Rust API project. This is a Level 0 task (no dependencies) that implements secure user authentication, password hashing, and token-based authorization.

## Context
This task provides the security foundation for the application. It enables user identification and authorization for protected resources like shopping carts. The authentication module will be used by Task 5 (Shopping Cart API) to validate user requests.

## Objectives
1. Implement JWT token creation and validation
2. Create user model with password hashing
3. Implement secure password verification
4. Provide reusable authentication utilities for other modules

## Dependencies
**None** - This is a Level 0 task that can run independently in parallel with Tasks 1, 4, and 6.

## Files to Create
- `src/auth/mod.rs` - Authentication module exports
- `src/auth/jwt.rs` - JWT token handling
- `src/auth/models.rs` - User model and password utilities
- `Cargo.toml` - Updates for authentication dependencies

## Technical Specifications

### Authentication Technology
- **JWT Library**: jsonwebtoken 8.3.0
- **Password Hashing**: argon2 0.5.0 (Argon2 algorithm, winner of Password Hashing Competition)
- **Random Generation**: rand 0.8.5
- **Token Lifetime**: 24 hours
- **Secret Key**: Hardcoded test key (would be environment variable in production)

### JWT Claims Structure
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: usize,   // Expiration time (Unix timestamp)
    pub iat: usize,   // Issued at (Unix timestamp)
}
```

### User Model
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]  // Never expose password hash
    pub password_hash: String,
}
```

## Implementation Plan

### Step 1: Update Cargo.toml
Add authentication dependencies:

```toml
[dependencies]
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

### Step 2: Create Module Exports (src/auth/mod.rs)
```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token};
pub use self::models::User;
```

**Design Note**: Re-export key functions and types at module level for cleaner imports by consumers.

### Step 3: Implement JWT Handling (src/auth/jwt.rs)
Key functions:

#### `create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error>`
- Generates expiration time (current time + 24 hours)
- Creates Claims with user_id as subject
- Encodes JWT with HMAC secret
- Returns signed token string

#### `validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error>`
- Decodes JWT with same HMAC secret
- Validates signature and expiration
- Returns claims if valid, error otherwise

**Security Notes**:
- Uses HS256 algorithm (HMAC-SHA256)
- Test secret key: `b"test_secret_key"` (production should use env var)
- Automatic expiration validation by jsonwebtoken library

### Step 4: Implement User Model (src/auth/models.rs)
Key methods:

#### `User::hash_password(password: &str) -> String`
- Generates random 32-byte salt
- Uses Argon2 with default configuration
- Returns encoded hash string
- Static method (no User instance required)

#### `User::verify_password(&self, password: &str) -> bool`
- Compares plain password against stored hash
- Uses Argon2 verification with timing attack resistance
- Returns true if match, false otherwise
- Safe to use in authentication checks

**Security Features**:
- Argon2 is memory-hard and resistant to GPU attacks
- Random salt prevents rainbow table attacks
- Password hash never serialized in JSON responses
- Verification has constant-time comparison internally

### Step 5: Integration Points
This module is imported by:
- Task 5 (Shopping Cart API) - for JWT validation in cart endpoints
- Task 7 (Integration Tests) - for testing authentication flow

## Architectural Considerations

### Password Security
- **Argon2**: Industry-standard algorithm, resistant to side-channel attacks
- **Random Salts**: Each password gets unique salt
- **No Plaintext Storage**: Only hashes are stored
- **Serde Skip**: `#[serde(skip_serializing)]` prevents hash exposure in API responses

### JWT Design
- **Stateless**: No server-side session storage required
- **Self-Contained**: Token carries user_id in claims
- **Time-Limited**: 24-hour expiration prevents long-term token reuse
- **Standard Format**: Compatible with any JWT library

### Module Organization
```
src/auth/
├── mod.rs       # Public interface, re-exports
├── jwt.rs       # Token operations
└── models.rs    # User type and password utilities
```

Clean separation of concerns:
- JWT logic isolated from user logic
- Password hashing separate from token handling
- Module exports only necessary public API

## Risks and Considerations

1. **Test Secret Key**: The hardcoded secret `b"test_secret_key"` is intentional for testing but MUST be replaced with an environment variable in production.

2. **No Database Integration**: This module provides models and utilities but doesn't persist users. Task 1's schema defines the users table, but actual database operations would be in a separate repository layer.

3. **Minimal Error Handling**: Returns library errors directly. Production code should wrap these in application-specific error types.

4. **No Token Refresh**: Tokens expire after 24 hours with no refresh mechanism. Production would implement refresh tokens.

5. **Concurrent Safety**: Password hashing is CPU-intensive. In high-load scenarios, consider async hashing with tokio::task::spawn_blocking.

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All authentication files created
- JWT creation and validation work correctly
- Password hashing and verification are secure
- Code compiles without errors
- Module can be imported by Task 5

## Related Tasks
- **Task 5**: Shopping Cart API (depends on this task for JWT validation)
- **Task 7**: Integration Tests (will test authentication flow)
- **Independent of**: Tasks 1, 4, 6 (runs in parallel)

## Security Best Practices Applied
✅ Use industry-standard algorithms (Argon2, HS256)
✅ Random salts for each password
✅ Never expose password hashes in API responses
✅ Time-limited tokens with expiration
✅ Constant-time password comparison (via Argon2)

## Production Improvements (Not in Scope)
- Load secret key from environment variable
- Implement refresh token mechanism
- Add token revocation/blacklist
- Use async password hashing for better performance
- Add rate limiting to prevent brute force
- Implement account lockout after failed attempts
- Add email verification and password reset flows

## References
- [jsonwebtoken Documentation](https://docs.rs/jsonwebtoken/)
- [Argon2 Documentation](https://docs.rs/argon2/)
- [JWT Standard (RFC 7519)](https://tools.ietf.org/html/rfc7519)
- [OWASP Password Storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- Project PRD: `.taskmaster/docs/prd.txt`
