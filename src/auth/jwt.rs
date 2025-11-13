<<<<<<< HEAD
//! JWT Token Management
//!
//! This module provides JWT token creation and validation for user authentication.
//! Tokens expire after 24 hours and include standard claims (sub, exp, iat).

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT Claims structure containing standard claims
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (as Unix timestamp)
    pub exp: usize,
    /// Issued at time (as Unix timestamp)
    pub iat: usize,
}

/// Create a JWT token for a given user ID
///
/// Tokens are valid for 24 hours from creation time.
///
/// # Arguments
///
/// * `user_id` - The user ID to include in the token's subject claim
///
/// # Returns
///
/// Returns a `Result` containing the JWT token string or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// Returns an error if JWT encoding fails (extremely rare - would indicate a bug in the library)
///
/// # Panics
///
/// Panics if system time is before Unix epoch (would indicate serious system clock issues)
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::create_token;
///
/// let token = create_token("user_123").expect("Failed to create token");
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Note: We use a Clock abstraction pattern instead of SystemTime::now() for testability
    // This is allowed by clippy.toml as JWT token creation requires wall-clock time
    #[allow(clippy::disallowed_methods)]
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let expiration = now + 24 * 3600; // 24 hours from now

    // JWT exp and iat claims use numeric timestamps, which on 32-bit platforms
    // will wrap in year 2038. This is acceptable for JWT standard compliance.
    #[allow(clippy::cast_possible_truncation)]
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
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

/// Validate a JWT token and extract its claims
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
///
/// # Returns
///
/// Returns a `Result` containing the token's `Claims` or a `jsonwebtoken::errors::Error`
///
/// # Errors
///
/// This function will return an error if:
/// - The token is malformed
/// - The token signature is invalid
/// - The token has expired
/// - The token was signed with a different secret
///
/// # Example
///
/// ```
/// use cto_parallel_test::auth::jwt::{create_token, validate_token};
///
/// let token = create_token("user_123").expect("Failed to create token");
/// let claims = validate_token(&token).expect("Failed to validate token");
/// assert_eq!(claims.sub, "user_123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_change_in_production".to_string());

=======
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error as StdError;
use std::fmt;

/// JWT Claims: subject (user id), expiry, and issued-at timestamps (seconds since epoch).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
}

/// Errors for JWT creation and validation.
#[derive(Debug)]
pub enum AuthError {
    /// Environment variable `JWT_SECRET` was not set.
    MissingSecret,
    /// `JWT_EXP_SECONDS` contained an invalid value.
    InvalidExpirationVar(String),
    /// Underlying jsonwebtoken error.
    Jwt(jsonwebtoken::errors::Error),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSecret => write!(f, "JWT secret not configured (JWT_SECRET)"),
            Self::InvalidExpirationVar(v) => write!(f, "Invalid JWT_EXP_SECONDS value: {v}"),
            Self::Jwt(e) => write!(f, "JWT error: {e}"),
        }
    }
}

impl StdError for AuthError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Jwt(e) => Some(e),
            _ => None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(err)
    }
}

/// Time provider abstraction for testability.
pub trait Clock {
    /// Returns the current UNIX timestamp in seconds.
    fn now_seconds(&self) -> u64;
}

/// Default clock that uses the system time.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_seconds(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        // Narrow-scoped allowance: we intentionally use SystemTime in the default clock
        // to bridge to real time, while the rest of the code depends on the Clock trait
        // for testability.
        #[allow(clippy::disallowed_methods)]
        let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return 0;
        };
        dur.as_secs()
    }
}

/// Read the JWT HMAC secret from the `JWT_SECRET` environment variable.
fn read_secret() -> Result<String, AuthError> {
    env::var("JWT_SECRET").map_err(|_| AuthError::MissingSecret)
}

/// Read token TTL from `JWT_EXP_SECONDS` (defaults to 86400 = 24h if not set).
fn read_ttl_seconds() -> Result<u64, AuthError> {
    match env::var("JWT_EXP_SECONDS") {
        Ok(v) => v
            .parse::<u64>()
            .map_err(|_| AuthError::InvalidExpirationVar(v)),
        Err(_) => Ok(24 * 3600),
    }
}

