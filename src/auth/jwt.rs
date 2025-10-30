use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (Unix timestamp)
    pub iat: usize,  // Issued at (Unix timestamp)
}

/// Creates a JWT token for the given user ID with 24-hour expiration
///
/// # Errors
///
/// Returns an error if JWT encoding fails
///
/// # Panics
///
/// Panics if system time is before `UNIX_EPOCH` (should never happen in practice)
#[allow(clippy::cast_possible_truncation)] // usize is sufficient for timestamps on all platforms
#[allow(clippy::disallowed_methods)] // SystemTime::now is required for JWT token generation
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours in seconds

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now,
    };

    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and returns the claims if valid
///
/// # Errors
///
/// Returns an error if token is invalid, expired, or has an invalid signature
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
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)] // Test code, usize is sufficient
    #[allow(clippy::disallowed_methods)] // Required for testing JWT timestamps
    fn test_token_contains_expiration() {
        let user_id = "456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Expiration should be approximately 24 hours from now
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
    }
}
