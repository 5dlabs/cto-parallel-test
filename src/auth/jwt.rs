use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
/// Token expires after 24 hours
///
/// # Errors
///
/// Returns `jsonwebtoken::errors::Error` if token encoding fails
///
/// # Panics
///
/// Panics if system time is before UNIX epoch (extremely unlikely in practice)
#[allow(clippy::cast_possible_truncation)] // usize is large enough for timestamps in practice
#[allow(clippy::disallowed_methods)] // SystemTime::now is acceptable for JWT expiry in this test project
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
/// # Errors
///
/// Returns `jsonwebtoken::errors::Error` if:
/// - Token format is invalid
/// - Token signature is invalid
/// - Token has expired
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
    #[allow(clippy::cast_possible_truncation)] // usize is large enough for timestamps in tests
    #[allow(clippy::disallowed_methods)] // SystemTime::now is acceptable in tests
    fn test_token_contains_expiration() {
        let user_id = "456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Token should expire in approximately 24 hours (86400 seconds)
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
        assert!(claims.iat <= now);
    }
}
