use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Create a JWT token for a user with 24-hour expiration
///
/// # Errors
///
/// Returns an error if JWT encoding fails
///
/// # Panics
///
/// Panics if system time is before UNIX epoch (should never happen on modern systems)
#[allow(clippy::cast_possible_truncation)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
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

/// Validate a JWT token and extract claims
///
/// # Errors
///
/// Returns an error if:
/// - The token is malformed
/// - The signature is invalid
/// - The token has expired
/// - Required claims are missing
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
    fn test_token_creation() {
        let user_id = "123";
        let token = create_token(user_id);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_token_validation_with_valid_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token);
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn test_token_contains_correct_claims() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "123");
        assert!(claims.exp > claims.iat);

        // Check expiration is approximately 24 hours in future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expected_exp = now + 86400; // 24 hours

        // Allow 10 second variance
        assert!((claims.exp as i64 - expected_exp as i64).abs() < 10);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "123";
        let token1 = create_token(user_id).unwrap();

        // Wait long enough to ensure different timestamps (1 second minimum for timestamp change)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).unwrap();

        // Tokens should be different due to different iat timestamps
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_empty_user_id_handled() {
        let token = create_token("");
        assert!(token.is_ok());

        let claims = validate_token(&token.unwrap()).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }
}
