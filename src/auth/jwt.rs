use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
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
}

fn current_timestamp_secs() -> u64 {
    #[allow(clippy::disallowed_methods)]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_only_signing_key_min_32_chars________".to_string())
}

/// Create a signed JWT for the provided user identifier with a 24-hour expiration.
///
/// # Errors
/// Returns any signing or encoding error produced by the `jsonwebtoken` crate.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = current_timestamp_secs();
    let expiration = now + 24 * 3600; // 24 hours from now

    let exp = usize::try_from(expiration)
        .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;
    let iat = usize::try_from(now)
        .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat,
    };

    let secret = jwt_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validate a JWT returning its decoded claims when valid.
///
/// # Errors
/// Returns an error when the token is malformed, expired, or signed with a different secret.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = jwt_secret();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
