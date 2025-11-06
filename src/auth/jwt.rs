//! JWT token creation and validation
//!
//! Provides functions for creating and validating JSON Web Tokens (JWT)
//! with 24-hour expiration using configurable secret keys.

use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Development fallback secret for JWT tokens
///
/// This is only used when `JWT_SECRET` environment variable is not set.
/// In production, `JWT_SECRET` should always be set to a secure random key.
const DEV_FALLBACK_SECRET: &str = "dev_jwt_secret_key_change_in_production";

fn get_dev_fallback_secret() -> String {
    DEV_FALLBACK_SECRET.to_string()
}

/// Abstraction over time source to improve testability and satisfy clippy `disallowed_methods` rule.
pub trait Clock: Send + Sync {
    /// Returns current time as `SystemTime`.
    fn now(&self) -> SystemTime;
}

/// Real clock implementation that delegates to the system clock.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    #[allow(clippy::disallowed_methods)] // Allowed in the single location responsible for wall-clock time
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

fn unix_timestamp_seconds(clock: &dyn Clock) -> Result<u64, Error> {
    let duration_since_epoch = clock
        .now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::from(ErrorKind::InvalidToken))?;
    Ok(duration_since_epoch.as_secs())
}

fn convert_timestamp(value: u64) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::from(ErrorKind::InvalidToken))
}

fn create_claims_with_clock(user_id: &str, clock: &dyn Clock) -> Result<Claims, Error> {
    let issued_at = unix_timestamp_seconds(clock)?;
    let expiration = issued_at
        .checked_add(24 * 3600)
        .ok_or_else(|| Error::from(ErrorKind::InvalidToken))?;

    Ok(Claims {
        sub: user_id.to_owned(),
        exp: convert_timestamp(expiration)?,
        iat: convert_timestamp(issued_at)?,
    })
}

fn create_token_with_clock(clock: &dyn Clock, user_id: &str) -> Result<String, Error> {
    let claims = create_claims_with_clock(user_id, clock)?;
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| get_dev_fallback_secret());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// JWT claims structure following RFC 7519
///
/// Contains standard JWT claims for authentication:
/// - `sub`: Subject (user ID)
/// - `exp`: Expiration time (Unix timestamp)
/// - `iat`: Issued at time (Unix timestamp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - User ID
    pub sub: String,
    /// Expiration time - Unix timestamp
    pub exp: usize,
    /// Issued at - Unix timestamp
    pub iat: usize,
}

/// Create a JWT token for a user ID with 24-hour expiration
///
/// # Arguments
///
/// * `user_id` - The user ID to embed in the token
///
/// # Returns
///
/// A JWT token string on success.
///
/// # Errors
///
/// Returns an error if token encoding fails (extremely rare, would indicate
/// a critical system issue) or if the system clock produces an invalid
/// timestamp (before Unix epoch or exceeding platform bounds).
///
/// # Panics
///
/// Returns an error if the system time is before the Unix epoch (January 1, 1970)
/// or if timestamp conversion to `usize` fails (would only occur on systems with
/// 32-bit `usize` in the distant future, year 2038+).
///
/// # Security
///
/// - Tokens expire after 24 hours
/// - Secret key loaded from `JWT_SECRET` environment variable
/// - Falls back to development key if not set (production should always set this)
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_clock(&SystemClock, user_id)
}

