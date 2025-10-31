use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (Unix timestamp)
    pub iat: usize,  // Issued at (Unix timestamp)
}

/// Creates a JWT token for the given user ID
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
///
/// Returns an error if JWT encoding fails
///
/// # Panics
///
/// Panics if system time is before `UNIX_EPOCH` (should never happen on modern systems)
#[allow(clippy::disallowed_methods)] // Using SystemTime::now() for JWT timestamp generation
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::cast_possible_truncation)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours from now

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

/// Validates a JWT token and returns the claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
///
/// Returns an error if the token is invalid, expired, or has an invalid signature
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
    fn test_jwt_token_contains_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        // Verify expiration is set to 24 hours from now
        #[allow(clippy::cast_possible_truncation, clippy::disallowed_methods)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400); // Should be within 24 hours
        assert_eq!(claims.iat, now); // Issued at should be now
    }

    #[test]
    fn test_invalid_token_fails() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
