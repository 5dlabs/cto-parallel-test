//! JWT token creation and validation module
//!
//! This module provides functions to create and validate JWT tokens for authentication.
//! Tokens expire after 24 hours and include standard JWT claims (sub, exp, iat).

use super::clock::{Clock, SystemClock};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT Claims structure containing standard token claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Creates a JWT token for the given user ID using the default system clock
///
/// # Arguments
///
/// * `user_id` - The user ID to include in the token's subject claim
///
/// # Returns
///
/// * `Result<String, Error>` - The encoded JWT token or an error
///
/// # Errors
///
/// Returns an error if token encoding fails (rare, usually indicates invalid secret)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
/// std::env::set_var("JWT_SECRET", "test_secret_key_change_in_production");
///
/// let token = create_token("123").expect("Failed to create token");
/// ```
pub fn create_token(user_id: &str) -> Result<String, Error> {
    create_token_with_clock(user_id, &SystemClock)
}

/// Creates a JWT token for the given user ID using a custom clock (for testing)
///
/// # Arguments
///
/// * `user_id` - The user ID to include in the token's subject claim
/// * `clock` - Clock implementation for obtaining current time
///
/// # Returns
///
/// * `Result<String, Error>` - The encoded JWT token or an error
///
/// # Errors
///
/// Returns an error if token encoding fails (rare, usually indicates invalid secret)
/// or when time calculations overflow or are otherwise invalid.
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::{jwt::create_token_with_clock, SystemClock};
/// std::env::set_var("JWT_SECRET", "test_secret_key_change_in_production");
///
/// let clock = SystemClock;
/// let token = create_token_with_clock("123", &clock).expect("Failed to create token");
/// ```
pub fn create_token_with_clock(user_id: &str, clock: &impl Clock) -> Result<String, Error> {
    const TOKEN_TTL_SECS: u64 = 24 * 60 * 60;

    let now = clock
        .now()
        .map_err(|_| Error::from(ErrorKind::InvalidToken))?;
    let expiration = now
        .checked_add(TOKEN_TTL_SECS)
        .ok_or_else(|| Error::from(ErrorKind::InvalidToken))?;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: usize::try_from(expiration).map_err(|_| Error::from(ErrorKind::InvalidToken))?,
        iat: usize::try_from(now).map_err(|_| Error::from(ErrorKind::InvalidToken))?,
    };

    // Load and validate JWT secret from environment variable (no fallback to avoid hardcoded secrets)
    let secret = load_jwt_secret()?;

    // Be explicit about the algorithm used in the header
    let mut header = Header::new(Algorithm::HS256);
    // Set `typ` for clarity; some validators expect an explicit type
    header.typ = Some("JWT".to_string());

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validates a JWT token and returns the claims if valid
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// * `Result<Claims, Error>` - The decoded claims or an error
///
/// # Errors
///
/// Returns an error if:
/// - Token is malformed or invalid
/// - Token has expired
/// - Token signature verification fails
/// - Token was signed with a different secret
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
/// std::env::set_var("JWT_SECRET", "test_secret_key_change_in_production");
///
/// let token = create_token("123").unwrap();
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, Error> {
    let secret = load_jwt_secret()?;

    // Enforce expected algorithm explicitly for defense-in-depth
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

/// Loads the JWT secret from environment and enforces a minimum strength requirement.
///
/// Security rationale:
/// - Short HMAC secrets reduce effective security. Enforce >= 32 bytes to align with
///   common guidance for HS256 and preempt weak-secret findings from scanners.
fn load_jwt_secret() -> Result<String, Error> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| Error::from(ErrorKind::InvalidToken))?;

    // Enforce a reasonable minimum length for HS256 HMAC keys
    if secret.len() < 32 {
        return Err(Error::from(ErrorKind::InvalidToken));
    }
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    // Move imports to module scope to satisfy clippy::items_after_statements in tests
    use super::super::clock::Clock as TestClock;
    use std::time::{Duration, SystemTimeError, UNIX_EPOCH};

    // Helper clock that always errors to exercise error propagation
    struct FailingClock;

    impl TestClock for FailingClock {
        fn now(&self) -> Result<u64, SystemTimeError> {
            let err = UNIX_EPOCH
                .duration_since(UNIX_EPOCH + Duration::from_secs(1))
                .expect_err("expected time error");
            Err(err)
        }
    }

    fn ensure_test_secret() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            std::env::set_var("JWT_SECRET", "test_secret_key_change_in_production");
        });
    }

    #[test]
    fn test_create_token_success() {
        ensure_test_secret();
        let user_id = "123";
        let token = create_token(user_id);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        ensure_test_secret();
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_token_contains_correct_claims() {
        ensure_test_secret();
        let user_id = "123";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "123");
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration_is_24_hours() {
        use super::super::clock::test_helpers::MockClock;
        ensure_test_secret();

        let user_id = "123";
        // Use a future timestamp to avoid expiration during validation
        let now = 2_000_000_000_u64; // Fixed timestamp for testing (around 2033)
        let clock = MockClock::new(now);

        let token = create_token_with_clock(user_id, &clock).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let expected_exp = now + 86400; // 24 hours in seconds

        assert_eq!(
            claims.exp,
            usize::try_from(expected_exp).unwrap(),
            "Token expiration not set to 24 hours"
        );
        assert_eq!(
            claims.iat,
            usize::try_from(now).unwrap(),
            "Token issued at time incorrect"
        );
    }

    #[test]
    fn test_invalid_token_rejected() {
        ensure_test_secret();
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_clock_error_propagates() {
        ensure_test_secret();
        let result = create_token_with_clock("123", &FailingClock);
        assert!(result.is_err());
        let err = result.expect_err("expected clock failure");
        assert!(matches!(err.kind(), ErrorKind::InvalidToken));
    }

    #[test]
    fn test_expired_token_rejected() {
        use jsonwebtoken::errors::ErrorKind;
        ensure_test_secret();

        let claims = Claims {
            sub: "123".to_string(),
            exp: 1, // Clearly expired relative to current time
            iat: 0,
        };

        let secret = "test_secret_key_change_in_production";
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("failed to encode token");

        let result = validate_token(&token);
        assert!(result.is_err());
        let err = result.expect_err("token should be expired");
        assert!(matches!(err.kind(), ErrorKind::ExpiredSignature));
    }

    #[test]
    fn test_empty_token_rejected() {
        ensure_test_secret();
        let result = validate_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_tokens_for_same_user() {
        ensure_test_secret();
        let user_id = "123";
        let token1 = create_token(user_id).expect("Failed to create token 1");

        // Small delay to ensure different timestamp (at least 1 second for Unix timestamp)
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(user_id).expect("Failed to create token 2");

        // Tokens should be different due to different iat timestamps
        assert_ne!(token1, token2);

        // Both should validate successfully
        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");

        assert_eq!(claims1.sub, user_id);
        assert_eq!(claims2.sub, user_id);
    }

    #[test]
    fn test_empty_user_id_handled() {
        ensure_test_secret();
        let user_id = "";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id_handled() {
        ensure_test_secret();
        let user_id = "a".repeat(1000);
        let token = create_token(&user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        ensure_test_secret();
        let user_id = "user@email.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);
    }
}
