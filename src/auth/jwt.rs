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

