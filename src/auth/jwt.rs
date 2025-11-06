//! JWT token creation and validation module
//!
//! This module provides functions to create and validate JWT tokens for authentication.
//! Tokens expire after 24 hours and include standard JWT claims (sub, exp, iat).

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing standard token claims
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
/// # Arguments
///
/// * `user_id` - The user ID to include in the token's subject claim
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
///
/// Returns an error if token encoding fails (rare, usually indicates invalid secret)
///
/// # Panics
///
/// Panics if system time is before Unix epoch (extremely unlikely in practice)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).expect("Timestamp too large for usize"),
        iat: usize::try_from(now).expect("Timestamp too large for usize"),
    };

    // Load JWT secret from environment variable with fallback for development
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and returns the claims if valid
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
///
/// Returns an error if:
/// - Token is malformed or invalid
/// - Token has expired
/// - Token signature verification fails
/// - Token was signed with a different secret
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
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
    fn test_token_contains_correct_claims() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "123");
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration_is_24_hours() {
        let user_id = "123";
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let expected_exp = now + 86400; // 24 hours in seconds

        // Calculate the difference using saturating operations to avoid overflow
        let exp_diff = if claims.exp >= usize::try_from(expected_exp).unwrap() {
            claims.exp - usize::try_from(expected_exp).unwrap()
        } else {
            usize::try_from(expected_exp).unwrap() - claims.exp
        };

        // Allow 10 seconds of variance for test execution time
        assert!(exp_diff < 10, "Token expiration not set to 24 hours");
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token_rejected() {
        let result = validate_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "123";
        let token1 = create_token(user_id).expect("Failed to create token 1");

        // Small delay to ensure different timestamp (at least 1 second for Unix timestamp)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different iat timestamps
        assert_ne!(token1, token2);

        // Both should validate successfully
        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");

        assert_eq!(claims1.sub, user_id);
        assert_eq!(claims2.sub, user_id);
    }

    #[test]
    fn test_empty_user_id_handled() {
        let user_id = "";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id_handled() {
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@email.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }
}
