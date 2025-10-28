# Autonomous Agent Prompt: User Authentication Module

## Mission
Implement JWT-based authentication and secure password hashing for a Rust e-commerce API. Create authentication infrastructure that other tasks will use for securing endpoints.

## What You Need to Do

### 1. Create Auth Module Structure
Create `src/auth/mod.rs`:

```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token};
pub use self::models::User;
```

### 2. Implement JWT Token Handling
Create `src/auth/jwt.rs`:

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

    // In a real app, this would be a proper secret key
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

### 3. Implement User Model with Password Hashing
Create `src/auth/models.rs`:

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

### 4. Update Dependencies
Add to `Cargo.toml` [dependencies] section:

```toml
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

## Key Features to Implement

### JWT Tokens
- **Claims Structure**: sub (user ID), exp (expiration), iat (issued at)
- **Expiration**: 24 hours from creation
- **Algorithm**: HS256 (HMAC SHA-256)
- **Secret Key**: Hardcoded for test (b"test_secret_key")

### Password Security
- **Algorithm**: Argon2 (memory-hard, side-channel resistant)
- **Salt**: 32 random bytes per password
- **Serialization**: Password hash NEVER included in JSON output
- **Verification**: Constant-time comparison via argon2 library

## Expected Behavior

### Token Creation
```rust
let token = create_token("123")?;
// Returns JWT string like: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### Token Validation
```rust
let claims = validate_token(&token)?;
// Returns Claims { sub: "123", exp: ..., iat: ... }
```

### Password Hashing
```rust
let hash = User::hash_password("password123");
// Returns: "$argon2i$v=19$m=4096,t=3,p=1$..."
// Each call produces different hash due to random salt
```

### Password Verification
```rust
let user = User { password_hash: hash, ... };
assert!(user.verify_password("password123"));  // true
assert!(!user.verify_password("wrong"));       // false
```

## Validation Steps
Before marking complete:

1. **File Structure**:
   ```bash
   ls -la src/auth/mod.rs src/auth/jwt.rs src/auth/models.rs
   ```

2. **Compilation**:
   ```bash
   cargo check
   ```

3. **Dependencies**:
   ```bash
   grep -E "jsonwebtoken|argon2|rand" Cargo.toml
   ```

4. **JWT Functionality** (manual test if possible):
   ```rust
   let token = create_token("test_user")?;
   let claims = validate_token(&token)?;
   assert_eq!(claims.sub, "test_user");
   ```

5. **Password Hashing** (manual test if possible):
   ```rust
   let hash = User::hash_password("test");
   assert!(argon2::verify_encoded(&hash, b"test").unwrap());
   ```

## Security Notes

### DO
- ✅ Use Argon2 for password hashing
- ✅ Generate random salt for each password
- ✅ Exclude password_hash from JSON serialization
- ✅ Set JWT expiration time
- ✅ Handle errors properly in validation

### DO NOT
- ❌ Store passwords in plain text
- ❌ Use weak hashing (MD5, SHA-1, SHA-256 without salt)
- ❌ Reuse salts across passwords
- ❌ Include password hash in API responses
- ❌ Skip error handling in crypto functions

### Production Considerations (Not for This Test)
- Use environment variable for JWT secret
- Implement token refresh mechanism
- Add token revocation/blacklisting
- Configure Argon2 parameters based on hardware
- Add rate limiting for authentication

## Success Definition
Task is complete when:
- ✅ All 3 files created (mod.rs, jwt.rs, models.rs)
- ✅ JWT creation and validation implemented
- ✅ Password hashing with Argon2 implemented
- ✅ User model with serialization controls
- ✅ Dependencies added to Cargo.toml
- ✅ `cargo check` passes without errors
- ✅ Security best practices followed

## Integration Notes
- Task 5 (Shopping Cart) will use `validate_token` for authentication
- Task 7 (Integration Tests) will test authentication flow
- This task has no dependencies - can run immediately
