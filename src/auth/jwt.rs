use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Token TTL in seconds. Default 24h; configurable via JWT_TTL_SECS.
// Clamp to a safe range to avoid extremely long-lived tokens due to misconfiguration.
const DEFAULT_TTL_SECS: u64 = 24 * 3600; // 24h
const MAX_TTL_SECS: u64 = 7 * 24 * 3600; // 7 days

/// JWT claims for authentication tokens.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    /// Subject (user id)
    pub sub: String,
    /// Expiration time (as seconds since epoch)
    pub exp: u64,
    /// Issued at (as seconds since epoch)
    pub iat: u64,
}

/// Internal abstraction for time to enable testability while keeping clippy happy.
trait Clock {
    fn now_unix(&self) -> u64;
}

struct SystemClock;

impl Clock for SystemClock {
    fn now_unix(&self) -> u64 {
        // Justification: This is the boundary where we obtain wall-clock time.
        // A Clock abstraction is used so tests can simulate time without using SystemTime directly.
        #[allow(clippy::disallowed_methods)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0));
        now.as_secs()
    }
}

fn get_secret_from_env() -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
    })?;

    // Enforce a minimum length to reduce risk of weak HMAC keys.
    // 32 bytes is a practical baseline for HS256.
    // Allow configuring the minimum via env while enforcing a hard floor of 32.
    let min_len: usize = std::env::var("JWT_SECRET_MIN_LEN")
        .ok()
        .and_then(|v| v.parse().ok())
        .map_or(32, |v: usize| v.max(32));

    if secret.len() < min_len {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(secret)
}

fn encoding_key_from_env() -> Result<EncodingKey, jsonwebtoken::errors::Error> {
    let secret = get_secret_from_env()?;
    Ok(EncodingKey::from_secret(secret.as_bytes()))
}

fn decoding_key_from_env() -> Result<DecodingKey, jsonwebtoken::errors::Error> {
    let secret = get_secret_from_env()?;
    Ok(DecodingKey::from_secret(secret.as_bytes()))
}

/// Create a signed JWT for the given user id with configured expiration.
///
/// # Errors
/// Returns an error if the `JWT_SECRET` environment variable is not set or is too short,
/// or if token signing fails.
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_clock(user_id, &SystemClock)
}

fn create_token_with_clock(
    user_id: &str,
    clock: &dyn Clock,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = clock.now_unix();
    let ttl_secs: u64 = std::env::var("JWT_TTL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v: &u64| *v > 0)
        .map_or(DEFAULT_TTL_SECS, |v| v.min(MAX_TTL_SECS));
    let expiration = now + ttl_secs; // default: 24 hours from now

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: now,
    };

    let key = encoding_key_from_env()?;
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &key)
}

/// Validate a JWT and return its claims if valid.
///
/// # Errors
/// Returns an error if the `JWT_SECRET` environment variable is not set/too short,
/// if the token is malformed, signed with a different secret, or expired.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = decoding_key_from_env()?;
    // Explicitly restrict to HS256 to avoid algorithm confusion
    // and allow a small leeway for clock skew when validating exp.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 30; // seconds
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod internal_tests {
    use super::*;
    use serial_test::serial;

    struct FixedClock(u64);

    impl Clock for FixedClock {
        fn now_unix(&self) -> u64 {
            self.0
        }
    }

    #[test]
    #[serial]
    #[allow(clippy::disallowed_methods)]
    fn create_token_with_fixed_clock_sets_fields() {
        std::env::set_var("JWT_SECRET", "test_secret_key_minimum_32_chars_long______");
        // Use real wall-clock time for fresh, non-expired tokens in test
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let clock = FixedClock(now);
        let token = super::create_token_with_clock("user123", &clock).expect("token");
        let claims = validate_token(&token).expect("validate");
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.iat, now);
        assert_eq!(claims.exp, now + 86_400);
    }
}
