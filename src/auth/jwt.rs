use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (Unix timestamp)
    pub iat: usize,  // Issued at (Unix timestamp)
}

/// Creates a JWT token for the given user ID.
/// The token expires after 24 hours.
///
/// # Errors
/// Returns an error if token encoding fails.
///
/// # Panics
/// Panics if system time is before Unix epoch (extremely rare).
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::disallowed_methods)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before Unix epoch")
        .as_secs() as usize;

    let expiration = now + 86400; // 24 hours in seconds

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now,
    };

    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and returns the claims if valid.
///
/// # Errors
/// Returns an error if the token is invalid, expired, or malformed.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
