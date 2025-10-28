# Acceptance Criteria: User Authentication Module

## Required Files

### ✅ `src/auth/mod.rs`
- [ ] File exists at `src/auth/mod.rs`
- [ ] Contains `pub mod jwt;` declaration
- [ ] Contains `pub mod models;` declaration
- [ ] Re-exports `create_token` and `validate_token` from jwt module
- [ ] Re-exports `User` from models module
- [ ] Proper pub use statements for ergonomic imports

### ✅ `src/auth/jwt.rs`
- [ ] File exists at `src/auth/jwt.rs`
- [ ] Imports jsonwebtoken crate components (encode, decode, Header, Validation, EncodingKey, DecodingKey)
- [ ] Imports serde::{Serialize, Deserialize}
- [ ] Imports std::time::{SystemTime, UNIX_EPOCH}
- [ ] Defines Claims struct with fields: sub (String), exp (usize), iat (usize)
- [ ] Claims struct derives Debug, Serialize, Deserialize
- [ ] Implements `create_token(user_id: &str)` function
- [ ] create_token returns `Result<String, jsonwebtoken::errors::Error>`
- [ ] Token expiration set to 24 hours (24 * 3600 seconds)
- [ ] Token includes sub (user ID), exp (expiration), and iat (issued at) claims
- [ ] Implements `validate_token(token: &str)` function
- [ ] validate_token returns `Result<Claims, jsonwebtoken::errors::Error>`
- [ ] Uses secret key b"test_secret_key"
- [ ] Proper error propagation with ? operator

### ✅ `src/auth/models.rs`
- [ ] File exists at `src/auth/models.rs`
- [ ] Imports serde::{Serialize, Deserialize}
- [ ] Imports argon2::{self, Config}
- [ ] Imports rand::Rng
- [ ] Defines User struct with fields: id (i32), username (String), email (String), password_hash (String)
- [ ] User struct derives Debug, Serialize, Deserialize
- [ ] password_hash field has `#[serde(skip_serializing)]` attribute
- [ ] Implements `verify_password(&self, password: &str) -> bool` method
- [ ] verify_password uses argon2::verify_encoded
- [ ] verify_password returns false on error (unwrap_or(false))
- [ ] Implements `hash_password(password: &str) -> String` static method
- [ ] hash_password generates 32-byte random salt
- [ ] hash_password uses Argon2 with Config::default()
- [ ] hash_password returns encoded hash string

### ✅ `Cargo.toml` Updates
- [ ] Contains `jsonwebtoken = "8.3.0"` dependency
- [ ] Contains `argon2 = "0.5.0"` dependency
- [ ] Contains `rand = "0.8.5"` dependency
- [ ] All dependencies in `[dependencies]` section
- [ ] Proper TOML syntax

## Functional Requirements

### JWT Token Creation
- [ ] Tokens are valid JWT format (header.payload.signature)
- [ ] Tokens include all required claims (sub, exp, iat)
- [ ] Tokens expire 24 hours after creation
- [ ] Different user IDs produce different tokens
- [ ] Function returns Ok with valid JWT string
- [ ] Function returns Err on encoding failure

### JWT Token Validation
- [ ] Valid tokens decode successfully
- [ ] Expired tokens are rejected
- [ ] Tampered tokens are rejected
- [ ] Invalid format tokens are rejected
- [ ] Claims are correctly extracted
- [ ] Function returns Ok with Claims on success
- [ ] Function returns Err on validation failure

### Password Hashing
- [ ] Same password produces different hashes (due to random salt)
- [ ] Hashes are in Argon2 encoded format
- [ ] Hash includes version, parameters, salt, and hash
- [ ] Function never panics on valid input
- [ ] Uses secure random number generator for salt
- [ ] Argon2 Config::default() provides secure parameters

