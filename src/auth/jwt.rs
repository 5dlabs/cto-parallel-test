use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure containing standard claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (as Unix timestamp)
    pub exp: usize,
    /// Issued at (as Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
///
/// # Arguments
///
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
///
/// A `Result` containing the JWT token string or an error
///
/// # Errors
///
/// Returns an error if token encoding fails
///
/// # Panics
///
/// Panics if the system time is before the UNIX epoch (extremely rare, only on system misconfiguration)
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).expect("Timestamp overflow"),
        iat: usize::try_from(now).expect("Timestamp overflow"),
    };

    // Load JWT secret from environment variable, fallback for development
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
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// A `Result` containing the decoded `Claims` or an error
///
/// # Errors
///
/// Returns an error if:
/// - Token is malformed
/// - Token signature is invalid
/// - Token has expired
/// - Token validation fails for any other reason
///
/// # Examples
///
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
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_create_token_success() {
        let token = create_token("123").expect("Failed to create token");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let token = create_token("123").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "123");
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);

        // Verify expiration is approximately 24 hours in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expected_exp = now + 86400;
        let diff = claims.exp.abs_diff(usize::try_from(expected_exp).unwrap());
        assert!(diff < 10); // Within 10 seconds tolerance
    }

    #[test]
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_token_rejected() {
        let result = validate_token("not.a.valid.jwt.token");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token_rejected() {
        let result = validate_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        let token1 = create_token("123").expect("Failed to create token 1");
        sleep(Duration::from_secs(1)); // Ensure different timestamps (1 second)
        let token2 = create_token("123").expect("Failed to create token 2");

        // Tokens should be different due to different timestamps
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id() {
        let long_id = "a".repeat(1000);
        let token = create_token(&long_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, long_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let special_id = "user@example.com!#$%^&*()";
        let token = create_token(special_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, special_id);
    }

    #[test]
    fn test_unicode_user_id() {
        let unicode_id = "用户123";
        let token = create_token(unicode_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, unicode_id);
    }

    #[test]
    fn test_whitespace_in_user_id() {
        let id_with_spaces = "user with spaces";
        let token = create_token(id_with_spaces).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, id_with_spaces);
    }

    #[test]
    fn test_token_expiration_set_correctly() {
        let token = create_token("123").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify expiration is 24 hours (86400 seconds) from issuance
        let expected_duration = 86400;
        let actual_duration = claims.exp - claims.iat;
        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_multiple_users_dont_interfere() {
        let token1 = create_token("user1").expect("Failed to create token 1");
        let token2 = create_token("user2").expect("Failed to create token 2");

        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");

        assert_eq!(claims1.sub, "user1");
        assert_eq!(claims2.sub, "user2");
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_tampered_token_rejected() {
        let token = create_token("123").expect("Failed to create token");

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered);
        assert!(result.is_err());
    }
}
