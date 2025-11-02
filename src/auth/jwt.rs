use super::clock::{Clock, SystemClock};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

/// Create a JWT token for the given user ID with 24-hour expiration
///
/// # Arguments
///
/// * `user_id` - The user ID to encode in the token
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - The JWT token or an error
///
/// # Errors
///
/// Returns an error if JWT encoding fails (extremely rare, indicates system issues)
///
/// # Security
///
/// The JWT secret is loaded from the `JWT_SECRET` environment variable.
/// In production, ensure this is set to a secure random value.
/// A fallback value is provided for development only.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_clock(user_id, &SystemClock)
}

/// Create a JWT token with a custom clock (for testing)
fn create_token_with_clock(
    user_id: &str,
    clock: &dyn Clock,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = clock.now_seconds();
    let expiration = now + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        #[allow(clippy::cast_possible_truncation)]
        exp: expiration as usize,
        #[allow(clippy::cast_possible_truncation)]
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
///
/// * `token` - The JWT token to validate
///
/// # Returns
///
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
///
/// Returns an error if:
/// - Token signature is invalid
/// - Token has expired
/// - Token format is malformed
/// - Token was tampered with
///
/// # Security
///
/// - Verifies token signature using the JWT secret
/// - Checks token expiration
/// - Rejects tampered or invalid tokens
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
    use super::super::clock::MockClock;
    use super::*;

    #[test]
    fn test_token_creation_succeeds() {
        let token = create_token("123").unwrap();
        assert!(!token.is_empty());
        assert!(token.contains('.'));
    }

    #[test]
    fn test_token_validation_succeeds_with_valid_token() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_invalid_token_is_rejected() {
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "123";
        // Use current time to avoid expiration issues in tests
        let token = create_token(user_id).unwrap();

        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "123");
        // Verify expiration is ~24 hours in future
        assert_eq!(claims.exp - claims.iat, 86_400);
    }

    #[test]
    fn test_expiration_is_24_hours_in_future() {
        let token = create_token("123").unwrap();
        let claims = validate_token(&token).unwrap();

        // Verify expiration is exactly 24 hours (86400 seconds) after issuance
        assert_eq!(claims.exp - claims.iat, 86_400);
    }

    #[test]
    fn test_same_user_produces_different_tokens_due_to_timestamp() {
        let token1 = create_token("123").unwrap();
        // Small delay to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let token2 = create_token("123").unwrap();

        // Decode both tokens to verify they have different timestamps
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        // Timestamps should be different (at least 1 second apart)
        assert!(claims2.iat > claims1.iat);
        assert!(claims2.exp > claims1.exp);
    }

    #[test]
    fn test_tampered_token_is_rejected() {
        let token = create_token("123").unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.chars().collect::<Vec<_>>();
        if let Some(c) = tampered.last_mut() {
            *c = if *c == 'a' { 'b' } else { 'a' };
        }
        let tampered_token: String = tampered.into_iter().collect();

        assert!(validate_token(&tampered_token).is_err());
    }

    #[test]
    fn test_empty_user_id_is_handled() {
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id_is_handled() {
        let long_id = "a".repeat(1000);
        let token = create_token(&long_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, long_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let special_id = "user!@#$%^&*()123";
        let token = create_token(special_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, special_id);
    }

    #[test]
    fn test_mock_clock_for_deterministic_timestamps() {
        // Test that the clock abstraction works correctly
        let clock = MockClock {
            timestamp: 2_000_000_000, // Future timestamp to avoid expiration
        };
        let token = create_token_with_clock("test_user", &clock).unwrap();

        // Note: We can't validate this token because validate_token uses current time
        // and the token would be invalid. This test verifies token creation works with mock clock.
        assert!(!token.is_empty());

        // Create another token with same clock - should have identical timestamps
        let token2 = create_token_with_clock("test_user", &clock).unwrap();
        assert_eq!(token, token2);
    }
}
