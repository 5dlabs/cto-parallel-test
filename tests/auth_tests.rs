use cto_parallel_test::auth::{create_token, validate_token, User};

#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
}

#[test]
fn test_password_hashing_and_verification() {
    let password = "secure_password";
    let hashed = User::hash_password(password);
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };
    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));
}

#[test]
fn test_password_hash_is_unique() {
    let password = "same_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Each hash should be different due to random salt
    assert_ne!(hash1, hash2);
}

#[test]
fn test_user_serialization_skips_password() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash123".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    assert!(!json.contains("password_hash"));
    assert!(json.contains("username"));
    assert!(json.contains("testuser"));
}

#[test]
fn test_invalid_token_rejected() {
    let result = validate_token("invalid.token.here");
    assert!(result.is_err());
}