/// Create a JWT for the given user id using environment configuration.
///
/// Environment:
/// - `JWT_SECRET` (required): HMAC secret used to sign the token
/// - `JWT_EXP_SECONDS` (optional, default 86400): token validity window
///
/// # Errors
/// Returns `AuthError::MissingSecret` if `JWT_SECRET` is not set or
/// `AuthError::InvalidExpirationVar` if `JWT_EXP_SECONDS` is invalid.
pub fn create_token(user_id: &str) -> Result<String, AuthError> {
    let secret = read_secret()?;
    let ttl = read_ttl_seconds()?;
    create_token_with(user_id, &SystemClock, secret.as_bytes(), ttl)
}

/// Create a JWT with an injected clock, secret and TTL (seconds).
/// Intended for testing and advanced integrations.
///
/// # Errors
/// Returns a [`AuthError::Jwt`] if encoding fails.
pub fn create_token_with(
    user_id: &str,
    clock: &dyn Clock,
    secret: &[u8],
    ttl_seconds: u64,
) -> Result<String, AuthError> {
    let now = clock.now_seconds();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now,
        exp: now.saturating_add(ttl_seconds),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(Into::into)
}

/// Validate a JWT using the secret from `JWT_SECRET` and return its claims.
///
/// # Errors
/// Returns `AuthError::MissingSecret` if `JWT_SECRET` is not set or
/// `AuthError::Jwt` if the token is invalid or expired.
pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let secret = read_secret()?;
>>>>>>> de7b56476 (feat(auth): implement JWT + Argon2 auth module)
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
<<<<<<< HEAD

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let user_id = "user_123";
        let token = create_token(user_id);

        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_validate_token_success() {
        let user_id = "user_456";
        let token = create_token(user_id).expect("Failed to create token");

        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration_is_24_hours() {
        let user_id = "user_789";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        #[allow(clippy::disallowed_methods)]
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let expected_expiration = now + 24 * 3600;

        // Cast to i64 for safe arithmetic in test - potential wrapping is acceptable in tests
        #[allow(clippy::cast_possible_wrap)]
        let exp_diff = (claims.exp as i64 - expected_expiration as i64).abs();

        // Allow 2 second tolerance for test execution time
        assert!(exp_diff <= 2);
    }

    #[test]
    fn test_validate_invalid_token() {
        let invalid_token = "invalid.token.string";
        let result = validate_token(invalid_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tampered_token() {
        let user_id = "user_original";
        let token = create_token(user_id).expect("Failed to create token");

        // Tamper with the token by modifying a character
        let mut tampered = token;
        tampered.push('x');

        let result = validate_token(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let user_id = "user_claims_test";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > 0);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);

        // Verify expiration is approximately 24 hours after issued time
        let duration = claims.exp - claims.iat;
        let expected_duration = 24 * 3600;
        assert_eq!(duration, expected_duration);
    }

    #[test]
    fn test_different_users_different_tokens() {
        let token1 = create_token("user_1").expect("Failed to create token");
        let token2 = create_token("user_2").expect("Failed to create token");

        assert_ne!(token1, token2);

        let claims1 = validate_token(&token1).expect("Failed to validate token");
        let claims2 = validate_token(&token2).expect("Failed to validate token");

        assert_eq!(claims1.sub, "user_1");
        assert_eq!(claims2.sub, "user_2");
    }

    #[test]
    fn test_empty_user_id() {
        let token = create_token("").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let user_id = "user@example.com";
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id);
    }
}
=======
    Ok(token_data.claims)
}
>>>>>>> de7b56476 (feat(auth): implement JWT + Argon2 auth module)
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error as StdError;
use std::fmt;

/// Minimum recommended length (in bytes) for HS256 secrets.
const MIN_SECRET_LEN: usize = 32;

