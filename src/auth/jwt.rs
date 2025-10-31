use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing user authentication information
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (as UNIX timestamp)
    pub exp: usize,
    /// Issued at time (as UNIX timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
///
/// # Arguments
/// * `user_id` - The unique identifier for the user
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token
///
/// # Errors
/// Returns an error if JWT encoding fails
///
/// # Panics
/// Panics if system time is before UNIX epoch (extremely unlikely in practice)
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("user123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    // Justified: JWT timestamps are in seconds since epoch, which fits in usize on all supported platforms
    #[allow(clippy::cast_possible_truncation)]
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: now as usize,
    };

    let secret = get_jwt_secret();
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
/// * `Ok(Claims)` - The decoded claims if token is valid
///
/// # Errors
/// Returns an error if the token is malformed, expired, or has an invalid signature
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("user123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

/// Gets the JWT secret from environment variable or uses default for development
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "test_secret_key".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let user_id = "test_user_123";
        let result = create_token(user_id);
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");
        let result = validate_token(&token);
        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_contains_all_claims() {
        let user_id = "test_user_789";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration_is_24_hours() {
        let user_id = "test_user_exp";
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Expiration should be approximately 24 hours (86400 seconds) from now
        let expected_exp_min = before + 86400;
        let expected_exp_max = after + 86400;

        assert!(
            claims.exp as u64 >= expected_exp_min && claims.exp as u64 <= expected_exp_max,
            "Token expiration should be approximately 24 hours from creation"
        );
    }

    #[test]
    fn test_validate_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_malformed_token() {
        let result = validate_token("not-a-jwt-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_token() {
        let result = validate_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_equality() {
        let claims1 = Claims {
            sub: "user1".to_string(),
            exp: 1000,
            iat: 500,
        };
        let claims2 = Claims {
            sub: "user1".to_string(),
            exp: 1000,
            iat: 500,
        };
        assert_eq!(claims1, claims2);
    }

    #[test]
    fn test_claims_clone() {
        let claims = Claims {
            sub: "user1".to_string(),
            exp: 1000,
            iat: 500,
        };
        let cloned = claims.clone();
        assert_eq!(claims, cloned);
    }
}
