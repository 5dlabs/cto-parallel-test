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
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if the JWT encoding fails
///
/// # Panics
/// Panics if system time is before the UNIX epoch (highly unlikely in practice)
#[allow(clippy::disallowed_methods)] // Test project uses SystemTime::now for simplicity
#[allow(clippy::cast_possible_truncation)] // Timestamp values are well within usize range
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

    // Note: Using hardcoded secret for test/demo purposes only
    // In production, this should be loaded from environment variables
    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and returns the claims if valid
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if the token is invalid, expired, or has an invalid signature
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // Note: Using hardcoded secret for test/demo purposes only
    // In production, this should be loaded from environment variables
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
        let result = validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // Test code
    #[allow(clippy::cast_possible_truncation)] // Test code
    fn test_token_contains_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
    }
}
