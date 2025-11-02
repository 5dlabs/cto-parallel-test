//! JWT token creation and validation.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims structure containing user information and token metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID as string)
    pub sub: String,
    /// Issued at timestamp
    pub iat: u64,
    /// Expiration timestamp (24 hours from issue)
    pub exp: u64,
}

/// Creates a JWT token for a user ID.
///
/// # Arguments
///
/// * `user_id` - The unique user identifier
///
/// # Returns
///
/// A JWT token string valid for 24 hours
///
/// # Errors
///
/// Returns an error if token encoding fails
///
/// # Panics
///
/// Panics if system time is before UNIX epoch (should never happen on modern systems)
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token(1).expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime needed for JWT token timestamps
pub fn create_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + 24 * 3600, // 24 hours
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and extracts the claims.
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The decoded `Claims` if the token is valid
///
/// # Errors
///
/// Returns an error if the token is invalid, expired, or malformed
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token(1).expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "1");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token(1).expect("Failed to create token");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_valid_token() {
        let token = create_token(42).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "42");
    }

    #[test]
    fn test_validate_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_user_id() {
        let user_id = 123;
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // SystemTime needed for test validation
    #[allow(clippy::cast_possible_wrap)] // Test-only code, wrapping won't occur in practice
    fn test_token_expiration_set() {
        let token = create_token(1).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Expiration should be approximately 24 hours from now (allow 10 second tolerance)
        assert!((claims.exp as i64 - now as i64 - 24 * 3600).abs() < 10);
    }
}
