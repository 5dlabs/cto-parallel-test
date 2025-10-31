use cto_parallel_test::auth::models::User;
use cto_parallel_test::auth::{create_token, validate_token};

#[test]
fn test_jwt_creation_and_validation() {
    // Create a token for user ID "123"
    let user_id = "123";
    let token = create_token(user_id).expect("Failed to create token");

    // Token should not be empty
    assert!(!token.is_empty());

    // Validate the token
    let claims = validate_token(&token).expect("Failed to validate token");

    // Verify user ID is correctly extracted
    assert_eq!(claims.sub, user_id);
}

#[test]
fn test_invalid_token_validation() {
    // Invalid token should fail validation
    let result = validate_token("invalid.token.string");
    assert!(result.is_err());
}

#[test]
fn test_password_hashing_produces_different_results() {
    let password = "TEST_PASSWORD_123";

    // Hash the same password twice
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Hashes should be different due to salt
    assert_ne!(hash1, hash2);

    // But both should be valid
    let user1 = User {
        id: 1,
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        password_hash: hash1,
    };

    let user2 = User {
        id: 2,
        username: "user2".to_string(),
        email: "user2@example.com".to_string(),
        password_hash: hash2,
    };

    assert!(user1.verify_password(password));
    assert!(user2.verify_password(password));
}

#[test]
fn test_password_hashing_and_verification() {
    let password = "TEST_SECURE_PASSWORD";
    let hashed = User::hash_password(password);

    // Hash should not equal plain password
    assert_ne!(hashed, password);

    // Create user with hashed password
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };

    // Test correct password verification
    assert!(user.verify_password(password));

    // Test incorrect password verification fails
    assert!(!user.verify_password("WRONG_PASSWORD"));
    assert!(!user.verify_password(""));
    assert!(!user.verify_password("TEST_SECURE_PASSWORD1"));
}

#[test]
fn test_user_serialization_skips_password_hash() {
    let user = User::new(
        1,
        "testuser".to_string(),
        "test@example.com".to_string(),
        "TEST_PASSWORD123",
    );

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Password hash should not be present in JSON
    assert!(!json.contains("password_hash"));
    assert!(!json.contains("password"));

    // Other fields should be present
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
    assert!(json.contains("\"id\":1"));
}

#[test]
fn test_user_new_constructor() {
    let user = User::new(
        42,
        "alice".to_string(),
        "alice@example.com".to_string(),
        "TEST_MY_SECRET_PASSWORD",
    );

    assert_eq!(user.id, 42);
    assert_eq!(user.username, "alice");
    assert_eq!(user.email, "alice@example.com");

    // Password should be hashed and verifiable
    assert!(user.verify_password("TEST_MY_SECRET_PASSWORD"));
    assert!(!user.verify_password("WRONG_PASSWORD"));
}

#[test]
fn test_multiple_users_different_hashes() {
    let password = "TEST_SHARED_PASSWORD";

    let user1 = User::new(
        1,
        "user1".to_string(),
        "user1@example.com".to_string(),
        password,
    );
    let user2 = User::new(
        2,
        "user2".to_string(),
        "user2@example.com".to_string(),
        password,
    );

    // Different users with same password should have different hashes
    assert_ne!(user1.password_hash, user2.password_hash);

    // But both should verify correctly
    assert!(user1.verify_password(password));
    assert!(user2.verify_password(password));
}
