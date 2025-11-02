use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

// Test fixture for JWT operations
const JWT_TEST_KEY: &str = "test-jwt-secret";

/// Helper function to convert timestamp to usize
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn timestamp_to_usize(timestamp: i64) -> usize {
    timestamp as usize
}

#[test]
fn test_password_hashing() {
    let password = "my_secure_password";

    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    assert_ne!(password, hashed);
    assert!(hashed.starts_with("$2"));
}

#[test]
fn test_password_verification_success() {
    let password = "correct_password";
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let result = verify(password, &hashed).expect("Failed to verify password");

    assert!(result);
}

#[test]
fn test_password_verification_failure() {
    let password = "correct_password";
    let wrong_password = "wrong_password";
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let result = verify(wrong_password, &hashed).expect("Failed to verify password");

    assert!(!result);
}

#[test]
fn test_password_hashing_different_results() {
    let password = "same_password";

    let hash1 = hash(password, DEFAULT_COST).expect("Failed to hash password");
    let hash2 = hash(password, DEFAULT_COST).expect("Failed to hash password");

    // Same password should produce different hashes (due to salt)
    assert_ne!(hash1, hash2);

    // But both should verify successfully
    assert!(verify(password, &hash1).unwrap());
    assert!(verify(password, &hash2).unwrap());
}

#[test]
fn test_jwt_token_creation() {
    let now = Utc::now();
    let claims = Claims {
        sub: "user123".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    assert!(!token.is_empty());
    assert!(token.contains('.'));
}

#[test]
fn test_jwt_token_validation_success() {
    let now = Utc::now();
    let claims = Claims {
        sub: "user456".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .expect("Failed to decode token");

    assert_eq!(token_data.claims.sub, "user456");
}

#[test]
fn test_jwt_token_validation_invalid_secret() {
    let now = Utc::now();
    let claims = Claims {
        sub: "user789".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    let wrong_secret = "wrong_secret";
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(wrong_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_jwt_token_expiration() {
    let now = Utc::now();
    let claims = Claims {
        sub: "expired_user".to_string(),
        iat: timestamp_to_usize((now - Duration::hours(48)).timestamp()),
        exp: timestamp_to_usize((now - Duration::hours(24)).timestamp()), // Expired
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_jwt_token_different_users() {
    let now = Utc::now();

    let claims1 = Claims {
        sub: "user1".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let claims2 = Claims {
        sub: "user2".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token1 = encode(
        &Header::default(),
        &claims1,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create token 1");

    let token2 = encode(
        &Header::default(),
        &claims2,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create token 2");

    assert_ne!(token1, token2);

    let decoded1 = decode::<Claims>(
        &token1,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .unwrap();

    let decoded2 = decode::<Claims>(
        &token2,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .unwrap();

    assert_eq!(decoded1.claims.sub, "user1");
    assert_eq!(decoded2.claims.sub, "user2");
}

#[test]
fn test_jwt_token_malformed() {
    let malformed_token = "this.is.not.a.valid.jwt.token";

    let result = decode::<Claims>(
        malformed_token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_jwt_token_empty() {
    let empty_token = "";

    let result = decode::<Claims>(
        empty_token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_password_empty() {
    let password = "";
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash empty password");

    assert!(verify(password, &hashed).unwrap());
}

#[test]
fn test_password_special_characters() {
    let password = "P@ssw0rd!#$%^&*()_+-=[]{}|;:,.<>?";
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    assert!(verify(password, &hashed).unwrap());
}

#[test]
fn test_password_unicode() {
    let password = "пароль密码🔐";
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    assert!(verify(password, &hashed).unwrap());
}

#[test]
fn test_jwt_token_with_long_subject() {
    let now = Utc::now();
    let long_subject = "a".repeat(1000);

    let claims = Claims {
        sub: long_subject.clone(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .expect("Failed to decode token");

    assert_eq!(decoded.claims.sub, long_subject);
}

#[test]
fn test_authentication_flow() {
    // Simulate user registration
    let user_password = "secure_password123";
    let password_hash = hash(user_password, DEFAULT_COST).expect("Failed to hash password");

    // Simulate user login - verify password
    let login_attempt_password = "secure_password123";
    let is_valid = verify(login_attempt_password, &password_hash).expect("Failed to verify");
    assert!(is_valid);

    // Create JWT token after successful login
    let now = Utc::now();
    let claims = Claims {
        sub: "user_email@example.com".to_string(),
        iat: timestamp_to_usize(now.timestamp()),
        exp: timestamp_to_usize((now + Duration::hours(24)).timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_TEST_KEY.as_ref()),
    )
    .expect("Failed to create JWT token");

    // Validate token on subsequent requests
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_TEST_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .expect("Failed to decode token");

    assert_eq!(decoded.claims.sub, "user_email@example.com");
}

#[test]
fn test_failed_authentication_flow() {
    // Simulate user registration
    let user_password = "correct_password";
    let password_hash = hash(user_password, DEFAULT_COST).expect("Failed to hash password");

    // Simulate failed login - wrong password
    let login_attempt_password = "wrong_password";
    let is_valid = verify(login_attempt_password, &password_hash).expect("Failed to verify");

    assert!(!is_valid);
    // No token should be created for failed authentication
}
