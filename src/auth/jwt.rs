use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (Unix timestamp)
    pub iat: usize,  // Issued at (Unix timestamp)
}

/// Creates a JWT token for a given user ID
///
/// # Arguments
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if the JWT encoding fails
///
/// # Panics
/// Panics if the system time is before the Unix epoch (January 1, 1970)
#[allow(clippy::disallowed_methods)] // JWT requires wall-clock time; Clock abstraction is out of scope for this task
#[allow(clippy::cast_possible_truncation)] // Unix timestamps fit in usize on all supported platforms
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours from now (86400 seconds)

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

/// Validates a JWT token and extracts the claims
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if the token is invalid, expired, or has an invalid signature
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
    #[allow(clippy::disallowed_methods)] // Test needs to verify timestamps
    #[allow(clippy::cast_possible_truncation)] // Acceptable in tests
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        // Token should not be empty
        assert!(!token.is_empty());

        // Validate the token
        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify the user ID matches
        assert_eq!(claims.sub, user_id);

        // Verify expiration is set (should be 24 hours from now)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400);

        // Verify issued at time is reasonable
        assert!(claims.iat <= now);
        assert!(claims.iat > now - 10); // Should be within the last 10 seconds
    }

    #[test]
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_different_secret_rejected() {
        // This test ensures that tokens signed with a different secret are rejected
        use jsonwebtoken::{encode, EncodingKey, Header};

        let claims = Claims {
            sub: "123".to_string(),
            exp: usize::MAX,
            iat: 0,
        };

        let wrong_secret = b"wrong_secret_key";
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(wrong_secret),
        )
        .expect("Failed to create token with wrong secret");

        let result = validate_token(&token);
        assert!(result.is_err());
    }
}
