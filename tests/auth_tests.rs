use cto_parallel_test::auth::{create_token, validate_token, User};

#[test]
fn test_jwt_creation_and_validation() {
    // Create a token for a test user
    let user_id = "123";
    let token = create_token(user_id).expect("Failed to create token");

    // Validate the token
    let claims = validate_token(&token).expect("Failed to validate token");

    // Verify the user ID is correct
    assert_eq!(claims.sub, user_id);

    // Verify timestamps are present
    assert!(claims.iat > 0);
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_invalid_token_fails_validation() {
    let result = validate_token("invalid_token_string");
    assert!(result.is_err(), "Invalid token should fail validation");
}

#[test]
fn test_password_hashing_produces_different_hashes() {
    let password = "secure_password";

    // Hash the same password twice
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Hashes should be different due to random salt
    assert_ne!(hash1, hash2, "Password hashes should be unique");
}

#[test]
fn test_password_verification_with_correct_password() {
    let password = "my_secure_password";
    let hashed = User::hash_password(password);

    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };

    // Correct password should verify
    assert!(
        user.verify_password(password),
        "Correct password should verify"
    );
}

#[test]
fn test_password_verification_with_incorrect_password() {
    let password = "my_secure_password";
    let hashed = User::hash_password(password);

    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };

    // Incorrect password should not verify
    assert!(
        !user.verify_password("wrong_password"),
        "Incorrect password should not verify"
    );
}

#[test]
fn test_user_serialization_excludes_password_hash() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: User::hash_password("password123"),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Password hash should not be in the JSON
    assert!(
        !json.contains("password_hash"),
        "Password hash should not be serialized"
    );

    // Username and email should be present
    assert!(json.contains("testuser"), "Username should be serialized");
    assert!(
        json.contains("test@example.com"),
        "Email should be serialized"
    );
}

#[test]
fn test_multiple_users_with_same_password_have_different_hashes() {
    let password = "shared_password";

    let user1 = User {
        id: 1,
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        password_hash: User::hash_password(password),
    };

    let user2 = User {
        id: 2,
        username: "user2".to_string(),
        email: "user2@example.com".to_string(),
        password_hash: User::hash_password(password),
    };

    // Even with the same password, hashes should be different
    assert_ne!(
        user1.password_hash, user2.password_hash,
        "Different users should have different hashes"
    );

    // But both should verify correctly
    assert!(
        user1.verify_password(password),
        "User 1 password should verify"
    );
    assert!(
        user2.verify_password(password),
        "User 2 password should verify"
    );
}
