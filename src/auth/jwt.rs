use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing standard JWT fields
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (as Unix timestamp)
    pub exp: usize,
    /// Issued at (as Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the specified user ID
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// Returns a `Result` containing the JWT token string or a JWT error
///
/// # Errors
///
/// Returns an error if:
/// - Token encoding fails (extremely rare)
/// - Secret key is empty (should not happen with default fallback)
///
/// # Panics
///
/// Panics if system time is before Unix epoch (should never happen in practice)
///
/// # Security
///
/// - Token expires after 24 hours (86400 seconds)
/// - Secret key is loaded from `JWT_SECRET` environment variable
/// - Falls back to development secret if not set (do not use in production)
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// println!("Token: {}", token);
/// ```
#[allow(clippy::disallowed_methods)] // SystemTime::now() needed for JWT timestamp generation
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    #[allow(clippy::cast_possible_truncation)] // Unix timestamps fit in usize on 64-bit systems
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
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns a `Result` containing the decoded `Claims` or a JWT error
///
/// # Errors
///
/// Returns an error if:
/// - Token format is invalid
/// - Token signature verification fails
/// - Token has expired
/// - Token is malformed or tampered with
///
/// # Security
///
/// - Verifies token signature using the secret key
/// - Checks token expiration automatically
/// - Secret key is loaded from `JWT_SECRET` environment variable
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").unwrap();
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
        let token = create_token(user_id).unwrap();

        // Token should not be empty
        assert!(!token.is_empty());

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        // Check expiration is set and is in the future
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);

        // Check that expiration is approximately 24 hours from now
        #[allow(clippy::disallowed_methods)] // Acceptable in tests
        #[allow(clippy::cast_possible_truncation)] // Test-only code
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expected_exp = now + 86400; // 24 hours
        let exp_diff = claims.exp.abs_diff(expected_exp);
        assert!(exp_diff < 10, "Expiration should be ~24 hours from now");
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

        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
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
