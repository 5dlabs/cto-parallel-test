//! JWT creation and validation utilities.
//!
//! Security notes:
//! - The JWT secret is REQUIRED and must be provided via the `JWT_SECRET` environment variable.
//! - Token expiration defaults to 24 hours and can be overridden with `JWT_EXP_HOURS`.

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Claims carried in issued JWTs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (seconds since UNIX epoch)
    pub exp: u64,
    /// Issued at (seconds since UNIX epoch)
    pub iat: u64,
    /// Not before (seconds since UNIX epoch). Set to `iat` to prevent early use.
    pub nbf: u64,
    /// Issuer (optional). If `JWT_ISSUER` is set, tokens are issued with this and validated against it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Audience (optional). If `JWT_AUDIENCE` is set, tokens are issued with this and validated against it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
}

/// Errors that can occur when creating or validating JWTs.
#[derive(Debug)]
pub enum JwtError {
    /// JWT secret not present in environment.
    MissingSecret,
    /// JWT secret is too weak (length requirement not met).
    WeakSecret,
    /// Underlying jsonwebtoken error.
    Jwt(jsonwebtoken::errors::Error),
    /// Invalid expiration value provided via configuration.
    InvalidExpiration,
}

impl Display for JwtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSecret => write!(f, "JWT secret not set (expected env var JWT_SECRET)"),
            Self::Jwt(e) => write!(f, "JWT error: {e}"),
            Self::InvalidExpiration => write!(f, "Invalid JWT expiration configuration"),
            Self::WeakSecret => write!(f, "JWT secret is too weak (min 32 bytes)"),
        }
    }
}

impl std::error::Error for JwtError {}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(value)
    }
}

/// Abstraction for retrieving current wall-clock time.
trait Clock {
    /// Returns seconds since UNIX epoch.
    fn now_secs(&self) -> u64;
}

/// System clock implementation.
struct SystemClock;

impl Clock for SystemClock {
    #[allow(clippy::disallowed_methods)] // Centralized abstraction over SystemTime for testability.
    fn now_secs(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }
}

/// Reads JWT secret from the `JWT_SECRET` environment variable.
fn jwt_secret() -> Result<String, JwtError> {
    let secret = env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)?;
    // Enforce minimum entropy: 32 raw bytes (256 bits) for HMAC-SHA256.
    if secret.len() < 32 {
        return Err(JwtError::WeakSecret);
    }
    Ok(secret)
}

/// Reads expiration hours from `JWT_EXP_HOURS` (defaults to 24).
fn expiration_hours() -> Result<u64, JwtError> {
    match env::var("JWT_EXP_HOURS") {
        Ok(val) => val.parse::<u64>().map_err(|_| JwtError::InvalidExpiration),
        Err(_) => Ok(24),
    }
}

/// Returns validation leeway (in seconds) to tolerate minor clock skew.
/// Configured via `JWT_LEEWAY_SECS` with default 60 and max 300.
fn leeway_secs() -> u64 {
    const DEFAULT: u64 = 60;
    const MAX: u64 = 300;
    match env::var("JWT_LEEWAY_SECS") {
        Ok(v) => v.parse::<u64>().map(|n| n.min(MAX)).unwrap_or(DEFAULT),
        Err(_) => DEFAULT,
    }
}

/// Creates a signed JWT for the provided `user_id`.
///
/// # Errors
/// Returns [`JwtError::MissingSecret`] if `JWT_SECRET` is unset, or wraps
/// underlying signing failures as [`JwtError::Jwt`]. May return
/// [`JwtError::InvalidExpiration`] if configured expiration is invalid.
pub fn create_token(user_id: &str) -> Result<String, JwtError> {
    create_token_with_clock(user_id, &SystemClock)
}

fn create_token_with_clock(user_id: &str, clock: &dyn Clock) -> Result<String, JwtError> {
    let iat = clock.now_secs();
    // Compute expiration in seconds safely (prevent integer overflow).
    let exp_secs = expiration_hours()?
        .checked_mul(3600)
        .ok_or(JwtError::InvalidExpiration)?;
    let exp = iat
        .checked_add(exp_secs)
        .ok_or(JwtError::InvalidExpiration)?;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat,
        nbf: iat,
        iss: env::var("JWT_ISSUER").ok(),
        aud: env::var("JWT_AUDIENCE").ok(),
    };

    let secret = jwt_secret()?;
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

/// Validates a JWT and returns its claims if valid.
///
/// # Errors
/// Returns [`JwtError::MissingSecret`] if `JWT_SECRET` is unset, or wraps
/// underlying validation failures as [`JwtError::Jwt`].
pub fn validate_token(token: &str) -> Result<Claims, JwtError> {
    let secret = jwt_secret()?;
    let mut validation = Validation::new(Algorithm::HS256);
    // Enforce `nbf` and expiration with small leeway to tolerate minor clock skew.
    validation.validate_nbf = true;
    validation.validate_exp = true;
    validation.leeway = leeway_secs();
    // Enforce issuer/audience checks if configured via environment.
    if let Ok(issuer) = env::var("JWT_ISSUER") {
        let mut set = HashSet::new();
        set.insert(issuer);
        validation.iss = Some(set);
    }
    if let Ok(aud) = env::var("JWT_AUDIENCE") {
        let mut set = HashSet::new();
        set.insert(aud);
        validation.aud = Some(set);
    }
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
