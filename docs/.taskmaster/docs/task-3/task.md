# Task 3: User Authentication Module

## Overview
Implement JWT-based authentication with secure password hashing using Argon2 for the e-commerce API.

## Objectives
- JWT token creation and validation
- Secure password hashing with Argon2
- User model with authentication methods
- 24-hour token expiration
- Independent authentication module

## Context
**Level 0** task - runs in parallel with Tasks 1, 4, and 6. No dependencies.

## Technical Specifications
- **JWT**: jsonwebtoken 8.3.0
- **Password Hashing**: argon2 0.5.0
- **Random Salt**: rand 0.8.5
- **Token Expiration**: 24 hours
- **Secret**: Environment variable (test_secret_key for development)

## Implementation

### Dependencies (Cargo.toml)
```toml
jsonwebtoken = "8.3.0"
argon2 = "0.5.0"
rand = "0.8.5"
```

### Module Structure (src/auth/mod.rs)
```rust
pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
```

### JWT Handling (src/auth/jwt.rs)
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub exp: usize,   // Expiration
    pub iat: usize,   // Issued at
}

pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 24 * 3600;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    let secret = b"test_secret_key";
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default())
        .map(|data| data.claims)
}
```

### User Model (src/auth/models.rs)
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

## Validation
```bash
cargo test auth::
```

## Dependencies
None (Level 0)

## Estimated Effort
45 minutes
