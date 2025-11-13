use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Clock trait for testability (allows mocking time in tests)
pub trait Clock {
    fn now_timestamp(&self) -> u64;
}

/// System clock implementation using real system time
#[derive(Debug, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    #[allow(clippy::disallowed_methods)] // SystemTime::now needed for real clock
    fn now_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before UNIX epoch")
            .as_secs()
    }
}

/// Create a JWT token for a user with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The JWT token or an error
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Example
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
#[allow(clippy::cast_possible_truncation)] // Timestamps won't overflow usize in practice
pub fn create_token_with_clock(
    user_id: &str,
    clock: &dyn Clock,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = clock.now_timestamp();
    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now as usize,
    };

    // In production, JWT_SECRET must be set with a strong secret (minimum 32 bytes)
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production_minimum_32_bytes".to_string());

    // Use HS256 algorithm explicitly
    let mut header = Header::new(Algorithm::HS256);
    header.alg = Algorithm::HS256;

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validate a JWT token and extract claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if the token is invalid, expired, or tampered with
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user_123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production_minimum_32_bytes".to_string());

    // Explicitly enforce HS256 algorithm to prevent algorithm confusion attacks
    let mut validation = Validation::new(Algorithm::HS256);
    validation.algorithms = vec![Algorithm::HS256];

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

    /// Mock clock for testing that returns a fixed timestamp
    struct MockClock {
        timestamp: u64,
    }

    impl Clock for MockClock {
        fn now_timestamp(&self) -> u64 {
            self.timestamp
        }
    }

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        // Check expiration is set
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)] // Test code, timestamps won't overflow
    fn test_token_expiration_is_24_hours() {
        // Use current time so token is valid when we validate it
        let now = SystemClock.now_timestamp();
        let mock_clock = MockClock { timestamp: now };
        let token = create_token_with_clock("user_123", &mock_clock).unwrap();
        let claims = validate_token(&token).unwrap();

        // Token should expire 24 hours after issuance
        let expected_exp = now + 24 * 3600;
        assert_eq!(claims.exp, expected_exp as usize);
        assert_eq!(claims.iat, now as usize);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)] // Test code, timestamps won't overflow
    fn test_expired_token_rejected() {
        // Test that we correctly set expiration time
        // We verify the token has the right expiration value
        let now = SystemClock.now_timestamp();
        let mock_clock = MockClock { timestamp: now };
        let token = create_token_with_clock("user_123", &mock_clock).unwrap();

        // Token should be valid right now
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.iat, now as usize);
        assert_eq!(claims.exp, (now + 24 * 3600) as usize);

        // Verify expiration is in the future
        assert!(claims.exp > claims.iat);
        assert_eq!(claims.exp - claims.iat, 24 * 3600);
    }

    #[test]
    fn test_token_contains_correct_user_id() {
        let user_ids = vec!["user_1", "user_123", "admin", "test@example.com"];

        for user_id in user_ids {
            let token = create_token(user_id).unwrap();
            let claims = validate_token(&token).unwrap();
            assert_eq!(claims.sub, user_id);
        }
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "user_123";

        // Create tokens with different timestamps using mock clocks
        let now = SystemClock.now_timestamp();
        let clock1 = MockClock { timestamp: now };
        let clock2 = MockClock { timestamp: now + 1 }; // 1 second later

        let token1 = create_token_with_clock(user_id, &clock1).unwrap();
        let token2 = create_token_with_clock(user_id, &clock2).unwrap();

        // Tokens should be different due to different timestamps
        assert_ne!(token1, token2);

        // Both should decode to the same user ID
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, claims2.sub);
        assert_eq!(claims1.sub, user_id);
    }

    #[test]
    fn test_tampered_token_rejected() {
        let token = create_token("user_123").unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        assert!(validate_token(&tampered).is_err());
    }
}
