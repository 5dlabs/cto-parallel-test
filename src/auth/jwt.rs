use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims structure containing user identification and token metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for a given user ID.
///
/// The token expires 24 hours after creation.
///
/// # Errors
///
/// Returns an error if token encoding fails.
#[allow(clippy::cast_possible_truncation, clippy::disallowed_methods)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as usize;

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

/// Validates a JWT token and returns the claims if valid.
///
/// # Errors
///
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
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::disallowed_methods)]
    fn test_token_contains_expiration() {
        let user_id = "456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(claims.exp > now);
        assert!(claims.iat <= now + 1); // Allow for 1 second of execution time
    }

    #[test]
    fn test_invalid_token_fails() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
