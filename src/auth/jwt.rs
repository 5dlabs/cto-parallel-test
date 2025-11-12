use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Clock trait for testable time operations
pub trait Clock {
    fn now(&self) -> u64;
}

/// Production clock implementation using system time
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        #[allow(clippy::disallowed_methods)]
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

/// Create a JWT token for the given user ID with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if JWT encoding fails (rare, usually indicates invalid configuration)
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_clock(user_id, &SystemClock)
}

/// Create a JWT token with a custom clock (for testing)
///
/// # Errors
/// Returns an error if JWT encoding fails
#[allow(clippy::cast_possible_truncation)] // JWT standard uses usize for timestamps; u64->usize cast is acceptable for years to come
pub fn create_token_with_clock(
    user_id: &str,
    clock: &dyn Clock,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = clock.now();
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

/// Validate a JWT token and extract claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if:
/// - Token is malformed or invalid
/// - Token signature verification fails
/// - Token has expired
/// - Token was signed with a different secret
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

    /// Mock clock for testing
    struct MockClock {
        timestamp: u64,
    }

    impl Clock for MockClock {
        fn now(&self) -> u64 {
            self.timestamp
        }
    }

    #[test]
    fn test_token_creation() {
        let user_id = "user_123";
        let token = create_token(user_id).expect("Failed to create token");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_token_validation() {
        let user_id = "user_123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)] // Test code; timestamps fit in usize for testing purposes
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        // Use a recent timestamp to avoid expiration issues
        let now = SystemClock.now();
        let mock_clock = MockClock { timestamp: now };

        let token = create_token_with_clock(user_id, &mock_clock).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.iat, now as usize);
        assert_eq!(claims.exp, (now + 24 * 3600) as usize); // 24 hours later
    }

    #[test]
    fn test_invalid_token_is_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)] // Test code; timestamps fit in usize for testing purposes
    fn test_token_expiration_is_24_hours() {
        let user_id = "user_789";
        // Use a recent timestamp to avoid expiration issues
        let now = SystemClock.now();
        let mock_clock = MockClock { timestamp: now };

        let token = create_token_with_clock(user_id, &mock_clock).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let expected_exp = now + 86400; // 24 hours in seconds
        assert_eq!(claims.exp, expected_exp as usize);
    }

    #[test]
    fn test_different_users_get_unique_tokens() {
        let token1 = create_token("user1").expect("Failed to create token");
        let token2 = create_token("user2").expect("Failed to create token");

        assert_ne!(token1, token2);

        let claims1 = validate_token(&token1).expect("Failed to validate token");
        let claims2 = validate_token(&token2).expect("Failed to validate token");

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
    }

    #[test]
    fn test_empty_user_id_is_handled() {
        let token = create_token("").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@email.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }
}
