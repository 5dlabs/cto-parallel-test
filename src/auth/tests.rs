use super::*;
use serial_test::serial;

#[test]
#[serial]
fn test_password_hashing() {
    let password = "test_password_123";
    let hash1 = User::hash_password(password).expect("hash1");
    let hash2 = User::hash_password(password).expect("hash2");

    // Hashes should be different (due to random salt)
    assert_ne!(hash1, hash2);

    // Both should verify correctly
    let user1 = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash1,
    };

    assert!(user1.verify_password(password));
    assert!(!user1.verify_password("wrong_password"));
}

#[test]
#[serial]
fn test_jwt_creation_and_validation() {
    std::env::set_var("JWT_SECRET", "test_secret_key_minimum_32_chars_long______");

    let user_id = "123";
    let token = crate::auth::jwt::create_token(user_id).unwrap();

    let claims = crate::auth::jwt::validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);

    // Check expiration is set
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
}

#[test]
#[serial]
fn test_invalid_token() {
    std::env::set_var("JWT_SECRET", "test_secret_key_minimum_32_chars_long______");
    let invalid_token = "invalid.token.here";
    assert!(crate::auth::jwt::validate_token(invalid_token).is_err());
}

#[test]
#[serial]
fn test_rejects_short_secret() {
    // Too short (<32 chars) should be rejected
    std::env::set_var("JWT_SECRET", "short_secret_key");
    let err = crate::auth::jwt::create_token("1").unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("InvalidToken") || msg.to_lowercase().contains("invalid token"));
}

#[test]
#[serial]
fn test_jwt_issuer_and_audience_optional_validation() {
    // Configure secret and optional issuer/audience
    std::env::set_var("JWT_SECRET", "test_secret_key_minimum_32_chars_long______");
    std::env::set_var("JWT_ISSUER", "acme-auth");
    std::env::set_var("JWT_AUDIENCE", "mobile");

    let token = crate::auth::jwt::create_token("user42").expect("token");

    // With matching env, validation should pass
    let claims = crate::auth::jwt::validate_token(&token).expect("validate ok");
    assert_eq!(claims.iss.as_deref(), Some("acme-auth"));
    assert_eq!(claims.aud.as_deref(), Some("mobile"));

    // Changing expected audience should cause validation failure
    std::env::set_var("JWT_AUDIENCE", "web");
    assert!(crate::auth::jwt::validate_token(&token).is_err());
}
