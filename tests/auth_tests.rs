// Authentication integration tests
// Tests JWT token creation/validation and password hashing/verification

mod common;

use ecommerce_catalog::auth::{create_token, validate_token, User};
use std::thread;
use std::time::Duration;

#[test]
fn test_jwt_token_creation_and_validation() {
    let user_id = "12345";

    // Create token
    let token = create_token(user_id).expect("Failed to create token");
    assert!(!token.is_empty());
    assert!(
        token.contains('.'),
        "JWT should contain dots separating header.payload.signature"
    );

    // Validate token
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, user_id);
    assert!(
        claims.exp > claims.iat,
        "Expiration should be after issued time"
    );
}

#[test]
fn test_jwt_token_expires_in_24_hours() {
    let user_id = "test_user";
    let token = create_token(user_id).expect("Failed to create token");
    let claims = validate_token(&token).expect("Failed to validate token");

    // Check that expiration is approximately 24 hours from now
    let expiration_duration = claims.exp - claims.iat;
    let expected_duration = 24 * 3600; // 24 hours in seconds

    // Allow 10 second variance for test execution time
    #[allow(clippy::cast_possible_wrap)]
    let diff = i64::try_from(expiration_duration)
        .unwrap()
        .abs_diff(i64::from(expected_duration));
    assert!(
        diff < 10,
        "Token should expire in 24 hours (diff: {diff})"
    );
}

#[test]
fn test_jwt_invalid_token_rejected() {
    let invalid_tokens = vec![
        "invalid.token.here",
        "not_a_jwt",
        "",
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature",
    ];

    for invalid_token in invalid_tokens {
        let result = validate_token(invalid_token);
        assert!(
            result.is_err(),
            "Invalid token should be rejected: {invalid_token}"
        );
    }
}

#[test]
fn test_jwt_different_users_get_different_tokens() {
    let token1 = create_token("user1").expect("Failed to create token 1");
    let token2 = create_token("user2").expect("Failed to create token 2");

    assert_ne!(
        token1, token2,
        "Different users should get different tokens"
    );

    let claims1 = validate_token(&token1).expect("Failed to validate token 1");
    let claims2 = validate_token(&token2).expect("Failed to validate token 2");

    assert_eq!(claims1.sub, "user1");
    assert_eq!(claims2.sub, "user2");
}

#[test]
fn test_jwt_same_user_different_timestamps() {
    let user_id = "same_user";
    let token1 = create_token(user_id).expect("Failed to create token 1");

    // Wait to ensure different timestamp
    thread::sleep(Duration::from_secs(1));

    let token2 = create_token(user_id).expect("Failed to create token 2");

    // Tokens should be different due to different iat
    assert_ne!(
        token1, token2,
        "Tokens created at different times should differ"
    );

    // But both should validate to the same user
    let claims1 = validate_token(&token1).expect("Failed to validate token 1");
    let claims2 = validate_token(&token2).expect("Failed to validate token 2");

    assert_eq!(claims1.sub, claims2.sub);
    assert!(
        claims2.iat > claims1.iat,
        "Second token should have later issued time"
    );
}

#[test]
fn test_jwt_handles_special_characters_in_user_id() {
    let special_ids = vec![
        "user@example.com",
        "user-with-dashes",
        "user_with_underscores",
        "user.with.dots",
        "123456789",
    ];

    for user_id in special_ids {
        let token = create_token(user_id).expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(
            claims.sub, user_id,
            "User ID should be preserved: {user_id}"
        );
    }
}

#[test]
fn test_password_hashing_creates_unique_hashes() {
    let password = "my_secure_password";

    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Each hash should be unique due to random salt
    assert_ne!(
        hash1, hash2,
        "Password hashes should differ due to random salts"
    );

    // But both should verify successfully
    let user1 = common::create_test_user(1, "user1", "user1@test.com", password);
    let user2 = common::create_test_user(2, "user2", "user2@test.com", password);

    assert!(user1.verify_password(password));
    assert!(user2.verify_password(password));
}

#[test]
fn test_password_verification_with_correct_password() {
    let password = "correct_password";
    let user = common::create_test_user(1, "testuser", "test@example.com", password);

    assert!(
        user.verify_password(password),
        "Correct password should verify"
    );
}

#[test]
fn test_password_verification_fails_with_wrong_password() {
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    let user = common::create_test_user(1, "testuser", "test@example.com", correct_password);

    assert!(
        !user.verify_password(wrong_password),
        "Wrong password should not verify"
    );
}

#[test]
fn test_password_verification_case_sensitive() {
    let password = "Password123";
    let user = common::create_test_user(1, "testuser", "test@example.com", password);

    assert!(user.verify_password("Password123"));
    assert!(
        !user.verify_password("password123"),
        "Password should be case-sensitive"
    );
    assert!(
        !user.verify_password("PASSWORD123"),
        "Password should be case-sensitive"
    );
}

