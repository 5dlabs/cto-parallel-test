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
/// # Arguments
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
/// A JWT token string valid for 24 hours
///
/// # Errors
/// Returns an error if the token encoding fails
///
/// # Panics
/// Panics if the system time is before the UNIX epoch or if casting to usize fails
#[allow(clippy::disallowed_methods)] // SystemTime::now needed for JWT timestamp
#[allow(clippy::cast_possible_truncation)] // usize sufficient for timestamp on modern systems
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

/// Validates a JWT token and returns its claims.
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// The decoded claims if the token is valid
///
/// # Errors
/// Returns an error if:
/// - The token signature is invalid
/// - The token has expired
/// - The token format is malformed
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
        assert!(claims.exp > claims.iat);
        // Verify expiration is approximately 24 hours from now (86400 seconds)
        assert_eq!(claims.exp - claims.iat, 86400);
    }

    #[test]
    fn test_jwt_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_tampered_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        // Tamper with the token by modifying the signature part
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() == 3 {
            // Modify the signature (last part)
            let tampered = format!("{}.{}.tampered", parts[0], parts[1]);
            let result = validate_token(&tampered);
            assert!(result.is_err());
        }
    }
}
