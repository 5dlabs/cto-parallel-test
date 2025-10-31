pub mod models;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: u64,    // Expiration time
}

/// JWT secret key (in production, this should come from environment variables)
const JWT_SECRET: &str = "your-secret-key-change-in-production";

/// Creates a JWT token for a given user ID
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// Result containing the JWT token string or an error
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Panics
/// Panics if the system clock goes backwards (extremely rare)
#[allow(clippy::disallowed_methods)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 3600; // Token valid for 1 hour

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// Validates a JWT token and returns the claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// Result containing the Claims or an error
///
/// # Errors
/// Returns an error if token is invalid, expired, or signature verification fails
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");
        assert!(!token.is_empty());

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
