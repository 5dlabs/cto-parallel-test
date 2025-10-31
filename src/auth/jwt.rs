use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

const SECRET: &[u8] = b"test_secret_key";

#[allow(clippy::disallowed_methods)]
fn current_timestamp() -> usize {
    // Justification: For this test project, we use wall-clock time to set JWT
    // issued-at and expiration. In production, this would be abstracted behind
    // a Clock trait for testability per coding-guidelines.
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0));
    usize::try_from(now.as_secs()).unwrap_or(usize::MAX)
}

pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let iat = current_timestamp();
    // 24 hours expressed in seconds
    let exp = iat.saturating_add(24 * 60 * 60);

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
