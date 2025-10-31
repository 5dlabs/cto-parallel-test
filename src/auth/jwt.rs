use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for the given user ID.
///
/// The token will expire 24 hours after creation.
///
/// # Arguments
/// * `user_id` - The user identifier to encode in the token
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token or an error
///
/// # Errors
/// Returns an error if the JWT encoding fails.
///
/// # Panics
/// Panics if the system time is before the UNIX epoch.
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::create_token;
///
/// let token = create_token("user_123").unwrap();
/// assert!(!token.is_empty());
/// ```
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::cast_possible_truncation)]
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
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
/// # Arguments
/// * `token` - The JWT token string to validate
///
/// # Returns
/// * `Result<Claims, jsonwebtoken::errors::Error>` - The decoded claims or an error
///
/// # Errors
/// Returns an error if the token is invalid, expired, or has an invalid signature.
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::{create_token, validate_token};
///
/// let token = create_token("user_123").unwrap();
/// let claims = validate_token(&token).unwrap();
/// assert_eq!(claims.sub, "user_123");
/// ```
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
