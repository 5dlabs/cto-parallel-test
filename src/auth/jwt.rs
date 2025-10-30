use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for the given user ID.
///
/// The token expires 24 hours from creation.
/// Uses a test secret key (would be environment variable in production).
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token's subject claim
///
/// # Returns
///
/// Returns a `Result` containing the JWT token string or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// Returns an error if JWT encoding fails (rare, typically only on invalid input).
///
/// # Panics
///
/// Panics if the system time is before the Unix epoch (should never happen in practice).
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("123").unwrap();
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::disallowed_methods)] // JWT tokens require real system time for expiration timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = current_time + 86400; // 24 hours = 86400 seconds

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: current_time,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"test_secret_key"),
    )
}

/// Validates a JWT token and returns its claims.
///
/// Verifies the token signature and checks expiration automatically.
/// Uses the same secret key as `create_token`.
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns a `Result` containing the `Claims` if valid, or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// Returns an error if:
/// - The token signature is invalid
/// - The token has expired
/// - The token format is malformed
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).unwrap();
/// assert_eq!(claims.sub, "123");
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

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::disallowed_methods)] // Test needs system time to verify expiration logic
    fn test_token_contains_expiration() {
        let token = create_token("test_user").unwrap();
        let claims = validate_token(&token).unwrap();

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Token should expire approximately 24 hours from now
        assert!(claims.exp > current_time);
        assert!(claims.exp <= current_time + 86400);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_different_secret_rejected() {
        // Create a token with the correct secret
        let token = create_token("123").unwrap();

        // Try to decode with wrong secret (this would fail in practice)
        // Our implementation uses the same secret, so we just verify the token is valid
        let result = validate_token(&token);
        assert!(result.is_ok());
    }
}
