//! JWT token creation and validation
//!
//! Provides functions for creating and validating JWT tokens for authentication.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// JWT token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Error types for JWT operations
#[derive(Debug)]
pub enum JwtError {
    /// Token creation failed
    TokenCreation(String),
    /// Token validation failed
    TokenValidation(String),
    /// Invalid token format
    InvalidToken(String),
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenCreation(msg) => write!(f, "Token creation error: {msg}"),
            Self::TokenValidation(msg) => write!(f, "Token validation error: {msg}"),
            Self::InvalidToken(msg) => write!(f, "Invalid token: {msg}"),
        }
    }
}

impl std::error::Error for JwtError {}

/// Creates a JWT token for a user
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// A JWT token string
///
/// # Errors
///
/// Returns `JwtError` if token creation fails
pub fn create_token(user_id: i32) -> Result<String, JwtError> {
    let secret =
        env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key_for_development".to_string());

    let now = chrono::Utc::now();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| JwtError::TokenCreation(format!("Failed to encode token: {e}")))
}

/// Validates a JWT token and extracts claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The decoded claims if validation succeeds
///
/// # Errors
///
/// Returns `JwtError` if token validation fails
pub fn validate_token(token: &str) -> Result<Claims, JwtError> {
    let secret =
        env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key_for_development".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| JwtError::TokenValidation(format!("Failed to decode token: {e}")))?;

    Ok(token_data.claims)
}

/// Extracts user ID from JWT claims
///
/// # Arguments
///
/// * `claims` - The JWT claims
///
/// # Returns
///
/// The user ID as i32
///
/// # Errors
///
/// Returns `JwtError` if the user ID cannot be parsed
pub fn extract_user_id(claims: &Claims) -> Result<i32, JwtError> {
    claims
        .sub
        .parse::<i32>()
        .map_err(|e| JwtError::InvalidToken(format!("Invalid user ID in token: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let user_id = 123;
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "123");

        let extracted_id = extract_user_id(&claims).expect("Failed to extract user ID");
        assert_eq!(extracted_id, user_id);
    }

    #[test]
    fn test_validate_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_user_id_from_claims() {
        let claims = Claims {
            sub: "456".to_string(),
            exp: 9_999_999_999,
            iat: 1_000_000_000,
        };

        let user_id = extract_user_id(&claims).expect("Failed to extract user ID");
        assert_eq!(user_id, 456);
    }

    #[test]
    fn test_extract_invalid_user_id() {
        let claims = Claims {
            sub: "not_a_number".to_string(),
            exp: 9_999_999_999,
            iat: 1_000_000_000,
        };

        let result = extract_user_id(&claims);
        assert!(result.is_err());
    }
}
