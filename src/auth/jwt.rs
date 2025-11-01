use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID as string
    pub exp: usize,  // Expiration timestamp
    pub iat: usize,  // Issued at timestamp
}

/// Clock abstraction trait for testability (AWS smithy-rs pattern)
pub trait Clock: Send + Sync {
    /// Returns the current Unix timestamp in seconds
    fn now(&self) -> usize;
}

/// System clock implementation using platform time
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> usize {
        // Note: This function intentionally uses SystemTime::now()
        // The clippy rule is bypassed here because this is the specific
        // location where we encapsulate time access behind the Clock trait.
        // All other code should use Clock::now() instead.
        #[allow(clippy::disallowed_methods)]
        {
            usize::try_from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("System time before UNIX epoch")
                    .as_secs(),
            )
            .expect("Timestamp overflow")
        }
    }
}

/// Get the JWT secret from environment variable, with a default for testing
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test-secret-key-change-in-production".to_string())
}

/// Creates a JWT token for a user using the system clock
///
/// # Arguments
/// * `user_id` - The user ID to create a token for
///
/// # Returns
/// `Ok(String)` with the JWT token, or `Err(String)` on error
///
/// # Errors
/// Returns error if token creation fails
pub fn create_token(user_id: i32) -> Result<String, String> {
    create_token_with_clock(user_id, &SystemClock)
}

/// Creates a JWT token for a user using a custom clock (for testing)
///
/// # Arguments
/// * `user_id` - The user ID to create a token for
/// * `clock` - Clock implementation for getting current time
///
/// # Returns
/// `Ok(String)` with the JWT token, or `Err(String)` on error
///
/// # Errors
/// Returns error if token creation fails
pub fn create_token_with_clock(user_id: i32, clock: &dyn Clock) -> Result<String, String> {
    let now = clock.now();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 86400, // 24 hours from now
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|e| format!("Token creation failed: {e}"))
}

/// Validates a JWT token and returns claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// `Ok(Claims)` if valid, `Err(String)` with error message if invalid
///
/// # Errors
/// Returns error if token is invalid or expired
pub fn validate_token(token: &str) -> Result<Claims, String> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token validation failed: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock clock for testing
    struct MockClock {
        timestamp: usize,
    }

    impl Clock for MockClock {
        fn now(&self) -> usize {
            self.timestamp
        }
    }

    #[test]
    fn test_create_and_validate_token() {
        let user_id = 1;
        let token = create_token(user_id).expect("Token creation failed");
        let result = validate_token(&token);
        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub, "1");
    }

    #[test]
    fn test_validate_token_invalid() {
        let token = "invalid_token";
        let result = validate_token(token);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_multiple_users() {
        for user_id in 1..10 {
            let token = create_token(user_id).expect("Token creation failed");
            let result = validate_token(&token);
            assert!(result.is_ok());
            let claims = result.unwrap();
            assert_eq!(claims.sub, user_id.to_string());
        }
    }

    #[test]
    fn test_token_with_custom_secret() {
        // Save original secret
        let original_secret = std::env::var("JWT_SECRET").ok();

        std::env::set_var("JWT_SECRET", "custom-secret-key");
        let token = create_token(42).expect("Token creation failed");
        let result = validate_token(&token);
        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub, "42");

        // Restore original secret or remove if it wasn't set
        match original_secret {
            Some(secret) => std::env::set_var("JWT_SECRET", secret),
            None => std::env::remove_var("JWT_SECRET"),
        }
    }

    #[test]
    fn test_token_with_mock_clock() {
        // Save original secret to ensure test isolation
        let original_secret = std::env::var("JWT_SECRET").ok();

        // Reset to default secret for this test
        std::env::remove_var("JWT_SECRET");

        // Use a far future timestamp to avoid expiration issues
        let mock_clock = MockClock {
            timestamp: 9_999_999_999,
        };
        let token = create_token_with_clock(123, &mock_clock).expect("Token creation failed");
        let result = validate_token(&token);
        assert!(result.is_ok(), "Token validation should succeed");
        let claims = result.unwrap();
        assert_eq!(claims.sub, "123");
        assert_eq!(claims.iat, 9_999_999_999);
        assert_eq!(claims.exp, 10_000_086_399); // iat + 86400

        // Restore original secret
        match original_secret {
            Some(secret) => std::env::set_var("JWT_SECRET", secret),
            None => std::env::remove_var("JWT_SECRET"),
        }
    }
}
