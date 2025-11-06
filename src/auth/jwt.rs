use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user identification and token metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (as UNIX timestamp)
    pub exp: usize,
    /// Issued at time (as UNIX timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID with 24-hour expiration
///
/// # Arguments
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if token encoding fails
///
/// # Panics
/// Panics if system time is before UNIX epoch (should never happen on valid systems)
///
/// # Security Notes
/// - Tokens expire after 24 hours
/// - `JWT_SECRET` should be set via environment variable in production
/// - Fallback secret is only for development/testing
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::cast_possible_truncation)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs() as usize;

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: now,
    };

    // Load JWT secret from environment variable with fallback for development
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and extracts its claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The validated claims or an error
///
/// # Errors
/// Returns an error if:
/// - Token is malformed
/// - Token signature is invalid
/// - Token has expired
/// - Token was signed with a different secret
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

        // Token should not be empty
        assert!(!token.is_empty());

        // Token should be valid
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        // Check user ID
        assert_eq!(claims.sub, user_id);

        // Check expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        assert!(claims.exp > now);

        // Check issued at is in the past or present
        assert!(claims.iat <= now + 1); // Allow 1 second tolerance

        // Check expiration is approximately 24 hours from issued time
        let expected_duration: usize = 24 * 3600;
        let actual_duration = claims.exp - claims.iat;
        let diff = actual_duration.abs_diff(expected_duration);
        assert!(diff < 10, "Token duration should be approximately 24 hours");
    }

    #[test]
    fn test_invalid_token_rejected() {
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err(), "Invalid token should be rejected");
    }

    #[test]
    fn test_tampered_token_rejected() {
        let user_id = "789";
        let token = create_token(user_id).unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered);
        assert!(result.is_err(), "Tampered token should be rejected");
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id() {
        let long_id = "a".repeat(1000);
        let token = create_token(&long_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, long_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let special_id = "user@example.com!#$%^&*()";
        let token = create_token(special_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, special_id);
    }
}
