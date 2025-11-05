// JWT tokens require real system time for expiration - cannot use Clock abstraction
#![allow(clippy::disallowed_methods)]

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Create a JWT token for a user ID with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user's identifier to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if token encoding fails (e.g., invalid secret key format)
///
/// # Panics
/// Panics if system time is before `UNIX_EPOCH` (should never happen on modern systems)
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::cast_possible_truncation)] // JWT timestamps are u64, but Claims uses usize for compatibility
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

/// Validate a JWT token and extract claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if:
/// - Token is malformed or invalid
/// - Token signature verification fails
/// - Token has expired
/// - Claims cannot be decoded
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
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
    use jsonwebtoken::{encode, errors::ErrorKind, EncodingKey, Header};

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        // Token should not be empty
        assert!(!token.is_empty());

        // Validate the token
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);

        // Check expiration is set
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn test_token_expiration_is_24_hours() {
        let token = create_token("123").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        let expected_exp = now + 86400; // 24 hours in seconds
        let time_diff = (claims.exp as i64 - expected_exp as i64).abs();

        // Allow 10 seconds of difference for test execution time
        assert!(time_diff < 10);
    }

    #[test]
    fn test_invalid_token() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_expired_token_is_rejected() {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

        let claims = Claims {
            sub: "expired_user".to_string(),
            exp: 0,
            iat: 0,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Failed to encode expired token");

        let err = validate_token(&token).expect_err("Expired token should be rejected");
        assert!(matches!(err.kind(), ErrorKind::ExpiredSignature));
    }

    #[test]
    fn test_token_signed_with_different_secret_is_rejected() {
        let claims = Claims {
            sub: "user_789".to_string(),
            exp: usize::MAX / 2,
            iat: 0,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"different_secret_key"),
        )
        .expect("Failed to encode token with alternate secret");

        let err = validate_token(&token)
            .expect_err("Token signed with different secret should be rejected");
        assert!(matches!(
            err.kind(),
            ErrorKind::InvalidSignature | ErrorKind::InvalidToken
        ));
    }

    #[test]
    fn test_tampered_token_is_rejected() {
        let token = create_token("123").expect("Failed to create token");
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        let mut signature = parts[2].to_string();
        let last_char = signature
            .pop()
            .expect("JWT signature segment should not be empty");
        let replacement = if last_char == 'A' { 'B' } else { 'A' };
        signature.push(replacement);

        let tampered_token = format!("{}.{}.{}", parts[0], parts[1], signature);

        let err = validate_token(&tampered_token).expect_err("Tampered token should be rejected");
        assert!(matches!(
            err.kind(),
            ErrorKind::InvalidSignature | ErrorKind::InvalidToken
        ));
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let user_id = "123";
        let token1 = create_token(user_id).expect("Failed to create token 1");

        // Wait to ensure different timestamps (1 second to be safe with second-precision timestamps)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different iat timestamps
        assert_ne!(token1, token2);

        // But both should validate to the same user
        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");
        assert_eq!(claims1.sub, claims2.sub);
    }

    #[test]
    fn test_claims_structure() {
        let token = create_token("user_456").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify all required claims are present
        assert_eq!(claims.sub, "user_456");
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
    }
}
