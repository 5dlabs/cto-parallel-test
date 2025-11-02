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
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - The JWT token or an error
///
/// # Errors
///
/// Returns an error if JWT encoding fails (e.g., invalid secret key format)
///
/// # Panics
///
/// Panics if the system time is before the UNIX epoch (should never happen in practice)
///
/// # Security Notes
///
/// - Tokens expire after 24 hours
/// - Secret key should be loaded from `JWT_SECRET` environment variable in production
/// - Development fallback secret should only be used for testing
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    #[allow(clippy::cast_possible_truncation)]
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
///
/// * `token` - The JWT token to validate
///
/// # Returns
///
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Token was signed with a different secret
///
/// # Security Notes
///
/// - Validates token signature
/// - Checks token expiration
/// - Returns error for tampered or expired tokens
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
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        // Token should not be empty
        assert!(!token.is_empty());

        // Validate the token
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        // Check expiration is set correctly (should be ~24 hours in future)
        #[allow(clippy::cast_possible_truncation)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(claims.exp > now);
        assert!(claims.iat <= now + 1); // Allow 1 second tolerance

        let expected_exp = now + 24 * 3600;
        let exp_diff = claims.exp.abs_diff(expected_exp);
        assert!(exp_diff < 10, "Expiration should be ~24 hours from now");
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_empty_token() {
        assert!(validate_token("").is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_different_users_different_tokens() {
        let token1 = create_token("user1").unwrap();
        let token2 = create_token("user2").unwrap();

        // Tokens should be different
        assert_ne!(token1, token2);

        // But both should validate correctly
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

        // But both should validate with same user
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user1");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id() {
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }
}
