//! JWT Token Handling
//!
//! This module provides functions for creating and validating JWT tokens.
//! Tokens are signed with a secret key and include standard claims.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure
///
/// Contains standard JWT claims for user authentication.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Create a JWT token for a user
///
/// # Arguments
/// * `user_id` - The user's unique identifier
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token
///
/// # Errors
/// Returns an error if JWT encoding fails (extremely rare, usually indicates invalid secret key)
///
/// # Panics
/// Panics if system clock has been set backwards before Unix epoch (essentially impossible in practice)
///
/// # Security
/// - Tokens expire after 24 hours
/// - Secret key is loaded from `JWT_SECRET` environment variable
/// - Falls back to development secret if `JWT_SECRET` is not set (not for production!)
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// println!("Token: {}", token);
/// ```
// Allow SystemTime::now() for JWT - real wall-clock time is required for token expiration
#[allow(clippy::disallowed_methods)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = current_time + 24 * 3600; // 24 hours from now

    // Casting is safe: timestamps in the next 2000+ years fit in usize on all platforms
    #[allow(clippy::cast_possible_truncation)]
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: current_time as usize,
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

/// Validate a JWT token and extract claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Ok(Claims)` - The decoded claims if token is valid
///
/// # Errors
/// Returns an error if:
/// - Token is malformed or has invalid format
/// - Token signature is invalid
/// - Token has expired
/// - Token was signed with a different secret key
///
/// # Security
/// - Validates token signature
/// - Checks token expiration
/// - Uses the same secret key as token creation
///
/// # Examples
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
        assert!(!token.is_empty());
        assert!(
            token.contains('.'),
            "JWT should have 3 parts separated by dots"
        );
    }

    #[test]
    fn test_token_validation() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // Test needs real time
    #[allow(clippy::cast_possible_truncation)] // Safe for timestamps
    #[allow(clippy::cast_possible_wrap)] // Safe for time diff calculation
    fn test_token_claims() {
        let user_id = "test_user_789";
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Check that issued_at is within reasonable range
        assert!(claims.iat >= before as usize);
        assert!(claims.iat <= after as usize);

        // Check that expiration is approximately 24 hours in the future
        let expected_exp = claims.iat + 24 * 3600;
        assert!((claims.exp as i64 - expected_exp as i64).abs() < 10);
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err(), "Invalid token should be rejected");
    }

    #[test]
    fn test_tampered_token() {
        let user_id = "test_user_999";
        let token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by modifying a character
        let mut tampered = token.clone();
        if let Some(ch) = tampered.chars().nth(10) {
            let replacement = if ch == 'a' { 'b' } else { 'a' };
            tampered = tampered
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 10 { replacement } else { c })
                .collect();
        }

        let result = validate_token(&tampered);
        assert!(result.is_err(), "Tampered token should be rejected");
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "test_user_same";

        let token1 = create_token(user_id).expect("Failed to create token 1");
        // Sleep long enough to ensure different timestamps (1 second to be safe)
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different timestamps
        assert_ne!(
            token1, token2,
            "Different tokens should be generated even for same user"
        );
    }
}
