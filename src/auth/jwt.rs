//! JWT token creation and validation
//!
//! This module handles JSON Web Token (JWT) operations for stateless authentication.
//! Tokens expire after 24 hours and include standard claims (sub, exp, iat).
//!
//! # Security Considerations
//! - JWT secret should be loaded from environment variable in production
//! - Tokens should be transmitted over HTTPS only
//! - Token expiration is enforced during validation

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing standard token claims
///
/// # Fields
/// - `sub`: Subject (user ID)
/// - `exp`: Expiration time (Unix timestamp)
/// - `iat`: Issued at time (Unix timestamp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
///
/// The token will expire 24 hours after creation. The JWT secret is loaded
/// from the `JWT_SECRET` environment variable, with a fallback for development.
///
/// # Arguments
/// - `user_id`: The user ID to encode in the token's subject claim
///
/// # Returns
/// - `Ok(String)`: The encoded JWT token
/// - `Err(jsonwebtoken::errors::Error)`: If token creation fails
///
/// # Errors
/// Returns an error if JWT encoding fails (rare, usually indicates invalid key)
///
/// # Panics
/// Panics if system time is before Unix epoch (extremely rare)
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime::now needed for JWT timestamps
#[allow(clippy::cast_possible_truncation)] // usize is sufficient for Unix timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

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

/// Validates a JWT token and extracts its claims
///
/// This function verifies the token's signature and checks its expiration.
/// Expired or tampered tokens will be rejected.
///
/// # Arguments
/// - `token`: The JWT token string to validate
///
/// # Returns
/// - `Ok(Claims)`: The decoded claims if token is valid
/// - `Err(jsonwebtoken::errors::Error)`: If token is invalid or expired
///
/// # Errors
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Token was signed with a different secret
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).expect("Token should be valid");
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
        let token = create_token("123").expect("Failed to create token");
        assert!(!token.is_empty());
        // JWT tokens have three parts separated by dots
        assert_eq!(token.matches('.').count(), 2);
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "user_123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // Test needs SystemTime::now
    #[allow(clippy::cast_possible_truncation)] // Test code, usize is fine for timestamps
    #[allow(clippy::cast_possible_wrap)] // Test code, i64 is fine for time difference
    fn test_token_contains_correct_claims() {
        let token = create_token("123").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        // Check that exp is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        assert!(claims.exp > now, "Expiration should be in the future");
        assert!(claims.iat <= now, "Issued at should be in the past or now");
        assert_eq!(claims.sub, "123");

        // Check expiration is approximately 24 hours (allow 10 second variance)
        let expected_exp = now + 24 * 3600;
        let exp_diff = (claims.exp as i64 - expected_exp as i64).abs();
        assert!(exp_diff < 10, "Expiration should be ~24 hours from now");
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err(), "Invalid token should be rejected");
    }

    #[test]
    fn test_empty_token() {
        let result = validate_token("");
        assert!(result.is_err(), "Empty token should be rejected");
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let token1 = create_token("123").expect("Failed to create token 1");
        // Sleep to ensure different iat (need at least 1 second for Unix timestamp difference)
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token("123").expect("Failed to create token 2");

        // Tokens should be different due to different iat
        assert_ne!(
            token1, token2,
            "Different tokens should have different values"
        );

        // Both should validate successfully
        let claims1 = validate_token(&token1).expect("Token 1 should validate");
        let claims2 = validate_token(&token2).expect("Token 2 should validate");

        assert_eq!(claims1.sub, claims2.sub);
        assert!(
            claims2.iat > claims1.iat,
            "Second token should have later iat"
        );
    }

    #[test]
    fn test_token_with_special_characters_in_user_id() {
        let user_id = "user@example.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_with_empty_user_id() {
        let token = create_token("").expect("Failed to create token with empty user_id");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_tampered_token() {
        let token = create_token("123").expect("Failed to create token");
        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered);
        assert!(result.is_err(), "Tampered token should be rejected");
    }
}
