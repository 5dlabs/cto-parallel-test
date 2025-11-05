use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Creates a JWT token for the given user ID
/// The token expires after 24 hours
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The JWT token or an error
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Panics
/// Panics if system time is before Unix epoch (extremely unlikely in practice)
///
/// # Examples
/// ```
/// use ecommerce_api::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime::now is appropriate for JWT timestamps
#[allow(clippy::cast_possible_truncation)] // usize truncation acceptable for timestamps
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
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

/// Validates a JWT token and returns the claims if valid
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if:
/// - The token is malformed or invalid
/// - The token signature is invalid
/// - The token has expired
/// - The token algorithm doesn't match expectations
///
/// # Examples
/// ```
/// use ecommerce_api::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user_123");
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
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);

        // Check expiration is set
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);

        // Check that expiration is approximately 24 hours from now
        let expected_exp = claims.iat + 86400;
        #[allow(clippy::cast_possible_wrap)] // Acceptable for test comparisons
        let diff = (claims.exp as i64 - expected_exp as i64).abs();
        assert!(diff < 10);
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
        assert!(
            claims.exp > claims.iat,
            "Expiration should be after issued time"
        );
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "same_user";

        let token1 = create_token(user_id).expect("Failed to create token 1");
        let claims1 = validate_token(&token1).expect("Failed to validate token 1");

        // Small delay to ensure different timestamps (at least 1 second)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).expect("Failed to create token 2");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");

        // Claims should have different issued-at times
        assert_ne!(claims1.iat, claims2.iat, "Issued-at times should differ");

        // But both should have the same subject
        assert_eq!(claims1.sub, user_id);
        assert_eq!(claims2.sub, user_id);
    }
}
