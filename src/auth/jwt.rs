use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Clock trait for testability
trait Clock {
    fn now_unix_timestamp(&self) -> usize;
}

/// Real system clock implementation
struct SystemClock;

impl Clock for SystemClock {
    // This is the Clock abstraction that clippy.toml requires.
    // `SystemTime::now()` is isolated here for testability - production code uses the Clock trait.
    #[allow(clippy::disallowed_methods)]
    fn now_unix_timestamp(&self) -> usize {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        usize::try_from(duration.as_secs()).expect("Unix timestamp overflow")
    }
}

/// Creates a JWT token for the given user ID with 24-hour expiration.
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// A JWT token string if successful, or a jsonwebtoken error
///
/// # Errors
///
/// Returns an error if the token encoding fails.
///
/// # Panics
///
/// Panics if the system time is before the UNIX epoch or if the timestamp overflows `usize`.
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("user123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_clock(user_id, &SystemClock)
}

fn create_token_with_clock(
    user_id: &str,
    clock: &dyn Clock,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = clock.now_unix_timestamp();
    let expiration = now + 86400; // 24 hours in seconds

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"test_secret_key"),
    )
}

/// Validates a JWT token and returns the claims if valid.
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The decoded claims if the token is valid, or a jsonwebtoken error
///
/// # Errors
///
/// Returns an error if the token is malformed, has an invalid signature, or has expired.
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("user123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"test_secret_key"),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClock {
        timestamp: usize,
    }

    impl Clock for MockClock {
        fn now_unix_timestamp(&self) -> usize {
            self.timestamp
        }
    }

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).expect("Token creation failed");
        let claims = validate_token(&token).expect("Token validation failed");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_mock_clock() {
        // Use a far-future timestamp to avoid expiration
        let mock_time = 2_000_000_000; // Year 2033
        let mock_clock = MockClock {
            timestamp: mock_time,
        };

        let user_id = "test_user";
        let token = create_token_with_clock(user_id, &mock_clock).expect("Token creation failed");

        // Note: `validate_token` uses real system time for expiration check,
        // so we just verify the token was created correctly
        let claims = validate_token(&token).expect("Token validation failed");

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.iat, mock_time);
        assert_eq!(claims.exp, mock_time + 86400);
    }

    #[test]
    fn test_token_contains_correct_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).expect("Token creation failed");
        let claims = validate_token(&token).expect("Token validation failed");

        let now = SystemClock.now_unix_timestamp();
        let expected_exp = now + 86400;

        // Use `abs_diff` for calculating absolute difference
        let diff = claims.exp.abs_diff(expected_exp);

        // Allow 5 seconds tolerance for test execution time
        assert!(diff <= 5, "Expiration time difference too large: {diff}");
    }
}
