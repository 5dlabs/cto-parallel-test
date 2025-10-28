# Task 3: User Authentication Module - Agent Prompt

You are a Rust security engineer tasked with implementing user authentication and JWT handling for a test e-commerce API.

## Your Mission
Create a complete authentication module with JWT token management and secure password hashing. This is a foundational security component that other tasks will depend on for user authorization.

## What You Must Create

### 1. Update `Cargo.toml`
Add these dependencies to the `[dependencies]` section:
```toml
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

### 2. Create `src/auth/mod.rs`
Module exports and re-exports:
```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token};
pub use self::models::User;
```

### 3. Create `src/auth/jwt.rs`
Implement JWT token handling with these exact components:

**Claims struct**:
- `sub: String` (user ID)
- `exp: usize` (expiration timestamp)
- `iat: usize` (issued at timestamp)
- Derive `Debug, Serialize, Deserialize`

**`create_token(user_id: &str)` function**:
- Calculate expiration: current time + 24 hours
- Create Claims with user_id, exp, iat
- Use secret: `b"test_secret_key"`
- Encode with `Header::default()` and `EncodingKey::from_secret`
- Return `Result<String, jsonwebtoken::errors::Error>`

**`validate_token(token: &str)` function**:
- Use same secret: `b"test_secret_key"`
- Decode with `Validation::default()` and `DecodingKey::from_secret`
- Return `Result<Claims, jsonwebtoken::errors::Error>`

### 4. Create `src/auth/models.rs`
Implement User model and password utilities:

**User struct**:
- `id: i32`
- `username: String`
- `email: String`
- `password_hash: String` with `#[serde(skip_serializing)]`
- Derive `Debug, Serialize, Deserialize`

**`User::verify_password(&self, password: &str) -> bool`**:
- Use `argon2::verify_encoded`
- Verify against `self.password_hash`
- Return `unwrap_or(false)` for errors

**`User::hash_password(password: &str) -> String`**:
- Generate random 32-byte salt with `rand::thread_rng()`
- Use `argon2::Config::default()`
- Hash with `argon2::hash_encoded`
- Return encoded hash string
- This is a static method (no &self)

## Key Requirements

✅ **Security**:
- Use Argon2 for password hashing (not bcrypt or plain SHA)
- Generate random salt for each password
- Never serialize password_hash in JSON
- JWT tokens expire after 24 hours

✅ **API Completeness**:
- Both `create_token` and `validate_token` must work
- Both `hash_password` and `verify_password` must work
- User struct must be serializable (except password_hash)

✅ **Error Handling**:
- Return proper Result types for fallible operations
- Use `unwrap()` only where specified
- Let library errors propagate upward

✅ **Module Structure**:
- Clean separation: JWT logic in jwt.rs, User logic in models.rs
- Re-export key functions at module level
- Proper use of pub visibility

## Constraints
- This is a **Level 0** task with no dependencies on other tasks
- Uses hardcoded test secret (acceptable for this test project)
- No database operations (just models and utilities)
- Keep implementations straightforward - this is a test project

## Validation
After completing the work:
1. Verify all files exist at specified paths
2. Ensure `cargo check` passes
3. Test token creation and validation work together
4. Test password hashing and verification work together
5. Confirm User struct can be serialized without password_hash

## Success Definition
Task is complete when:
- All three auth files exist with correct implementations
- JWT tokens can be created and validated
- Passwords can be hashed and verified
- User struct serializes correctly (no password_hash in JSON)
- Code compiles without errors
- All dependencies resolve

## Context
You're working on a parallel task execution test.

**Your independence**:
- No dependencies - you can start immediately

**Tasks depending on you**:
- Task 5: Shopping Cart API (needs JWT validation for auth)
- Task 7: Integration Tests (will test your auth flow)

**Running in parallel (Level 0)**:
- Task 1: Database Schema
- Task 4: Product Catalog
- Task 6: Frontend Components

---

**Start working now. Create the files, write the secure code, and verify authentication works.**
