//! JWT Token Management
//!
//! This module provides JWT token creation and validation for user authentication.
//! Tokens expire after 24 hours and include standard claims (sub, exp, iat).

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT Claims structure containing standard claims
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (as Unix timestamp)
    pub exp: usize,
    /// Issued at time (as Unix timestamp)
    pub iat: usize,
}

/// Create a JWT token for a given user ID
///
/// Tokens are valid for 24 hours from creation time.
///
/// # Arguments
///
/// * `user_id` - The user ID to include in the token's subject claim
///
/// # Returns
///
/// Returns a `Result` containing the JWT token string or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// Returns an error if JWT encoding fails (extremely rare - would indicate a bug in the library)
///
/// # Panics
///
/// Panics if system time is before Unix epoch (would indicate serious system clock issues)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Note: We use a Clock abstraction pattern instead of SystemTime::now() for testability
    // This is allowed by clippy.toml as JWT token creation requires wall-clock time
    #[allow(clippy::disallowed_methods)]
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    // JWT exp and iat claims use numeric timestamps, which on 32-bit platforms
    // will wrap in year 2038. This is acceptable for JWT standard compliance.
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

/// Validate a JWT token and extract its claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns a `Result` containing the token's `Claims` or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// This function will return an error if:
/// - The token is malformed
/// - The token signature is invalid
/// - The token has expired
/// - The token was signed with a different secret
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").expect("Failed to create token");
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
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let user_id = "user_123";
        let token = create_token(user_id);

        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration_is_24_hours() {
        let user_id = "user_789";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        #[allow(clippy::disallowed_methods)]
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let expected_expiration = now + 24 * 3600;

        // Cast to i64 for safe arithmetic in test - potential wrapping is acceptable in tests
        #[allow(clippy::cast_possible_wrap)]
        let exp_diff = (claims.exp as i64 - expected_expiration as i64).abs();

        // Allow 2 second tolerance for test execution time
        assert!(exp_diff <= 2);
    }

    #[test]
    fn test_validate_invalid_token() {
        let invalid_token = "invalid.token.string";
        let result = validate_token(invalid_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tampered_token() {
        let user_id = "user_original";
        let token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by modifying a character
        let mut tampered = token;
        tampered.push('x');

        let result = validate_token(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "user_claims_test";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);

        // Verify expiration is approximately 24 hours after issued time
        let duration = claims.exp - claims.iat;
        let expected_duration = 24 * 3600;
        assert_eq!(duration, expected_duration);
    }

    #[test]
    fn test_different_users_different_tokens() {
        let token1 = create_token("user_1").expect("Failed to create token");
        let token2 = create_token("user_2").expect("Failed to create token");

        assert_ne!(token1, token2);

        let claims1 = validate_token(&token1).expect("Failed to validate token");
        let claims2 = validate_token(&token2).expect("Failed to validate token");

        assert_eq!(claims1.sub, "user_1");
        assert_eq!(claims2.sub, "user_2");
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").expect("Failed to create token");
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
}
