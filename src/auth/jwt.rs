//! JWT token creation and validation.
//!
//! This module provides stateless authentication using JSON Web Tokens (JWT).
//! Tokens expire after 24 hours and include standard claims (sub, exp, iat).

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims structure containing user ID and expiration information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID.
///
/// The token will expire after 24 hours and includes:
/// - `sub`: The user ID
/// - `exp`: Expiration timestamp (now + 24 hours)
/// - `iat`: Issued at timestamp (current time)
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// A JWT token string or an error if token creation fails
///
/// # Security
///
/// The `JWT_SECRET` environment variable should be set in production.
/// A fallback secret is provided for development only.
///
/// # Errors
///
/// Returns an error if:
/// - Token encoding fails (rare, indicates a configuration issue)
/// - System time cannot be determined (system clock issues)
///
/// # Panics
///
/// Panics if the system clock has moved backwards (extremely rare).
/// This indicates a serious system time issue that should be investigated.
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
// JWT authentication inherently requires wall-clock time (not performance measurement)
// Allow SystemTime::now() for this security-critical use case where we need actual timestamps
#[allow(clippy::disallowed_methods)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    // SAFETY: Unix timestamps fit in usize on all modern platforms
    // u64 to usize cast is safe here as timestamps are well within usize range
    #[allow(clippy::cast_possible_truncation)]
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now as usize,
    };

    // In production, load from environment variable
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and extracts its claims.
///
/// This function verifies:
/// - Token signature is valid
/// - Token has not expired
/// - Token structure is correct
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The validated claims or an error if validation fails
///
/// # Security
///
/// Tokens with invalid signatures, expired tokens, or malformed tokens
/// will be rejected with an error.
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Token cannot be decoded
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let user_id = "123";
        let token = create_token(user_id);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    #[allow(clippy::cast_possible_truncation)]
    fn test_token_contains_correct_claims() {
        let user_id = "user_abc";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify user ID
        assert_eq!(claims.sub, user_id);

        // Verify expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
        assert!(claims.exp > now);

        // Verify expiration is approximately 24 hours from issue time
        let expected_exp = claims.iat + 24 * 3600;
        assert_eq!(claims.exp, expected_exp);

        // Verify issued at is reasonable (within last minute)
        assert!(now - claims.iat < 60);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_user_id_handled() {
        let token = create_token("").expect("Failed to create token with empty user ID");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_long_user_id_handled() {
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_tampered_token_rejected() {
        let token = create_token("123").expect("Failed to create token");

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        tampered.push('x');

        let result = validate_token(&tampered);
        assert!(result.is_err());
    }
}
