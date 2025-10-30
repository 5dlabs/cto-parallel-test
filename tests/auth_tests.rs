use cto_parallel_test::auth::{jwt, models::User};

#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = jwt::create_token(user_id).unwrap();
    let claims = jwt::validate_token(&token).unwrap();
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
fn test_password_hashing_produces_different_hashes() {
    let password = "test_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Due to random salt, same password should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_user_serialization_skips_password_hash() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash123".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    assert!(!json.contains("password_hash"));
    assert!(json.contains("username"));
}

#[test]
fn test_invalid_token() {
    let result = jwt::validate_token("invalid_token");
    assert!(result.is_err());
}
