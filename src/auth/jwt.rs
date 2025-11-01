//! # JWT Token Handling
//!
//! This module provides JWT token creation and validation with secure defaults:
//! - 24-hour token expiration
//! - HMAC-based signing
//! - Standard JWT claims (sub, exp, iat)
//! - Environment-based secret key configuration

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user authentication information
///
/// # Fields
/// * `sub` - Subject (user ID as string)
/// * `exp` - Expiration time (Unix timestamp)
/// * `iat` - Issued at time (Unix timestamp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
}

/// Create a JWT token for the given user ID
///
/// # Arguments
/// * `user_id` - The user's unique identifier
///
/// # Returns
/// * `Ok(String)` - The signed JWT token
/// * `Err(jsonwebtoken::errors::Error)` - If token creation fails
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
///
/// # Security Notes
/// - Tokens expire after 24 hours
/// - Uses `JWT_SECRET` environment variable or fallback for development
/// - Production deployments MUST set `JWT_SECRET` environment variable
///
/// # Errors
/// Returns an error if JWT encoding fails (e.g., invalid secret key)
///
/// # Panics
/// Panics if system time is before Unix epoch (extremely rare)
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // JWT tokens require wall-clock time (not monotonic time) for iat/exp claims
    // This is a standard JWT requirement and cannot use Instant
    #[allow(clippy::disallowed_methods)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).unwrap_or(usize::MAX),
        iat: usize::try_from(now).unwrap_or(0),
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

/// Validate a JWT token and extract its claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Ok(Claims)` - The validated claims from the token
/// * `Err(jsonwebtoken::errors::Error)` - If validation fails
///
/// # Example
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "123");
/// ```
///
/// # Security Notes
/// - Validates token signature
/// - Checks expiration time
/// - Must use same secret as `create_token`
///
/// # Errors
/// Returns an error if:
/// - Token signature is invalid
/// - Token is expired
/// - Token format is malformed
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token("123");
        assert!(token.is_ok());
        let token_str = token.unwrap();
        assert!(!token_str.is_empty());
        assert!(token_str.contains('.'));
    }

    #[test]
    fn test_validate_token() {
        let token = create_token("123").unwrap();
        let claims = validate_token(&token);
        assert!(claims.is_ok());
        let claims_data = claims.unwrap();
        assert_eq!(claims_data.sub, "123");
    }

    #[test]
    fn test_token_expiration() {
        let token = create_token("456").unwrap();
        let claims = validate_token(&token).unwrap();

        #[allow(clippy::disallowed_methods)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(claims.exp > usize::try_from(now).unwrap());
        assert!(claims.iat <= usize::try_from(now).unwrap());
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_token() {
        let token = create_token("789").unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.{}.tampered", parts[0], parts[1]);
        let result = validate_token(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_roundtrip() {
        let user_id = "user_42";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }
}