/// Validate a JWT token and extract claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// The decoded claims if validation succeeds.
///
/// # Errors
///
/// Returns an error if:
/// - Token is malformed
/// - Token signature is invalid
/// - Token has expired
/// - Token was signed with a different secret
///
/// # Security
///
/// - Validates token signature
/// - Checks expiration timestamp
/// - Uses the same secret key as token creation
///
/// # Examples
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user_123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| get_dev_fallback_secret());

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
    use jsonwebtoken::errors::ErrorKind;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    static ENV_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    struct EnvGuard {
        _guard: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn new() -> Self {
            let guard = ENV_MUTEX.lock().unwrap_or_else(PoisonError::into_inner);
            std::env::remove_var("JWT_SECRET");
            Self { _guard: guard }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            std::env::remove_var("JWT_SECRET");
        }
    }

    #[test]
    fn test_token_creation() {
        let _env_guard = EnvGuard::new();
        let user_id = "test_user_123";
        let token = create_token(user_id).expect("Failed to create token");

        // Token should not be empty
        assert!(!token.is_empty());

        // Token should have 3 parts separated by dots (JWT format)
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "JWT should have 3 parts: header.payload.signature"
        );
    }

    #[test]
    fn test_token_validation_success() {
        let _env_guard = EnvGuard::new();
        let user_id = "test_user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify user ID matches
        assert_eq!(claims.sub, user_id);

        // Verify timestamps are reasonable
        assert!(
            claims.exp > claims.iat,
            "Expiration should be after issued time"
        );

        // Verify expiration is approximately 24 hours from issued time
        let expected_duration = 24_i64 * 3600; // 24 hours in seconds
        let actual_duration = i64::try_from(claims.exp - claims.iat).unwrap();
        assert!(
            (actual_duration - expected_duration).abs() < 10,
            "Token should expire in approximately 24 hours"
        );
    }

    #[test]
    fn test_invalid_token_rejected() {
        let _env_guard = EnvGuard::new();
        let invalid_token = "invalid.token.here";
        let result = validate_token(invalid_token);

        assert!(result.is_err(), "Invalid token should be rejected");
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // SystemTime::now() is acceptable in tests
    fn test_expired_token_rejected() {
        let _env_guard = EnvGuard::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs();

        let claims = Claims {
            sub: "expired_user".to_string(),
            exp: usize::try_from(now.saturating_sub(120)).expect("Timestamp overflow"),
            iat: usize::try_from(now.saturating_sub(240)).expect("Timestamp overflow"),
        };

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| get_dev_fallback_secret());

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Failed to encode expired token");

        let err = validate_token(&token).expect_err("Expired token should be rejected");
        assert!(
            matches!(err.kind(), ErrorKind::ExpiredSignature),
            "Expected expired signature error but got {err:?}"
        );
    }

    #[test]
    fn test_tampered_token_rejected() {
        let _env_guard = EnvGuard::new();
        let user_id = "test_user_789";
        let token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last_char) = tampered.pop() {
            tampered.push(if last_char == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered);
        assert!(result.is_err(), "Tampered token should be rejected");
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // SystemTime::now() is acceptable in tests
    fn test_token_rejected_with_wrong_secret() {
        let _env_guard = EnvGuard::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs();

        let claims = Claims {
            sub: "user_env_secret".to_string(),
            exp: usize::try_from(now.checked_add(3600).expect("Timestamp overflow"))
                .expect("Timestamp overflow"),
            iat: usize::try_from(now).expect("Timestamp overflow"),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"alternate_secret_for_test"),
        )
        .expect("Failed to sign token with alternate secret");

        let result = validate_token(&token);

        assert!(
            result.is_err(),
            "Token signed with a different secret should be rejected"
        );
    }

    #[test]
    fn test_different_users_have_different_tokens() {
        let _env_guard = EnvGuard::new();
        let token1 = create_token("user_1").expect("Failed to create token 1");
        let token2 = create_token("user_2").expect("Failed to create token 2");

        assert_ne!(
            token1, token2,
            "Different users should have different tokens"
        );
    }

    #[derive(Clone)]
    struct FixedClock {
        instant: SystemTime,
    }

    impl FixedClock {
        fn new(instant: SystemTime) -> Self {
            Self { instant }
        }
    }

    impl Clock for FixedClock {
        fn now(&self) -> SystemTime {
            self.instant
        }
    }

    #[test]
    fn test_same_user_different_timestamps() {
        let _env_guard = EnvGuard::new();
        let user_id = "test_user_same";

        let system_clock = SystemClock;
        let base_time = system_clock.now();
        let token1 = create_token_with_clock(&FixedClock::new(base_time), user_id)
            .expect("Failed to create token 1");
        let token2 = create_token_with_clock(
            &FixedClock::new(base_time + Duration::from_secs(1)),
            user_id,
        )
        .expect("Failed to create token 2");

        assert_ne!(
            token1, token2,
            "Same user at different times should have different tokens"
        );

        let claims1 = validate_token(&token1).expect("Failed to validate token 1");
        let claims2 = validate_token(&token2).expect("Failed to validate token 2");
        assert_eq!(claims1.sub, claims2.sub);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // SystemTime::now() is acceptable in tests
    fn test_claims_fields() {
        let _env_guard = EnvGuard::new();
        let user_id = "claims_test_user";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        // Verify all required fields are present
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0, "Expiration should be set");
        assert!(claims.iat > 0, "Issued at should be set");

        // Verify expiration is in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs();
        assert!(
            claims.exp as u64 > now,
            "Token should not be expired immediately"
        );
    }

    #[test]
    fn test_empty_user_id() {
        let _env_guard = EnvGuard::new();
        let token = create_token("").expect("Failed to create token with empty user ID");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "", "Empty user ID should be preserved");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let _env_guard = EnvGuard::new();
        let user_id = "user@example.com|123!#$%";
        let token = create_token(user_id).expect("Failed to create token with special characters");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(
            claims.sub, user_id,
            "Special characters should be preserved"
        );
    }
}
