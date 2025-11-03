//! JWT token creation and validation
//!
//! Provides stateless authentication using JSON Web Tokens (JWT) with 24-hour expiration.
//! Tokens are signed with a secret key loaded from the `JWT_SECRET` environment variable.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims structure containing user identification and timing information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the specified user ID
///
/// The token expires 24 hours from creation and includes standard JWT claims:
/// - `sub`: User ID
/// - `exp`: Expiration timestamp (now + 24 hours)
/// - `iat`: Issued at timestamp (now)
///
/// # Arguments
///
/// * `user_id` - The user identifier to embed in the token
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - JWT token string on success
///
/// # Errors
///
/// Returns an error if JWT encoding fails (extremely rare, typically due to memory allocation failure).
///
/// # Panics
///
/// Panics if system time is before UNIX epoch (extremely rare, indicates system misconfiguration).
///
/// # Security
///
/// Uses the `JWT_SECRET` environment variable for signing. Falls back to a test secret
/// for development only. **Production environments must set `JWT_SECRET`**.
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
// JWT tokens require real wall-clock time for expiration
#[allow(clippy::disallowed_methods)]
#[allow(clippy::cast_possible_truncation)] // usize is appropriate for JWT timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch")
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
/// Verifies the token signature and expiration timestamp. Returns the decoded claims
/// if the token is valid and not expired.
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// * `Result<Claims, jsonwebtoken::errors::Error>` - Decoded claims on success
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Secret key doesn't match
///
/// # Examples
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
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // JWT tests require wall-clock time
    #[allow(clippy::cast_possible_wrap)] // Test uses usize timestamps, cast is safe for reasonable values
    #[allow(clippy::cast_possible_truncation)] // usize is appropriate for JWT timestamps
    fn test_token_contains_correct_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Token should expire approximately 24 hours from now (86400 seconds)
        let expected_exp = now + 86400;
        let exp_diff = (claims.exp as i64 - expected_exp as i64).abs();

        // Allow 10 second tolerance for test execution time
        assert!(exp_diff < 10, "Expiration time not within expected range");
    }

    #[test]
    fn test_validate_invalid_token() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_malformed_token() {
        let malformed_tokens = vec!["", "not_a_jwt", "a.b", "a.b.c.d"];

        for token in malformed_tokens {
            let result = validate_token(token);
            assert!(
                result.is_err(),
                "Expected error for malformed token: {token}"
            );
        }
    }

    #[test]
    fn test_different_users_get_different_tokens() {
        let token1 = create_token("user1").unwrap();
        let token2 = create_token("user2").unwrap();

        // Tokens should be different
        assert_ne!(token1, token2);

        // But both should be valid
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
    }

    #[test]
    fn test_same_user_different_times_different_tokens() {
        let token1 = create_token("user1").unwrap();
        // Sleep for at least 1 second to ensure different timestamps
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token("user1").unwrap();

        // Tokens should be different due to different timestamps
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_token_with_empty_user_id() {
        let token = create_token("").expect("Should handle empty user_id");
        let claims = validate_token(&token).expect("Should validate empty user_id token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_token_with_special_characters() {
        let user_ids = vec!["user@example.com", "user-123", "user_name", "user 123"];

        for user_id in user_ids {
            let token = create_token(user_id).expect("Should create token");
            let claims = validate_token(&token).expect("Should validate token");
            assert_eq!(claims.sub, user_id);
        }
    }
}
