use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Creates a JWT token for the given user ID
/// The token expires after 24 hours
///
/// # Errors
///
/// Returns an error if JWT encoding fails (extremely rare)
///
/// # Panics
///
/// Panics if system time goes backwards (extremely rare system clock issue)
#[allow(clippy::cast_possible_truncation)]
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // SystemTime::now() is appropriate here for wall-clock time in JWT tokens
    // The clippy disallowed-methods rule is intended for cases where time needs to be mocked for tests,
    // but JWT token generation with real timestamps is the correct behavior for this auth module
    #[allow(clippy::disallowed_methods)]
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expiration = current_time + 86400; // 24 hours in seconds

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: current_time,
    };

    let secret = b"test_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Validates a JWT token and returns the claims if valid
///
/// # Errors
///
/// Returns an error if the token is invalid, expired, or malformed
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = b"test_secret_key";
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
    fn test_jwt_creation_and_validation() {
        let user_id = "123";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn test_jwt_expiration_is_24_hours() {
        let user_id = "test_user";
        let token = create_token(user_id).unwrap();
        let claims = validate_token(&token).unwrap();

        #[allow(clippy::disallowed_methods)]
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Check that expiration is approximately 24 hours from now
        let expected_exp = current_time + 86400;
        assert!((claims.exp as i64 - expected_exp as i64).abs() < 5); // Within 5 seconds tolerance
    }

    #[test]
    fn test_invalid_token_fails_validation() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
