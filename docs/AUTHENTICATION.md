# Authentication Module Documentation

## Overview

This project implements a production-grade authentication system using JWT tokens and Argon2 password hashing. The authentication module provides secure user authentication with industry-standard practices.

## Features

- **JWT Token Authentication**: Stateless authentication with 24-hour token expiration
- **Argon2 Password Hashing**: OWASP-recommended password hashing with random salts
- **Secure by Default**: Password hashes never exposed in API responses
- **Environment Configuration**: Configurable JWT secrets for different environments

## Configuration

### Environment Variables

The authentication module requires the following environment variables:

#### JWT_SECRET (Required for Production)
- **Description**: Secret key used for signing JWT tokens
- **Format**: String of at least 32 characters
- **Example**: `export JWT_SECRET="your_secure_random_secret_key_minimum_32_characters_long"`
- **Development Default**: Falls back to a test key if not set (NOT for production use)

### Generating a Secure JWT Secret

For production environments, generate a secure random secret:

```bash
# Using OpenSSL
openssl rand -base64 32

# Using /dev/urandom
cat /dev/urandom | head -c 32 | base64

# Using Python
python3 -c "import secrets; print(secrets.token_urlsafe(32))"
```

## API Usage

### Creating a JWT Token

```rust
use cto_parallel_test::auth::jwt::create_token;

let user_id = "user_123";
let token = create_token(user_id)?;
// Returns a JWT token valid for 24 hours
```

### Validating a JWT Token

```rust
use cto_parallel_test::auth::jwt::validate_token;

let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...";
let claims = validate_token(token)?;
println!("User ID: {}", claims.sub);
println!("Expires at: {}", claims.exp);
```

### Password Hashing

```rust
use cto_parallel_test::auth::models::User;

// Hash a password
let password = "user_password";
let hash = User::hash_password(password);

// Verify a password
let user = User {
    id: 1,
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    password_hash: hash,
};

if user.verify_password(password) {
    println!("Password correct!");
}
```

## Security Features

### Password Security
- **Argon2id Algorithm**: Memory-hard function resistant to GPU attacks
- **Random Salt Generation**: Each password gets a unique 32-byte salt
- **Constant-Time Comparison**: Prevents timing attacks
- **No Password in Responses**: `#[serde(skip_serializing)]` ensures hashes are never exposed

### JWT Security
- **24-Hour Expiration**: Tokens automatically expire after 24 hours
- **Standard Claims**: Implements RFC 7519 with sub, exp, and iat claims
- **Environment-Based Secrets**: Production secrets loaded from environment
- **HS256 Signing**: HMAC with SHA-256 for token signatures

### Best Practices Implemented
- ✅ No hardcoded secrets in production code
- ✅ Passwords never stored in plaintext
- ✅ Password hashes never exposed in API responses
- ✅ Secure random salt generation for each password
- ✅ Token expiration to limit exposure window
- ✅ Error handling that doesn't leak information
- ✅ Comprehensive test coverage including edge cases

## Testing

The authentication module includes comprehensive tests:

```bash
# Run all authentication tests
cargo test auth

# Run with output
cargo test auth -- --nocapture

# Run specific test
cargo test test_password_hashing
```

### Test Coverage
- Password hashing and verification
- JWT token creation and validation
- Token expiration handling
- Invalid token rejection
- Password hash serialization exclusion
- Special characters and Unicode support
- Edge cases (empty passwords, long passwords, etc.)

## Integration with Other Modules

This authentication module is designed to integrate with:
- **API Routes** (future): Login and registration endpoints
- **Middleware** (future): Request authentication and authorization
- **User Management** (future): Profile updates, password resets
- **Session Management** (future): Token refresh and revocation

## Performance Considerations

- **Argon2 Hashing**: ~100ms per hash (intentionally slow for security)
- **JWT Validation**: <1ms per validation
- **No Database Calls**: Authentication logic is stateless
- **Thread Safety**: All functions are thread-safe

For high-throughput scenarios, consider:
- Using `tokio::task::spawn_blocking` for password hashing
- Implementing token caching for validation
- Rate limiting authentication attempts

## Troubleshooting

### Common Issues

#### "JWT_SECRET not set"
- **Development**: The module falls back to a test secret
- **Production**: Must set JWT_SECRET environment variable
- **Solution**: `export JWT_SECRET="your-secure-secret"`

#### "Failed to hash password"
- **Cause**: Usually memory constraints
- **Solution**: Ensure adequate system memory (Argon2 is memory-intensive)

#### "Invalid token"
- **Causes**: Token expired, tampered, or signed with different secret
- **Solution**: Check token expiration and ensure consistent JWT_SECRET

## Security Checklist

Before deploying to production:
- [ ] Set JWT_SECRET environment variable with secure random value
- [ ] Never commit .env files with real secrets
- [ ] Implement rate limiting on authentication endpoints
- [ ] Add logging for authentication failures (without logging passwords)
- [ ] Set up token refresh mechanism if needed
- [ ] Consider implementing token revocation/blacklisting
- [ ] Review and update Argon2 parameters based on hardware
- [ ] Implement account lockout after failed attempts
- [ ] Add multi-factor authentication for sensitive operations

## References

- [JWT RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)
- [Argon2 Specification](https://github.com/P-H-C/phc-winner-argon2)
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
