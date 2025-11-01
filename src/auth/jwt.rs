//! JWT token creation and validation
//!
//! This module provides stateless authentication using JSON Web Tokens (JWT).
//! Tokens expire after 24 hours and include standard claims.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing standard token information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - the user ID this token represents
    pub sub: String,
    /// Expiration time (seconds since `UNIX_EPOCH`)
    pub exp: usize,
    /// Issued at time (seconds since `UNIX_EPOCH`)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
///
/// # Arguments
///
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
///
/// Returns a `Result` containing the encoded JWT string on success, or a JWT error on failure.
///
/// # Errors
///
/// Returns an error if JWT encoding fails (e.g., invalid secret key format).
///
/// # Panics
///
/// Panics if system time is before `UNIX_EPOCH` (extremely rare, would indicate system clock issues).
///
/// # Security
///
/// - Tokens expire after 24 hours
/// - Secret key is loaded from `JWT_SECRET` environment variable
/// - Falls back to a development secret if not set (NOT for production)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// ```
#[allow(clippy::cast_possible_truncation)]  // usize is appropriate for JWT timestamps
#[allow(clippy::disallowed_methods)]  // SystemTime::now is acceptable for JWT token timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX_EPOCH")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now as usize,
    };

    // In production, JWT_SECRET MUST be set via environment variable
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and extracts its claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns a `Result` containing the decoded `Claims` on success, or a JWT error on failure.
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Secret key doesn't match
///
/// # Validation
///
/// This function validates:
/// - Token signature using the secret key
/// - Token expiration time
/// - Token structure and format
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").unwrap();
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user_123");
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
#[allow(clippy::disallowed_methods)]  // Allow SystemTime::now in tests
#[allow(clippy::cast_possible_truncation)]  // Allow casts in tests
#[allow(clippy::cast_possible_wrap)]  // Allow casts in tests
#[allow(clippy::cast_lossless)]  // Allow casts in tests
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
    fn test_validate_valid_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token);
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        // Verify subject
        assert_eq!(claims.sub, user_id);

        // Verify expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        assert!(claims.exp > now);

        // Verify issued at is in the past or now
        assert!(claims.iat <= now + 1); // Allow 1 second tolerance

        // Verify expiration is ~24 hours from issued
        let expected_duration = 24 * 3600;
        let actual_duration = claims.exp - claims.iat;
        assert!((actual_duration as i64 - expected_duration as i64).abs() < 10);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_empty_token_rejected() {
        assert!(validate_token("").is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "123";
        let token1 = create_token(user_id).unwrap();

        // Wait 1 second to ensure different timestamp (JWT uses seconds, not milliseconds)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).unwrap();

        // Tokens should be different due to different iat
        assert_ne!(token1, token2);

        // But both should validate to the same user
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, claims2.sub);
    }

    #[test]
    fn test_tampered_token_rejected() {
        let token = create_token("123").unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        tampered.push('x');

        assert!(validate_token(&tampered).is_err());
    }
}
