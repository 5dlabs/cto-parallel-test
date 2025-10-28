# Task 3: User Authentication Module - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `src/auth/mod.rs` exists
- [ ] `src/auth/jwt.rs` exists
- [ ] `src/auth/models.rs` exists
- [ ] `Cargo.toml` has been updated with authentication dependencies

## Code Quality Criteria

### ✅ Module Exports (src/auth/mod.rs)
- [ ] Contains `pub mod jwt;`
- [ ] Contains `pub mod models;`
- [ ] Re-exports `create_token` function: `pub use self::jwt::create_token;`
- [ ] Re-exports `validate_token` function: `pub use self::jwt::validate_token;`
- [ ] Re-exports `User` struct: `pub use self::models::User;`
- [ ] Valid Rust module syntax

### ✅ JWT Implementation (src/auth/jwt.rs)
- [ ] Imports jsonwebtoken types: `encode, decode, Header, Validation, EncodingKey, DecodingKey`
- [ ] Imports serde traits: `Serialize, Deserialize`
- [ ] Imports time utilities: `std::time::{SystemTime, UNIX_EPOCH}`
- [ ] Claims struct defined with:
  - `sub: String` field
  - `exp: usize` field
  - `iat: usize` field
  - `Debug, Serialize, Deserialize` derives
- [ ] `create_token(user_id: &str)` function exists
  - Returns `Result<String, jsonwebtoken::errors::Error>`
  - Calculates expiration as current time + 24 hours (86400 seconds)
  - Creates Claims with user_id in sub field
  - Sets iat to current timestamp
  - Uses secret key `b"test_secret_key"`
  - Calls `encode` with Header::default(), claims, and EncodingKey
- [ ] `validate_token(token: &str)` function exists
  - Returns `Result<Claims, jsonwebtoken::errors::Error>`
  - Uses same secret key `b"test_secret_key"`
  - Calls `decode` with token, DecodingKey, and Validation::default()
  - Returns token_data.claims
- [ ] All functions are `pub` (publicly accessible)

### ✅ User Model (src/auth/models.rs)
- [ ] Imports serde traits: `Serialize, Deserialize`
- [ ] Imports argon2 functions and Config
- [ ] Imports rand::Rng
- [ ] User struct defined with:
  - `id: i32` field
  - `username: String` field
  - `email: String` field
  - `password_hash: String` field
  - `Debug, Serialize, Deserialize` derives
  - `#[serde(skip_serializing)]` attribute on password_hash
- [ ] User impl block contains `verify_password` method
  - Signature: `pub fn verify_password(&self, password: &str) -> bool`
  - Uses `argon2::verify_encoded`
  - Verifies against `self.password_hash`
  - Converts password to bytes with `.as_bytes()`
  - Returns `unwrap_or(false)` on errors
- [ ] User impl block contains `hash_password` method
  - Signature: `pub fn hash_password(password: &str) -> String`
  - Generates random 32-byte salt array with `rand::thread_rng().gen::<[u8; 32]>()`
  - Uses `Config::default()` for Argon2 config
  - Calls `argon2::hash_encoded` with password bytes, salt, and config
  - Returns unwrapped encoded hash string

### ✅ Dependencies (Cargo.toml)
- [ ] Includes `jsonwebtoken = "8.3.0"`
- [ ] Includes `argon2 = "0.5.0"`
- [ ] Includes `rand = "0.8.5"`
- [ ] Dependencies are in the `[dependencies]` section
- [ ] TOML syntax is valid

## Security Criteria

### ✅ Password Security
- [ ] Uses Argon2 algorithm (not MD5, SHA, or plain bcrypt)
- [ ] Generates random salt for each password hash
- [ ] Salt is 32 bytes (256 bits)
- [ ] Password hash is never exposed in serialized User JSON
- [ ] Verification uses constant-time comparison (provided by Argon2)

### ✅ JWT Security
- [ ] Tokens have expiration time (24 hours from creation)
- [ ] Tokens include issued-at timestamp
- [ ] Uses HMAC-SHA256 for signing (default Header algorithm)
- [ ] Secret key is used consistently for encode and decode
- [ ] Validation checks signature and expiration automatically

## Functional Criteria

