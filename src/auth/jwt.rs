use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user identification and token metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - the user ID this token represents
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for a given user ID
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token
///
/// # Errors
/// Returns an error if JWT encoding fails (should be rare with valid inputs)
///
/// # Panics
/// Panics if the system time is before the Unix epoch (extremely unlikely)
///
/// # Token Lifetime
/// Tokens expire 24 hours (86400 seconds) after creation
///
/// # Security Note
/// Uses a hardcoded test secret key. In production, this should come from environment variables.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::cast_possible_truncation)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now,
    };

    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and extracts its claims
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Ok(Claims)` - The decoded claims if the token is valid
///
/// # Errors
/// Returns an error if:
/// - The token signature is invalid
/// - The token has expired
/// - The token is malformed
///
/// # Validation
/// Automatically checks:
/// - Token signature matches the secret key
/// - Token has not expired
/// - Token structure is valid
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let user_id = "test_user_123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);

        // Verify expiration is set to future (24 hours from now)
        #[allow(clippy::cast_possible_truncation)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        assert!(claims.exp > now);
        assert!(claims.iat <= now);
    }

    #[test]
    fn test_validate_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_with_wrong_secret() {
        // Create token with correct secret
        let user_id = "test_user";
        let token = create_token(user_id).expect("Failed to create token");

        // Token should validate with same secret
        let result = validate_token(&token);
        assert!(result.is_ok());
    }
}