/// JWT Claims: subject (user id), expiry, and issued-at timestamps (seconds since epoch).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
}

/// Errors for JWT creation and validation.
#[derive(Debug)]
pub enum AuthError {
    /// Environment variable `JWT_SECRET` was not set.
    MissingSecret,
    /// `JWT_EXP_SECONDS` contained an invalid value.
    InvalidExpirationVar(String),
    /// The configured `JWT_SECRET` is too short for HS256.
    WeakSecret,
    /// Underlying jsonwebtoken error.
    Jwt(jsonwebtoken::errors::Error),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSecret => write!(f, "JWT secret not configured (JWT_SECRET)"),
            Self::InvalidExpirationVar(v) => write!(f, "Invalid JWT_EXP_SECONDS value: {v}"),
            Self::WeakSecret => write!(
                f,
                "JWT secret too short; require at least {MIN_SECRET_LEN} bytes"
            ),
            Self::Jwt(e) => write!(f, "JWT error: {e}"),
        }
    }
}

impl StdError for AuthError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Jwt(e) => Some(e),
            _ => None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(err)
    }
}

/// Time provider abstraction for testability.
pub trait Clock {
    /// Returns the current UNIX timestamp in seconds.
    fn now_seconds(&self) -> u64;
}

/// Default clock that uses the system time.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_seconds(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        // Narrow-scoped allowance: we intentionally use SystemTime in the default clock
        // to bridge to real time, while the rest of the code depends on the Clock trait
        // for testability.
        #[allow(clippy::disallowed_methods)]
        let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return 0;
        };
        dur.as_secs()
    }
}

/// Read the JWT HMAC secret from the `JWT_SECRET` environment variable.
fn read_secret() -> Result<String, AuthError> {
    let secret = env::var("JWT_SECRET").map_err(|_| AuthError::MissingSecret)?;
    if secret.len() < MIN_SECRET_LEN {
        return Err(AuthError::WeakSecret);
    }
    Ok(secret)
}

/// Read token TTL from `JWT_EXP_SECONDS` (defaults to 86400 = 24h if not set).
fn read_ttl_seconds() -> Result<u64, AuthError> {
    match env::var("JWT_EXP_SECONDS") {
        Ok(v) => v
            .parse::<u64>()
            .map_err(|_| AuthError::InvalidExpirationVar(v)),
        Err(_) => Ok(24 * 3600),
    }
}

/// Create a JWT for the given user id using environment configuration.
///
/// Environment:
/// - `JWT_SECRET` (required): HMAC secret used to sign the token
/// - `JWT_EXP_SECONDS` (optional, default 86400): token validity window
///
/// # Errors
/// Returns `AuthError::MissingSecret` if `JWT_SECRET` is not set or
/// `AuthError::InvalidExpirationVar` if `JWT_EXP_SECONDS` is invalid.
pub fn create_token(user_id: &str) -> Result<String, AuthError> {
    let secret = read_secret()?;
    let ttl = read_ttl_seconds()?;
    create_token_with(user_id, &SystemClock, secret.as_bytes(), ttl)
}

/// Create a JWT with an injected clock, secret and TTL (seconds).
/// Intended for testing and advanced integrations.
///
/// # Errors
/// Returns a [`AuthError::Jwt`] if encoding fails.
pub fn create_token_with(
    user_id: &str,
    clock: &dyn Clock,
    secret: &[u8],
    ttl_seconds: u64,
) -> Result<String, AuthError> {
    let now = clock.now_seconds();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now,
        exp: now.saturating_add(ttl_seconds),
    };

    // Explicitly set HS256 to avoid algorithm confusion
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(Into::into)
}

/// Validate a JWT using the secret from `JWT_SECRET` and return its claims.
///
/// # Errors
/// Returns `AuthError::MissingSecret` if `JWT_SECRET` is not set or
/// `AuthError::Jwt` if the token is invalid or expired.
pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let secret = read_secret()?;
    // Restrict validation to HS256 explicitly
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