### ✅ JWT Token Lifecycle
- [ ] Create token with user ID
- [ ] Token can be decoded to retrieve user ID
- [ ] Expired tokens are rejected (if tested after 24 hours)
- [ ] Invalid signatures are rejected
- [ ] Token contains valid JSON structure

### ✅ Password Lifecycle
- [ ] Plain password can be hashed
- [ ] Correct password verification returns true
- [ ] Incorrect password verification returns false
- [ ] Each hash of same password produces different result (due to random salt)
- [ ] Hash can be stored and verified later

## Compilation and Testing Criteria

### ✅ Build Verification
- [ ] `cargo check` completes without errors
- [ ] `cargo build` completes successfully
- [ ] No warnings related to unused imports
- [ ] All dependencies resolve correctly

### ✅ Unit Test Validation
Implement and run these tests:

**JWT Test**:
```rust
#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
}
```

**Password Test**:
```rust
#[test]
fn test_password_hashing_and_verification() {
    let password = "secure_password";
    let hashed = User::hash_password(password);
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };
    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));
}
```

**Serialization Test**:
```rust
#[test]
fn test_user_serialization_skips_password() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash123".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    assert!(!json.contains("password_hash"));
    assert!(json.contains("username"));
}
```

- [ ] All unit tests pass
- [ ] JWT test creates and validates token successfully
- [ ] Password test hashes and verifies correctly
- [ ] Serialization test confirms password_hash is not in JSON

## Integration Criteria

### ✅ Compatibility with Dependent Tasks
- [ ] Task 5 can import and use `validate_token` function
- [ ] Task 5 can import and use `User` model
- [ ] Task 7 can test JWT token flow
- [ ] Module exports are accessible from other modules

## Testing Commands

### Manual Validation Steps

1. **Verify File Existence**
   ```bash
   ls -la src/auth/mod.rs
   ls -la src/auth/jwt.rs
   ls -la src/auth/models.rs
   ```

2. **Check Rust Compilation**
   ```bash
   cargo check
   cargo build
   ```

3. **Validate Dependencies**
   ```bash
   cargo tree | grep jsonwebtoken
   cargo tree | grep argon2
   cargo tree | grep rand
   ```

4. **Run Unit Tests**
   ```bash
   cargo test auth
   ```

5. **Test JWT Functionality (via rust code)**
   ```rust
   use crate::auth::{create_token, validate_token};

   let token = create_token("test_user_123").unwrap();
   println!("Token: {}", token);

   let claims = validate_token(&token).unwrap();
   println!("User ID: {}", claims.sub);
   ```

6. **Test Password Functionality**
   ```rust
   use crate::auth::User;

   let hash = User::hash_password("mypassword");
   println!("Hash: {}", hash);

   let user = User {
       id: 1,
       username: "test".to_string(),
       email: "test@test.com".to_string(),
       password_hash: hash,
   };

   println!("Correct password: {}", user.verify_password("mypassword"));
   println!("Wrong password: {}", user.verify_password("wrong"));
   ```

## Success Definition

**Task is COMPLETE when:**
1. All required files exist with correct implementations
2. JWT tokens can be created and validated successfully
3. Passwords can be hashed and verified securely
4. User struct serializes without exposing password_hash
5. All unit tests pass
6. Code compiles without errors or warnings
7. Dependencies resolve correctly

**Task is INCOMPLETE if:**
- Any required file is missing
- Compilation errors exist
- JWT creation or validation fails
- Password hashing or verification fails
- Password hash appears in serialized JSON
- Unit tests fail
- Security best practices are violated

## Estimated Completion Time
45 minutes (as specified in PRD)

## Dependencies
None - This is a Level 0 task

## Blocks
- Task 5: Shopping Cart API (needs JWT validation)
- Task 7: Integration Tests (needs authentication to test)

## Security Checklist
- [ ] ✅ Argon2 used (not weaker algorithms)
- [ ] ✅ Random salts generated
- [ ] ✅ Password hash never serialized
- [ ] ✅ JWT tokens have expiration
- [ ] ✅ HMAC-SHA256 used for signing
- [ ] ⚠️ Test secret key used (acceptable for test project)
- [ ] ⚠️ No rate limiting (out of scope)
- [ ] ⚠️ No token refresh (out of scope)
