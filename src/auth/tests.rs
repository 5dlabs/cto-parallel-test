use crate::auth::jwt::{create_token, validate_token};
use crate::auth::models::User;

fn ensure_secret() {
    // Use a fixed secret for tests to avoid flakiness.
    std::env::set_var("JWT_SECRET", "test_secret_key_for_unit_tests_please_change");
    // Ensure default of 24 hours unless overridden by tests.
    std::env::remove_var("JWT_EXP_HOURS");
}

#[test]
fn test_password_hashing() {
    let test_pw = "test_password_123";
    let hash1 = User::hash_password(test_pw);
    let hash2 = User::hash_password(test_pw);

    // Hashes should be different (due to random salt)
    assert_ne!(hash1, hash2);

    // Both should verify correctly
    let user1 = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash1,
    };

    assert!(user1.verify_password(test_pw));
    assert!(!user1.verify_password("wrong_password"));
}

#[test]
fn test_jwt_creation_and_validation() {
    ensure_secret();

    let user_id = "123";
    let token = create_token(user_id).unwrap();

    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);

    // Check expiration and issued at are set
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_invalid_token() {
    ensure_secret();

    let invalid_token = "invalid.token.here";
    assert!(validate_token(invalid_token).is_err());
}
