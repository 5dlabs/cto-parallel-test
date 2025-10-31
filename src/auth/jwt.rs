use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

// Lazily resolved JWT secret to avoid hardcoding secrets in the binary.
// The secret must be provided via the `JWT_SECRET` environment variable.
// In tests, we allow a fallback test value to keep examples simple.
static SECRET_BYTES: OnceLock<Vec<u8>> = OnceLock::new();

fn load_secret() -> Option<Vec<u8>> {
    if let Ok(s) = env::var("JWT_SECRET") {
        // Enforce a minimum length for HS256 secrets (>= 32 bytes recommended)
        let bytes = s.into_bytes();
        if bytes.len() >= 32 {
            return Some(bytes);
        }
        // If too short, refuse to use it
        return None;
    }
    None
}

fn get_secret() -> Result<&'static [u8], jsonwebtoken::errors::Error> {
    let bytes = SECRET_BYTES.get_or_init(|| {
        // Try env first
        if let Some(b) = load_secret() {
            return b;
        }
        // Last expression depends on build profile
        // Test-only fallback to keep unit tests hermetic without leaking secrets
        #[cfg(test)]
        {
            b"this_is_a_test_secret_key_with_sufficient_length".to_vec()
        }
        // In non-test builds, we do not allow an empty or hardcoded default
        #[cfg(not(test))]
        {
            Vec::new()
        }
    });

    if bytes.is_empty() {
        // Map to a jsonwebtoken error kind to keep API stable
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidKeyFormat,
        ));
    }
    Ok(bytes.as_slice())
}

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

/// Create a signed JWT for a given user ID.
///
/// # Errors
/// Returns a `jsonwebtoken::errors::Error` if token encoding fails.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let iat = current_timestamp();
    // 24 hours expressed in seconds
    let exp = iat.saturating_add(24 * 60 * 60);

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat,
    };

    let secret = get_secret()?;
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validate a JWT and return its claims if valid.
///
/// # Errors
/// Returns a `jsonwebtoken::errors::Error` if decoding or validation fails.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_secret()?;
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trip() {
        // Ensure a test secret is set and sufficiently long
        // Setting env vars may be unsafe in some contexts; restrict to tests
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var(
                "JWT_SECRET",
                "test_secret_key_value_that_is_plenty_long_for_hs256",
            );
        }

        let token = create_token("user-123").expect("token");
        let claims = validate_token(&token).expect("claims");
        assert_eq!(claims.sub, "user-123");
    }
}
