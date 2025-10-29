use cto_parallel_test::auth::{create_token, validate_token, User};

/// Test JWT token creation and validation
/// Verifies that a token can be created for a user ID and then validated successfully
#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
}

/// Test that password hashing produces different results for the same password
/// This verifies that the salt is working correctly
#[test]
fn test_password_hashing_produces_different_results() {
    let password = "secure_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Same password should produce different hashes due to random salt
    assert_ne!(
        hash1, hash2,
        "Password hashes should be different due to unique salts"
    );
}

/// Test password hashing and verification with correct password
/// Verifies that a hashed password can be verified successfully
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

    // Correct password should verify successfully
    assert!(
        user.verify_password(password),
        "Correct password should verify"
    );
}

/// Test that incorrect password verification fails
/// Verifies that the password verification correctly rejects wrong passwords
#[test]
fn test_incorrect_password_verification_fails() {
    let password = "secure_password";
    let hashed = User::hash_password(password);

    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };

    // Incorrect password should fail verification
    assert!(
        !user.verify_password("wrong_password"),
        "Incorrect password should fail"
    );
}

/// Test that User serialization skips `password_hash` field
/// Verifies that the password hash is not included in JSON serialization
#[test]
fn test_user_serialization_skips_password_hash() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hash123".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();

    // Password hash should not be in the serialized JSON
    assert!(
        !json.contains("password_hash"),
        "password_hash should be skipped in serialization"
    );
    assert!(
        json.contains("username"),
        "username should be in serialization"
    );
    assert!(json.contains("email"), "email should be in serialization");
}

/// Test JWT token validation with invalid token
/// Verifies that invalid tokens are properly rejected
#[test]
fn test_invalid_token_fails_validation() {
    let result = validate_token("invalid.token.here");
    assert!(result.is_err(), "Invalid token should fail validation");
}

/// Test JWT token expiration time is set correctly
/// Verifies that the token expiration is approximately 24 hours from creation
#[test]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn test_jwt_expiration_is_24_hours() {
    let user_id = "test_user";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();

    #[allow(clippy::disallowed_methods)]
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    // Check that expiration is approximately 24 hours from now
    let expected_exp = current_time + 86400;
    assert!(
        (claims.exp as i64 - expected_exp as i64).abs() < 5,
        "Token expiration should be 24 hours from creation (within 5 seconds tolerance)"
    );
}
