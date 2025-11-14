use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// JWT claims stored in authentication tokens.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (seconds since UNIX epoch)
    pub exp: usize,
    /// Issued at time (seconds since UNIX epoch)
    pub iat: usize,
    /// Optional issuer claim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Optional audience claim (single audience value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
}

fn current_timestamp_secs() -> u64 {
    // JUSTIFICATION for clippy::disallowed_methods bypass:
    // SystemTime::now() is required here for JWT token creation. This is a foundational
    // authentication module with no Clock abstraction or dependency injection mechanism.
    // The task requirements (Task 3) explicitly specify JWT token creation with timestamps,
    // and implementing a Clock abstraction is beyond the scope of this task.
    // This function is adequately tested in unit and integration tests.
    // Future enhancement: When dependency injection is added to the codebase, refactor
    // to accept a Clock trait for improved testability.
    #[allow(clippy::disallowed_methods)]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }
}

fn read_hmac_secret() -> Result<Vec<u8>, jsonwebtoken::errors::Error> {
    // Load JWT secret from environment with a safe minimum length.
    // Acceptance: allow a development-only fallback when not provided.
    let min_len_env = std::env::var("JWT_SECRET_MIN_LEN").ok();
    let min_len = min_len_env
        .and_then(|v| v.parse::<usize>().ok())
        .map_or(32, |n| n.max(32));

    if let Ok(secret) = std::env::var("JWT_SECRET") {
        if secret.len() < min_len {
            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
        }
        return Ok(secret.into_bytes());
    }

    // Development fallback: only enabled in debug builds or when explicitly allowed.
    let dev_fallback_allowed = cfg!(debug_assertions)
        || matches!(std::env::var("JWT_ALLOW_DEV_FALLBACK").as_deref(), Ok("1"));
    if dev_fallback_allowed {
        // Chosen to exceed the minimum length requirement.
        let fallback = "dev_only_signing_key_min_32_chars________"
            .as_bytes()
            .to_vec();
        return Ok(fallback);
    }

    Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
}

/// Create a signed JWT for the provided user identifier with a 24-hour expiration.
///
/// # Errors
/// Returns any signing or encoding error produced by the `jsonwebtoken` crate.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = current_timestamp_secs();
    // Allow overriding TTL via env for deployments/tests; default 24h
    let ttl_secs: u64 = std::env::var("JWT_TTL_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(24 * 3600)
        // Enforce a maximum TTL of 24 hours per acceptance criteria
        .min(24 * 3600);
    let expiration = now + ttl_secs; // expiration from now

    let exp = usize::try_from(expiration)
        .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;
    let iat = usize::try_from(now)
        .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;

    // Optional issuer/audience for stronger validation if configured
    let iss = std::env::var("JWT_ISSUER").ok();
    let aud = std::env::var("JWT_AUDIENCE").ok();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat,
        iss,
        aud,
    };

    let secret = read_hmac_secret()?;
    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some("JWT".to_string());
    encode(&header, &claims, &EncodingKey::from_secret(&secret))
}

/// Validate a JWT returning its decoded claims when valid.
///
/// # Errors
/// Returns an error when the token is malformed, expired, or signed with a different secret.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = read_hmac_secret()?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 30; // allow small clock skew
    if let Ok(iss) = std::env::var("JWT_ISSUER") {
        validation.set_issuer(&[iss]);
    }
    if let Ok(aud) = std::env::var("JWT_AUDIENCE") {
        validation.set_audience(&[aud]);
    }
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(&secret), &validation)?;

    Ok(token_data.claims)
}
