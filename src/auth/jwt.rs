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
/// The token expires after 24 hours (86400 seconds).
///
/// # Arguments
/// * `user_id` - The user identifier to include in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if JWT encoding fails (e.g., invalid secret key format).
///
/// # Panics
/// Panics if the system clock is set before Unix epoch (should never happen in practice).
///
/// # Example
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("user123").unwrap();
/// assert!(!token.is_empty());
/// ```
// Allow SystemTime::now() for this test project - production would use a Clock abstraction
#[allow(clippy::disallowed_methods)]
#[allow(clippy::cast_possible_truncation)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 86400, // 24 hours from now
        iat: now,
    };

    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and returns its claims.
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if:
/// - Token is malformed or has invalid format
/// - Token signature is invalid
/// - Token has expired
/// - Token validation fails for any other reason
///
/// # Example
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("user123").unwrap();
/// let claims = validate_token(&token).unwrap();
/// assert_eq!(claims.sub, "user123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
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
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    #[allow(clippy::cast_possible_truncation)]
    fn test_token_contains_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Token should expire in approximately 24 hours
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
    }
}
