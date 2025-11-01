use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Create a JWT token for a given user ID with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user ID to include in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// * Returns an error if token encoding fails
///
/// # Panics
/// * Panics if system time is before UNIX epoch (should never happen in practice)
///
/// # Security
/// * Token expires after 24 hours
/// * Secret key is loaded from `JWT_SECRET` environment variable
/// * Falls back to development key if `JWT_SECRET` is not set (NOT for production)
#[allow(clippy::cast_possible_truncation)] // u64 timestamp fits in usize on all practical platforms
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now as usize,
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
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// * Returns an error if the token is invalid, expired, or tampered with
///
/// # Security
/// * Validates token signature
/// * Checks token expiration
/// * Rejects tampered or invalid tokens
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
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token);
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);

        // Verify expiration is approximately 24 hours in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_secs() as usize;
        let expected_exp = now + 86400; // 24 hours
        let exp_diff = (claims.exp as i64 - expected_exp as i64).abs();
        assert!(
            exp_diff < 10,
            "Expiration time should be ~24 hours from now"
        );
    }

    #[test]
    fn test_invalid_token_is_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_token_is_rejected() {
        let user_id = "123";
        let mut token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by changing a character
        token.push('x');

        let result = validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "123";

        let token1 = create_token(user_id).expect("Failed to create token1");
        // Sleep for 1 second to ensure different iat timestamp
        std::thread::sleep(std::time::Duration::from_secs(1));
        let token2 = create_token(user_id).expect("Failed to create token2");

        // Tokens should be different due to different iat timestamps
        assert_ne!(token1, token2);

        // But both should be valid
        let claims1 = validate_token(&token1).expect("Failed to validate token1");
        let claims2 = validate_token(&token2).expect("Failed to validate token2");

        assert_eq!(claims1.sub, user_id);
        assert_eq!(claims2.sub, user_id);
    }
}
