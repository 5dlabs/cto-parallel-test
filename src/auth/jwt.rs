use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (Unix timestamp)
    pub iat: usize,  // Issued at (Unix timestamp)
}

/// Creates a JWT token for the given user ID.
/// The token expires after 24 hours.
///
/// # Arguments
/// * `user_id` - The user ID to include in the token claims
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if JWT encoding fails (e.g., invalid header or claims)
///
/// # Panics
/// Panics if system time is before Unix epoch (which should never happen in practice)
///
/// # Example
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("user123").unwrap();
/// assert!(!token.is_empty());
/// ```
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::disallowed_methods)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours in seconds

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

/// Validates a JWT token and returns the claims if valid.
///
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if:
/// - Token format is invalid
/// - Token signature is invalid
/// - Token has expired
/// - Token claims cannot be decoded
///
/// # Example
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("user123").unwrap();
/// let claims = validate_token(&token).unwrap();
/// assert_eq!(claims.sub, "user123");
/// ```
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
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::disallowed_methods)]
    fn test_token_contains_expiration() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Expiration should be approximately 24 hours from now
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 86400 + 5); // Allow 5 second tolerance
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::disallowed_methods)]
    fn test_token_contains_issued_at() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Issued at should be approximately now
        assert!(claims.iat <= now);
        assert!(claims.iat >= now - 5); // Allow 5 second tolerance
    }

    #[test]
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid.token.string");
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_token_rejected() {
        let token = create_token("user123").unwrap();
        let mut tampered_token = token.clone();
        // Tamper with the token by changing a character
        tampered_token.push('X');
        let result = validate_token(&tampered_token);
        assert!(result.is_err());
    }
}
