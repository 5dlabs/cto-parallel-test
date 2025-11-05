# CTO Parallel Test - User Authentication Module

A production-grade Rust authentication library implementing JWT token handling and Argon2 password hashing.

## Features

- ✅ **JWT Token Management**: Create and validate JSON Web Tokens with 24-hour expiration
- ✅ **Secure Password Hashing**: Argon2id algorithm with random salt generation
- ✅ **Type-Safe Models**: Request/Response DTOs for authentication endpoints
- ✅ **Security Best Practices**: Password hashes never serialized, constant-time comparison
- ✅ **Comprehensive Tests**: 19 unit tests covering all functionality and edge cases
- ✅ **Production Ready**: No mocks, configurable via environment variables

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cto-parallel-test = "0.1.0"
```

## Quick Start

### Creating and Validating JWT Tokens

```rust
use cto_parallel_test::auth::jwt::{create_token, validate_token};

// Create a token for a user
let token = create_token("user_123").expect("Failed to create token");

// Validate and extract claims
let claims = validate_token(&token).expect("Failed to validate token");
println!("User ID: {}", claims.sub);
println!("Expires at: {}", claims.exp);
```

### Password Hashing and Verification

```rust
use cto_parallel_test::auth::models::User;

// Hash a password (do this during registration)
let password = "user_provided_password";
let password_hash = User::hash_password(password);

// Create a user with the hashed password
let user = User {
    id: 1,
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    password_hash,
};

// Verify password during login
if user.verify_password(password) {
    println!("Password correct!");
} else {
    println!("Invalid password");
}
```

## Configuration

### JWT Secret Key

Set the `JWT_SECRET` environment variable in production:

```bash
export JWT_SECRET="your_secure_random_secret_key_minimum_32_characters_long"
```

**Generate a secure secret:**
```bash
openssl rand -base64 32
```

**Note:** The library provides a fallback secret for development, but you MUST set a secure secret in production.

## API Documentation

### JWT Module (`auth::jwt`)

#### `create_token(user_id: &str) -> Result<String, Error>`

Creates a JWT token with:
- **Subject (sub)**: User ID
- **Issued At (iat)**: Current timestamp
- **Expiration (exp)**: 24 hours from now

**Returns:** Encoded JWT string or error

#### `validate_token(token: &str) -> Result<Claims, Error>`

Validates a JWT token and extracts claims.

**Returns:** Claims struct containing sub, exp, and iat, or error if invalid/expired

#### `Claims` Struct

```rust
pub struct Claims {
    pub sub: String,  // User ID
    pub exp: usize,   // Expiration timestamp (seconds since UNIX_EPOCH)
    pub iat: usize,   // Issued at timestamp (seconds since UNIX_EPOCH)
}
```

### Models Module (`auth::models`)

#### `User` Struct

```rust
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]  // Password hash never appears in JSON
    pub password_hash: String,
}
```

**Methods:**
- `User::hash_password(password: &str) -> String`: Hash a password using Argon2
- `user.verify_password(password: &str) -> bool`: Verify a password against the stored hash

#### Request/Response DTOs

```rust
// Login request
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Registration request
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// Authentication response
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}
```

## Security Features

### Password Security
- **Argon2id** algorithm (winner of Password Hashing Competition)
- **Memory-hard** function resistant to GPU attacks
- **Random 32-byte salt** for each password
- **Never logged or serialized** - `#[serde(skip_serializing)]` prevents exposure
- **Constant-time comparison** to prevent timing attacks

### JWT Security
- **24-hour expiration** - tokens expire automatically
- **Signature verification** - tampering detection
- **Environment-based secrets** - no hardcoded keys
- **Standard claims** - sub, exp, iat for compatibility

## Testing

Run the test suite:

```bash
# Run all tests
cargo test --workspace --all-features

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test auth::jwt
cargo test auth::models
```

**Test Coverage:** 19 unit tests + 4 doc tests covering:
- JWT creation and validation
- Password hashing and verification
- Token expiration handling
- Invalid token rejection
- Edge cases (empty passwords, special characters, Unicode, long passwords)
- Serialization safety (password hash exclusion)

## Code Quality

This project follows strict quality standards:

```bash
# Format check
cargo fmt --all -- --check

# Linting with pedantic checks
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic

# All quality gates must pass before merge
```

## Architecture

### Module Structure

```
src/
├── lib.rs              # Library root
└── auth/
    ├── mod.rs          # Module exports
    ├── jwt.rs          # JWT token handling
    └── models.rs       # User models and DTOs
```

### Dependencies

- `jsonwebtoken 8.3.0` - JWT encoding/decoding
- `argon2 0.5.0` - Password hashing
- `rand 0.8.5` - Cryptographic random number generation
- `serde 1.0` - Serialization framework
- `serde_json 1.0` - JSON support

## Integration Examples

### Web Framework Integration (Actix-web)

```rust
use actix_web::{post, web, HttpResponse, Result};
use cto_parallel_test::auth::{create_token, LoginRequest, AuthResponse, User};

#[post("/login")]
async fn login(req: web::Json<LoginRequest>) -> Result<HttpResponse> {
    // Fetch user from database (example)
    let user = fetch_user_by_username(&req.username).await?;
    
    // Verify password
    if !user.verify_password(&req.password) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    
    // Create JWT token
    let token = create_token(&user.id.to_string())
        .map_err(|_| HttpResponse::InternalServerError())?;
    
    // Return auth response
    let response = AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
    };
    
    Ok(HttpResponse::Ok().json(response))
}
```

### Protected Route Middleware

```rust
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use cto_parallel_test::auth::validate_token;

async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    
    let claims = validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;
    
    // Store user_id in request extensions for handlers to access
    req.extensions_mut().insert(claims.sub);
    
    Ok(req)
}
```

## Performance Considerations

- **JWT Operations**: Fast (<10ms for creation/validation)
- **Password Hashing**: Intentionally slow (~100-500ms) for security
- **No Database Queries**: All operations are CPU-bound
- **Async Recommendation**: Use `tokio::task::spawn_blocking` for password hashing in async contexts

## Roadmap

This module provides the foundation for:
- **Task 2**: API endpoints (login, register routes)
- **Task 5**: Shopping cart (requires authentication)
- **Task 7**: Integration tests (auth flow testing)

## License

This project is part of the CTO Parallel Test suite.

## Contributing

This module follows the coding guidelines in `coding-guidelines.md` and GitHub workflow in `github-guidelines.md`.

Before submitting changes:
1. Run `cargo fmt --all`
2. Run `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic`
3. Run `cargo test --workspace --all-features`
4. Ensure all quality gates pass

## Support

For issues or questions, please refer to the project documentation in the `docs/` directory.
