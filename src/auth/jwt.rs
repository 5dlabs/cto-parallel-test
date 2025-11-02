use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user information and expiration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: u64,
    /// Issued at (Unix timestamp)
    pub iat: u64,
}

/// JWT secret key for signing and validation
/// In production, this should come from environment variables
const JWT_SECRET: &str = "your-secret-key-change-in-production";

/// Creates a JWT token for the given user ID
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// A Result containing the JWT token string or an error
///
/// # Errors
///
/// Returns an error if token encoding fails
///
/// # Panics
///
/// Panics if system time is before `UNIX_EPOCH` (extremely rare, system clock issue)
#[allow(clippy::disallowed_methods)] // JWT requires wall-clock time for exp/iat claims
pub fn create_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 86400, // 24 hours
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

/// Validates a JWT token and returns the claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// A Result containing the decoded claims or an error
///
/// # Errors
///
/// Returns an error if token validation fails (expired, invalid signature, etc.)
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token(1);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = 42;
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token);

        assert!(claims.is_ok());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn test_validate_token_invalid() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_claims_contain_user_id() {
        let user_id = 123;
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, "123");
    }

    #[test]
    fn test_token_expiration_set() {
        let token = create_token(1).unwrap();
        let claims = validate_token(&token).unwrap();

        #[allow(clippy::disallowed_methods)] // Test needs wall-clock time to verify JWT claims
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Token should expire 24 hours from now (with some tolerance)
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);
    }

    #[test]
    fn test_token_issued_at_set() {
        let token = create_token(1).unwrap();
        let claims = validate_token(&token).unwrap();

        #[allow(clippy::disallowed_methods)] // Test needs wall-clock time to verify JWT claims
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Issued at should be close to now (within 5 seconds tolerance)
        assert!(claims.iat <= now);
        assert!(claims.iat >= now - 5);
    }
}
