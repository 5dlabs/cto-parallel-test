use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for the given user ID with a 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token's subject claim
///
/// # Returns
/// A Result containing the encoded JWT token string or an error
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Panics
/// Panics if the system time is before `UNIX_EPOCH` (shouldn't happen in practice)
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

/// Validates a JWT token and returns the claims if valid
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// A Result containing the decoded Claims or an error
///
/// # Errors
/// Returns an error if token validation fails (invalid signature, expired, malformed)
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
    fn test_jwt_contains_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        #[allow(clippy::cast_possible_truncation)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Expiration should be approximately 24 hours from now (allowing 10 second buffer)
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86410);
    }

    #[test]
    fn test_invalid_token_fails() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
