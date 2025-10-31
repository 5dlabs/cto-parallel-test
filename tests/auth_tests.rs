use cto_parallel_test::auth::{create_token, validate_token, User};

#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).expect("Token creation failed");
    let claims = validate_token(&token).expect("Token validation failed");
    assert_eq!(claims.sub, user_id);
}

#[test]
fn test_password_hashing_produces_different_results() {
    let password = "test_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Different salts should produce different hashes
    assert_ne!(hash1, hash2);
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

    // Correct password should verify
    assert!(user.verify_password(password));

    // Incorrect password should not verify
    assert!(!user.verify_password("wrong_password"));
}

#[test]
fn test_user_serialization_skips_password_hash() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };

    let json = serde_json::to_string(&user).expect("Serialization failed");

    // Password hash should not be in the JSON
    assert!(!json.contains("password_hash"));
    assert!(!json.contains("hashed_password"));

    // Other fields should be present
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
}

#[test]
fn test_invalid_token_fails_validation() {
    let result = validate_token("invalid.token.here");
    assert!(result.is_err());
}