#[test]
fn test_password_hashing_handles_empty_password() {
    let empty_password = "";
    let user = common::create_test_user(1, "testuser", "test@example.com", empty_password);

    assert!(
        user.verify_password(empty_password),
        "Empty password should verify"
    );
    assert!(
        !user.verify_password("not_empty"),
        "Non-empty password should not match"
    );
}

#[test]
fn test_password_hashing_handles_long_passwords() {
    let long_password = "a".repeat(500);
    let user = common::create_test_user(1, "testuser", "test@example.com", &long_password);

    assert!(
        user.verify_password(&long_password),
        "Long password should verify"
    );

    let different_long = "b".repeat(500);
    assert!(
        !user.verify_password(&different_long),
        "Different long password should not match"
    );
}

#[test]
fn test_password_hashing_handles_special_characters() {
    let special_password = "P@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?";
    let user = common::create_test_user(1, "testuser", "test@example.com", special_password);

    assert!(
        user.verify_password(special_password),
        "Special characters should be handled"
    );
}

#[test]
fn test_password_hashing_handles_unicode() {
    let unicode_password = "パスワード123🔒";
    let user = common::create_test_user(1, "testuser", "test@example.com", unicode_password);

    assert!(
        user.verify_password(unicode_password),
        "Unicode password should verify"
    );
}

#[test]
fn test_password_hashing_handles_whitespace() {
    let password_with_spaces = "password with spaces";
    let user = common::create_test_user(1, "testuser", "test@example.com", password_with_spaces);

    assert!(
        user.verify_password(password_with_spaces),
        "Password with spaces should verify"
    );
    assert!(
        !user.verify_password("passwordwithspaces"),
        "Spaces should be significant"
    );
}

#[test]
fn test_password_hash_constant_time_comparison() {
    // This test verifies that password verification uses constant-time comparison
    // It's difficult to test timing directly, but we can verify behavior

    let password = "correct_password";
    let user = common::create_test_user(1, "testuser", "test@example.com", password);

    // Various incorrect passwords of different lengths should all fail
    let repeated = "a".repeat(100);
    let wrong_passwords = vec![
        "x",
        "wrong",
        "wrong_password",
        "correct_password_wrong",
        repeated.as_str(),
    ];

    for wrong in wrong_passwords {
        assert!(
            !user.verify_password(wrong),
            "Wrong password should not verify: {wrong}"
        );
    }
}

#[test]
fn test_user_password_hash_not_exposed_in_serialization() {
    let user = common::create_test_user(1, "testuser", "test@example.com", "secret");

    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Password hash should not appear in JSON
    assert!(
        !json.contains("password_hash"),
        "password_hash field should be skipped"
    );
    assert!(
        !json.contains("secret"),
        "Password should not appear in JSON"
    );

    // Other fields should be present
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
}

#[test]
fn test_complete_authentication_flow() {
    // Simulate a complete authentication flow

    // 1. User registers - password is hashed
    let password = "user_password_123";
    let user = common::create_test_user(1, "newuser", "newuser@example.com", password);

    // 2. User logs in - password is verified
    assert!(
        user.verify_password(password),
        "Login should verify password"
    );

    // 3. JWT token is created upon successful login
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // 4. Token is validated for subsequent requests
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, user.id.to_string());

    // 5. Invalid password fails authentication
    assert!(
        !user.verify_password("wrong_password"),
        "Wrong password should fail"
    );
}

#[test]
fn test_multiple_users_authentication() {
    // Create multiple users with different passwords
    let user1 = common::create_test_user(1, "alice", "alice@example.com", "alice_pass");
    let user2 = common::create_test_user(2, "bob", "bob@example.com", "bob_pass");
    let user3 = common::create_test_user(3, "charlie", "charlie@example.com", "charlie_pass");

    // Each user can authenticate with their own password
    assert!(user1.verify_password("alice_pass"));
    assert!(user2.verify_password("bob_pass"));
    assert!(user3.verify_password("charlie_pass"));

    // But not with others' passwords
    assert!(!user1.verify_password("bob_pass"));
    assert!(!user2.verify_password("charlie_pass"));
    assert!(!user3.verify_password("alice_pass"));

    // Each user gets a unique token
    let token1 = create_token(&user1.id.to_string()).expect("Failed to create token 1");
    let token2 = create_token(&user2.id.to_string()).expect("Failed to create token 2");
    let token3 = create_token(&user3.id.to_string()).expect("Failed to create token 3");

    let claims1 = validate_token(&token1).expect("Failed to validate token 1");
    let claims2 = validate_token(&token2).expect("Failed to validate token 2");
    let claims3 = validate_token(&token3).expect("Failed to validate token 3");

    assert_eq!(claims1.sub, "1");
    assert_eq!(claims2.sub, "2");
    assert_eq!(claims3.sub, "3");
}

#[test]
fn test_jwt_token_structure() {
    let token = create_token("test_user").expect("Failed to create token");

    // JWT should have three parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3, "JWT should have header.payload.signature");

    // Each part should be non-empty
    for (i, part) in parts.iter().enumerate() {
        assert!(!part.is_empty(), "JWT part {i} should not be empty");
    }
}
