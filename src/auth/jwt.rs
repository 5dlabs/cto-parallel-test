//! # JWT Token Handling
//!
//! This module provides JWT token creation and validation with secure defaults:
//! - 24-hour token expiration
//! - HMAC-based signing
//! - Standard JWT claims (sub, exp, iat)
//! - Environment-based secret key configuration

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user authentication information
///
/// # Fields
/// * `sub` - Subject (user ID as string)
/// * `exp` - Expiration time (Unix timestamp)
/// * `iat` - Issued at time (Unix timestamp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
}

/// Create a JWT token for the given user ID
///
/// # Arguments
/// * `user_id` - The user's unique identifier
///
/// # Returns
/// * `Ok(String)` - The signed JWT token
/// * `Err(jsonwebtoken::errors::Error)` - If token creation fails
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
///
/// # Security Notes
/// - Tokens expire after 24 hours
/// - Uses `JWT_SECRET` environment variable or fallback for development
/// - Production deployments MUST set `JWT_SECRET` environment variable
///
/// # Errors
/// Returns an error if JWT encoding fails (e.g., invalid secret key)
///
/// # Panics
/// Panics if system time is before Unix epoch (extremely rare)
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // JWT tokens require wall-clock time (not monotonic time) for iat/exp claims
    // This is a standard JWT requirement and cannot use Instant
    #[allow(clippy::disallowed_methods)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).unwrap_or(usize::MAX),
        iat: usize::try_from(now).unwrap_or(0),
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

/// Validate a JWT token and extract its claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Ok(Claims)` - The validated claims from the token
/// * `Err(jsonwebtoken::errors::Error)` - If validation fails
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "123");
/// ```
///
/// # Validation
/// - Checks token signature
/// - Verifies expiration time
/// - Ensures token structure is valid
///
/// # Errors
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token structure is malformed
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
    fn test_jwt_creation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        // Token should not be empty
        assert!(!token.is_empty());

        // Token should have JWT structure (three parts separated by dots)
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_jwt_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        // Check expiration is set
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);

        // Expiration should be approximately 24 hours from issued at
        let expected_duration = 24 * 3600;
        let actual_duration = claims.exp.saturating_sub(claims.iat);
        let diff = actual_duration.abs_diff(expected_duration);
        assert!(diff < 10, "Token expiration should be ~24 hours");
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_tampered_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        tampered.replace_range(10..11, "X");

        // Validation should fail
        assert!(validate_token(&tampered).is_err());
    }

    #[test]
    fn test_different_users_different_tokens() {
        let token1 = create_token("user1").unwrap();
        let token2 = create_token("user2").unwrap();

        // Tokens should be different
        assert_ne!(token1, token2);

        // Claims should have different user IDs
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
    }

    #[test]
    fn test_same_user_different_timestamps() {
        let token1 = create_token("user1").unwrap();
        // Sleep for at least 1 second to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token("user1").unwrap();

        // Tokens should be different due to different issued-at timestamps
        assert_ne!(token1, token2);

        // But both should validate to the same user
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, claims2.sub);

        // Verify different issued-at times
        assert_ne!(claims1.iat, claims2.iat);
    }
}
