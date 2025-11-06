use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Creates a JWT token for the given user ID with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if token encoding fails (rare, indicates invalid input or system issues)
///
/// # Panics
/// Panics if the system time is before `UNIX_EPOCH` (should never happen on valid systems)
///
/// # Security Notes
/// - Tokens expire after 24 hours
/// - Uses `JWT_SECRET` environment variable, falls back to test secret in development
/// - Production deployments MUST set `JWT_SECRET` environment variable
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch")
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

/// Validates a JWT token and extracts the claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// This function will return an error if:
/// - The token is malformed
/// - The token signature is invalid
/// - The token has expired
/// - The token was signed with a different secret
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
    fn test_token_validation() {
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
        let user_id = "456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        // Check user ID
        assert_eq!(claims.sub, "456");

        // Check expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time")
            .as_secs();
        assert!(claims.exp > now as usize);

        // Check issued at is reasonable (within last minute)
        assert!(claims.iat <= now as usize);
        assert!(claims.iat > (now - 60) as usize);

        // Check expiration is ~24 hours from now
        let expected_exp = now + 86400;
        let exp_diff = (claims.exp as i64 - expected_exp as i64).abs();
        assert!(exp_diff < 10, "Expiration should be ~24 hours in future");
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        // Tamper with the token
        let mut tampered = token.clone();
        tampered.push('x');

        let result = validate_token(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_user_ids() {
        let token1 = create_token("user1").unwrap();
        let token2 = create_token("user2").unwrap();

        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_empty_user_id() {
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
