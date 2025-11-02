//! JWT token creation and validation for stateless authentication
//!
//! This module handles JWT token lifecycle:
//! - Token creation with 24-hour expiration
//! - Token validation and claims extraction
//! - Secure secret key management via environment variables

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims structure containing user identity and token metadata
///
/// Fields:
/// - `sub`: Subject (user ID as string)
/// - `exp`: Expiration time (Unix timestamp in seconds)
/// - `iat`: Issued at time (Unix timestamp in seconds)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the specified user ID
///
/// The token expires after 24 hours and is signed with the `JWT_SECRET`
/// environment variable. Falls back to a development secret if not set.
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token's `sub` claim
///
/// # Returns
///
/// Returns `Ok(String)` containing the encoded JWT token on success,
/// or `Err(jsonwebtoken::errors::Error)` if token creation fails.
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
///
/// # Security Notes
///
/// - Tokens expire after 24 hours
/// - Production environments MUST set `JWT_SECRET` environment variable
/// - Development fallback secret is intentionally weak
///
/// # Errors
///
/// This function will return an error if JWT encoding fails (e.g., invalid claims structure).
///
/// # Panics
///
/// Panics if system time is before Unix epoch (extremely unlikely in practice).
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Note: Using SystemTime::now() as required by task specification
    // In production, consider using a Clock abstraction for better testability
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let expiration = now.as_secs() + 24 * 3600; // 24 hours from now

    // JWT spec uses numeric dates (seconds since epoch). We use usize for platform compatibility.
    // u64::MAX exceeds practical timestamp limits (year 584 billion+), so conversion is safe.
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).expect("timestamp exceeds usize range"),
        iat: usize::try_from(now.as_secs()).expect("timestamp exceeds usize range"),
    };

    // Load JWT secret from environment, fallback to development secret
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
/// Verifies the token signature and checks expiration time.
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns `Ok(Claims)` containing the decoded claims on success,
/// or `Err(jsonwebtoken::errors::Error)` if validation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).unwrap();
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
        let token_str = token.unwrap();
        assert!(!token_str.is_empty());
        // JWT tokens have three parts separated by dots
        assert_eq!(token_str.matches('.').count(), 2);
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token);
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "789";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        // Check user ID is correct
        assert_eq!(claims.sub, user_id);

        // Check expiration is in the future (~24 hours)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let now_usize = usize::try_from(now).expect("timestamp exceeds usize range");
        let expected_exp = now_usize + 86400; // 24 hours

        // Allow 10 second tolerance for test execution time
        let exp_diff = claims.exp.abs_diff(expected_exp);
        assert!(exp_diff < 10);

        // Check issued at time is recent
        let iat_diff = claims.iat.abs_diff(now_usize);
        assert!(iat_diff < 10);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_token_rejected() {
        let malformed_token = "not-even-a-jwt";
        let result = validate_token(malformed_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token_rejected() {
        let result = validate_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "same_user";

        let token1 = create_token(user_id).expect("Failed to create token 1");
        // Sleep for 1 second to ensure different timestamps (JWT uses second granularity)
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different timestamps
        assert_ne!(token1, token2);

        // But both should be valid
        let claims1 = validate_token(&token1).expect("Token 1 invalid");
        let claims2 = validate_token(&token2).expect("Token 2 invalid");

        assert_eq!(claims1.sub, user_id);
        assert_eq!(claims2.sub, user_id);

        // The iat (issued at) times should be different by at least 1 second
        assert!(claims2.iat > claims1.iat);
    }

    #[test]
    fn test_token_with_empty_user_id() {
        let token = create_token("").expect("Failed to create token with empty user ID");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_token_with_special_characters() {
        let user_id = "user@example.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_with_long_user_id() {
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }
}
