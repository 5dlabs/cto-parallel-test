//! JWT token creation and validation
//!
//! Provides functions for creating and validating JSON Web Tokens (JWT)
//! with 24-hour expiration using configurable secret keys.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims structure following RFC 7519
///
/// Contains standard JWT claims for authentication:
/// - `sub`: Subject (user ID)
/// - `exp`: Expiration time (Unix timestamp)
/// - `iat`: Issued at time (Unix timestamp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - User ID
    pub sub: String,
    /// Expiration time - Unix timestamp
    pub exp: usize,
    /// Issued at - Unix timestamp
    pub iat: usize,
}

/// Create a JWT token for a user ID with 24-hour expiration
///
/// # Arguments
///
/// * `user_id` - The user ID to embed in the token
///
/// # Returns
///
/// A JWT token string on success.
///
/// # Errors
///
/// Returns an error if token encoding fails (extremely rare, would indicate
/// a critical system issue).
///
/// # Panics
///
/// Panics if the system time is before the Unix epoch (January 1, 1970) or if
/// timestamp conversion to `usize` fails (would only occur on systems with
/// 32-bit `usize` in the distant future, year 2038+).
///
/// # Security
///
/// - Tokens expire after 24 hours
/// - Secret key loaded from `JWT_SECRET` environment variable
/// - Falls back to development key if not set (production should always set this)
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime::now() is acceptable for JWT exp/iat claims
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before Unix epoch");

    let expiration = now.as_secs() + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).expect("Expiration timestamp too large"),
        iat: usize::try_from(now.as_secs()).expect("Current timestamp too large"),
    };

    // Load JWT secret from environment, with fallback for development
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validate a JWT token and extract claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The decoded claims if validation succeeds.
///
/// # Errors
///
/// Returns an error if:
/// - Token is malformed
/// - Token signature is invalid
/// - Token has expired
/// - Token was signed with a different secret
///
/// # Security
///
/// - Validates token signature
/// - Checks expiration timestamp
/// - Uses the same secret key as token creation
///
/// # Examples
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
    fn test_token_creation() {
        let user_id = "test_user_123";
        let token = create_token(user_id).expect("Failed to create token");

        // Token should not be empty
        assert!(!token.is_empty());

        // Token should have 3 parts separated by dots (JWT format)
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "JWT should have 3 parts: header.payload.signature"
        );
    }

    #[test]
    fn test_token_validation_success() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify user ID matches
        assert_eq!(claims.sub, user_id);

        // Verify timestamps are reasonable
        assert!(
            claims.exp > claims.iat,
            "Expiration should be after issued time"
        );

        // Verify expiration is approximately 24 hours from issued time
        let expected_duration = 24_i64 * 3600; // 24 hours in seconds
        let actual_duration = i64::try_from(claims.exp - claims.iat).unwrap();
        assert!(
            (actual_duration - expected_duration).abs() < 10,
            "Token should expire in approximately 24 hours"
        );
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);

        assert!(result.is_err(), "Invalid token should be rejected");
    }

    #[test]
    fn test_tampered_token_rejected() {
        let user_id = "test_user_789";
        let token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered);
        assert!(result.is_err(), "Tampered token should be rejected");
    }

    #[test]
    fn test_different_users_have_different_tokens() {
        let token1 = create_token("user_1").expect("Failed to create token 1");
        let token2 = create_token("user_2").expect("Failed to create token 2");

        assert_ne!(
            token1, token2,
            "Different users should have different tokens"
        );
    }

    #[test]
    fn test_same_user_different_timestamps() {
        let user_id = "test_user_same";
        let token1 = create_token(user_id).expect("Failed to create token 1");

        // Delay to ensure different timestamp (at least 1 second since tokens use second precision)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different timestamps
        assert_ne!(
            token1, token2,
            "Same user at different times should have different tokens"
        );

        // But both should validate and have the same user ID
        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");
        assert_eq!(claims1.sub, claims2.sub);
    }

    #[test]
    fn test_claims_fields() {
        let user_id = "claims_test_user";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify all required fields are present
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0, "Expiration should be set");
        assert!(claims.iat > 0, "Issued at should be set");

        // Verify expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs();
        assert!(
            claims.exp as u64 > now,
            "Token should not be expired immediately"
        );
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").expect("Failed to create token with empty user ID");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "", "Empty user ID should be preserved");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com|123!#$%";
        let token = create_token(user_id).expect("Failed to create token with special characters");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(
            claims.sub, user_id,
            "Special characters should be preserved"
        );
    }
}
