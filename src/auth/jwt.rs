use super::clock::{Clock, SystemClock};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: u64,    // Expiration time (seconds since UNIX epoch)
    pub iat: u64,    // Issued at (seconds since UNIX epoch)
}

/// Error type for JWT creation/validation issues.
#[derive(Debug)]
pub enum JwtError {
    MissingSecret,
    WeakSecret,
    InvalidExpiration,
    Jwt(jsonwebtoken::errors::Error),
}

impl Display for JwtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSecret => write!(f, "JWT secret not set (env JWT_SECRET)"),
            Self::WeakSecret => write!(f, "JWT secret too weak (min 32 bytes)"),
            Self::InvalidExpiration => write!(f, "Invalid JWT expiration configuration"),
            Self::Jwt(e) => write!(f, "JWT error: {e}"),
        }
    }
}

impl std::error::Error for JwtError {}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(e)
    }
}

fn read_secret() -> Result<String, JwtError> {
    let secret = env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)?;
    if secret.len() < 32 {
        return Err(JwtError::WeakSecret);
    }
    Ok(secret)
}

fn exp_hours() -> Result<u64, JwtError> {
    match env::var("JWT_EXP_HOURS") {
        Ok(v) => v.parse::<u64>().map_err(|_| JwtError::InvalidExpiration),
        Err(_) => Ok(24),
    }
}

fn leeway_secs() -> u64 {
    const DEFAULT: u64 = 60;
    const MAX: u64 = 300;
    match env::var("JWT_LEEWAY_SECS") {
        Ok(v) => v.parse::<u64>().map(|n| n.min(MAX)).unwrap_or(DEFAULT),
        Err(_) => DEFAULT,
    }
}

/// Create a JWT token for the given user ID with 24-hour expiration
///
/// Security: the JWT secret must be provided via `JWT_SECRET` and be >= 32 bytes.
///
/// # Errors
///
/// Returns [`JwtError::MissingSecret`] if `JWT_SECRET` is unset, [`JwtError::WeakSecret`]
/// if the secret is shorter than 32 bytes, [`JwtError::InvalidExpiration`] if the
/// configured expiration cannot be represented safely, or wraps underlying
/// `jsonwebtoken` errors as [`JwtError::Jwt`].
pub fn create_token(user_id: &str) -> Result<String, JwtError> {
    create_token_with_clock(user_id, &SystemClock)
}

/// Create a JWT token with a custom clock (for testing)
fn create_token_with_clock(user_id: &str, clock: &dyn Clock) -> Result<String, JwtError> {
    let now = clock.now_seconds();
    let exp_delta = exp_hours()?;
    // Safe expiration calculation with overflow checks
    let exp_secs = exp_delta
        .checked_mul(3600)
        .ok_or(JwtError::InvalidExpiration)?;
    let expiration = now
        .checked_add(exp_secs)
        .ok_or(JwtError::InvalidExpiration)?;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: now,
    };

    let secret = read_secret()?;

    Ok(encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

/// Validate a JWT token and extract claims
///
/// # Errors
///
/// Returns [`JwtError::MissingSecret`] if `JWT_SECRET` is unset, [`JwtError::WeakSecret`]
/// if the secret is too short, or wraps signature/format/expiration failures from
/// `jsonwebtoken` as [`JwtError::Jwt`].
pub fn validate_token(token: &str) -> Result<Claims, JwtError> {
    let secret = read_secret()?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    // jsonwebtoken does not validate nbf by default; set leeway for robustness
    validation.leeway = leeway_secs();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::super::clock::MockClock;
    use super::*;
    use crate::test_support::env_lock;
    use rand_core::OsRng;
    use rand_core::RngCore;

    fn ensure_secret() {
        // Generate a cryptographically-secure random secret for tests (48 bytes, hex-encoded)
        let mut buf = [0u8; 48];
        OsRng.fill_bytes(&mut buf);
        let secret = hex_string(&buf);
        std::env::set_var("JWT_SECRET", secret);
        // Ensure default of 24 hours unless overridden by tests.
        std::env::remove_var("JWT_EXP_HOURS");
    }

    fn hex_string(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0f) as usize] as char);
        }
        out
    }

    #[test]
    fn test_token_creation_succeeds() {
        let _g = env_lock();
        ensure_secret();
        let token = create_token("123").unwrap();
        assert!(!token.is_empty());
        assert!(token.contains('.'));
    }

    #[test]
    fn test_token_validation_succeeds_with_valid_token() {
        let _g = env_lock();
        ensure_secret();
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_invalid_token_is_rejected() {
        let _g = env_lock();
        ensure_secret();
        let invalid_token = "invalid.token.here";
        assert!(validate_token(invalid_token).is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let _g = env_lock();
        ensure_secret();
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "123");
        assert_eq!(claims.exp - claims.iat, 86_400);
    }

    #[test]
    fn test_expiration_is_24_hours_in_future() {
        let _g = env_lock();
        ensure_secret();
        let token = create_token("123").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.exp - claims.iat, 86_400);
    }

    #[test]
    fn test_same_user_produces_different_tokens_due_to_timestamp() {
        let _g = env_lock();
        ensure_secret();
        let token1 = create_token("123").unwrap();
        // Small delay to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let token2 = create_token("123").unwrap();

        // Decode both tokens to verify they have different timestamps
        let claims1 = validate_token(&token1).unwrap();
        let claims2 = validate_token(&token2).unwrap();

        // Timestamps should be different (at least 1 second apart)
        assert!(claims2.iat > claims1.iat);
        assert!(claims2.exp > claims1.exp);
    }

    #[test]
    fn test_tampered_token_is_rejected() {
        let _g = env_lock();
        ensure_secret();
        let token = create_token("123").unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.chars().collect::<Vec<_>>();
        if let Some(c) = tampered.last_mut() {
            *c = if *c == 'a' { 'b' } else { 'a' };
        }
        let tampered_token: String = tampered.into_iter().collect();

        assert!(validate_token(&tampered_token).is_err());
    }

    #[test]
    fn test_empty_user_id_is_handled() {
        let _g = env_lock();
        ensure_secret();
        let token = create_token("").unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "");
    }

    #[test]
    fn test_long_user_id_is_handled() {
        let _g = env_lock();
        ensure_secret();
        let long_id = "a".repeat(1000);
        let token = create_token(&long_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, long_id);
    }

    #[test]
    fn test_special_characters_in_user_id() {
        let _g = env_lock();
        ensure_secret();
        let special_id = "user!@#$%^&*()123";
        let token = create_token(special_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, special_id);
    }

    #[test]
    fn test_mock_clock_for_deterministic_timestamps() {
        // Test that the clock abstraction works correctly
        let clock = MockClock {
            timestamp: 2_000_000_000, // Future timestamp to avoid expiration
        };
        // Use a temporary secret for this test
        let _g = env_lock();
        ensure_secret();
        let token = create_token_with_clock("test_user", &clock).unwrap();

        // Note: We can't validate this token because validate_token uses current time
        // and the token would be invalid. This test verifies token creation works with mock clock.
        assert!(!token.is_empty());

        // Create another token with same clock - should have identical timestamps
        let token2 = create_token_with_clock("test_user", &clock).unwrap();
        assert_eq!(token, token2);
    }
}