### Password Verification
- [ ] Correct password returns true
- [ ] Incorrect password returns false
- [ ] Invalid hash format returns false (doesn't panic)
- [ ] Uses constant-time comparison (via argon2 library)
- [ ] No timing side-channel vulnerabilities

### User Model Serialization
- [ ] User can be serialized to JSON
- [ ] Serialized User does NOT include password_hash field
- [ ] User can be deserialized from JSON
- [ ] id, username, and email serialize correctly

## Validation Tests

### Compilation Tests
```bash
cargo check
```
- [ ] Compiles without errors
- [ ] No warnings related to new code
- [ ] All imports resolve correctly

### Dependency Verification
```bash
grep jsonwebtoken Cargo.toml
grep argon2 Cargo.toml
grep rand Cargo.toml
```
- [ ] All three dependencies present
- [ ] Correct versions specified

### Module Structure
```bash
ls -la src/auth/
```
- [ ] Directory exists
- [ ] Contains mod.rs, jwt.rs, models.rs

### JWT Functionality Test
```rust
// Example test
#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
    assert!(claims.exp > claims.iat);
}
```
- [ ] Token creation succeeds
- [ ] Token validation succeeds
- [ ] Claims contain correct user ID
- [ ] Expiration is after issued-at time

### Password Hashing Test
```rust
// Example test
#[test]
fn test_password_hashing() {
    let password = "test_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Different salts produce different hashes
    assert_ne!(hash1, hash2);

    // Both verify successfully
    assert!(argon2::verify_encoded(&hash1, password.as_bytes()).unwrap());
    assert!(argon2::verify_encoded(&hash2, password.as_bytes()).unwrap());
}
```
- [ ] Same password produces different hashes
- [ ] Both hashes verify successfully
- [ ] Hash format is valid Argon2

### Serialization Test
```rust
// Example test
#[test]
fn test_user_serialization() {
    let user = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(!json.contains("password_hash"));
    assert!(json.contains("username"));
}
```
- [ ] User serializes to JSON
- [ ] password_hash is NOT in JSON output
- [ ] Other fields ARE in JSON output

## Security Requirements

### Cryptographic Strength
- [ ] Argon2 is memory-hard (resistant to GPU/ASIC attacks)
- [ ] Salt is cryptographically random (uses rand crate)
- [ ] Salt is unique per password (32 bytes)
- [ ] JWT uses HMAC-SHA256 signature
- [ ] Token expiration prevents long-lived token abuse

### Side-Channel Resistance
- [ ] Password verification uses constant-time comparison (via argon2)
- [ ] No timing attacks possible on password verification
- [ ] No obvious information leakage in error messages

### Data Protection
- [ ] Password hash never serialized to JSON
- [ ] Password hash marked with skip_serializing
- [ ] No plain-text passwords stored
- [ ] Proper error handling prevents information leakage

### Known Test Limitations (Acceptable)
- ⚠️ Hardcoded JWT secret (test only - document for production fix)
- ⚠️ No token refresh mechanism
- ⚠️ No token revocation/blacklisting
- ⚠️ Basic claims structure
- ⚠️ No rate limiting

## Non-Functional Requirements

### Code Quality
- [ ] Follows Rust naming conventions
- [ ] Proper error handling (no unwrap() in library code except hash_password)
- [ ] Clear function signatures
- [ ] Appropriate use of Result types
- [ ] Code is properly formatted (`cargo fmt`)

### Documentation
- [ ] Comments explain test secret key limitation
- [ ] Claims struct fields documented
- [ ] Security considerations clear from code

### Performance
- [ ] Argon2 parameters appropriate for test environment
- [ ] No unnecessary allocations
- [ ] Efficient token encoding/decoding

## Integration Readiness

### Task 5 Dependency
- [ ] validate_token is publicly accessible
- [ ] Claims can be extracted from tokens
- [ ] Ready for use in cart API authentication

### Task 7 Testing
- [ ] All functions testable
- [ ] Deterministic behavior (except salt randomness)
- [ ] Clear success/failure cases

## Edge Cases

### JWT Edge Cases
- [ ] Empty user ID handled correctly
- [ ] Very long user ID handled correctly
- [ ] Token validation fails gracefully on invalid input
- [ ] Expired token properly rejected

### Password Edge Cases
- [ ] Empty password hashes successfully
- [ ] Very long password hashes successfully
- [ ] Special characters in password handled correctly
- [ ] verification with invalid hash returns false, doesn't panic

## Success Metrics

- **Completion**: All required files created with correct implementations
- **Quality**: Code passes `cargo check` and follows best practices
- **Security**: Implements industry-standard authentication patterns
- **Integration**: Ready for use by Task 5 and testing by Task 7
- **Cryptography**: Uses appropriate algorithms and secure defaults

## Manual Verification Checklist

1. [ ] Verify all 3 files created in src/auth/
2. [ ] Confirm Cargo.toml has all 3 dependencies
3. [ ] Run `cargo check` - must pass
4. [ ] Verify JWT creation returns valid token string
5. [ ] Verify token validation extracts correct claims
6. [ ] Verify password hashing produces different outputs
7. [ ] Verify password verification works correctly
8. [ ] Confirm password_hash not in JSON serialization
9. [ ] Check security best practices followed
10. [ ] Validate no compilation warnings
