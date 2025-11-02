use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Creates a JWT token for a given user ID.
/// The token expires after 24 hours.
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// A JWT token string or an error if token creation fails
///
/// # Errors
///
/// Returns an error if JWT encoding fails
///
/// # Panics
///
/// Panics if system time is before UNIX epoch (should never happen in practice)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("123").unwrap();
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime::now needed for JWT expiration
#[allow(clippy::cast_possible_truncation)] // JWT spec uses numeric timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).unwrap_or(usize::MAX),
        iat: usize::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
        .unwrap_or(usize::MAX),
    };

    // In production, load from environment variable
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and extracts the claims.
///
/// # Arguments
///
/// * `token` - The JWT token to validate
///
/// # Returns
///
/// The decoded claims if the token is valid, or an error if validation fails
///
/// # Errors
///
/// Returns an error if token is invalid, expired, or signature verification fails
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).unwrap();
/// assert_eq!(claims.sub, "123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_expiration_is_set() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        // Check expiration is set and is in the future
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);

        // Check expiration is approximately 24 hours in the future (within 10 seconds tolerance)
        let expected_exp = claims.iat + 24 * 3600;
        let exp_diff = claims.exp.abs_diff(expected_exp);
        assert!(
            exp_diff < 10,
            "Expiration should be ~24 hours from issue time"
        );
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_different_users_have_different_tokens() {
        let token1 = create_token("user1").unwrap();
        let token2 = create_token("user2").unwrap();
        assert_ne!(token1, token2);

        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
    }

    #[test]
    fn test_same_user_different_timestamps() {
        let token1 = create_token("user1").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token("user1").unwrap();

        // Tokens should be different due to different timestamps
        assert_ne!(token1, token2);

        // But both should validate to the same user
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();
        assert_eq!(claims1.sub, claims2.sub);
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com!#$";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_very_long_user_id() {
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }
}
