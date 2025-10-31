use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for a given user ID
///
/// # Arguments
/// * `user_id` - The user ID to include in the token's subject claim
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token
/// * `Err` - jsonwebtoken encoding error
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Panics
/// Panics if system time is before `UNIX_EPOCH` (extremely unlikely)
///
/// # Security Note
/// Uses a hardcoded test secret key - production should use environment variable
#[allow(clippy::cast_possible_truncation)]
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

/// Validates a JWT token and returns the claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Ok(Claims)` - The decoded claims if token is valid
/// * `Err` - jsonwebtoken decoding/validation error
///
/// # Errors
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
///
/// # Security Note
/// Uses the same hardcoded test secret key for validation
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
    #[allow(clippy::cast_possible_truncation)]
    fn test_token_contains_expiration() {
        let token = create_token("test_user").unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Token should expire in approximately 24 hours
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
    }

    #[test]
    fn test_invalid_token_fails() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
