use super::*;

#[test]
fn test_password_hashing() {
    let password = "test_password_123";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

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
fn test_invalid_token() {
    std::env::set_var("JWT_SECRET", "test_secret_key_minimum_32_chars_long______");
    let invalid_token = "invalid.token.here";
    assert!(crate::auth::jwt::validate_token(invalid_token).is_err());
}
