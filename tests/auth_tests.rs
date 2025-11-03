//! Integration tests for authentication functionality
//!
//! Tests JWT token creation/validation and password hashing/verification.

mod common;

use cto_parallel_test::auth::jwt::{create_token, validate_token};
use cto_parallel_test::auth::models::User;

// JWT Token Tests

#[test]
fn test_jwt_token_creation() {
    let user_id = "user_123";
    let token_result = create_token(user_id);

    assert!(token_result.is_ok());
    let token = token_result.unwrap();
    assert!(!token.is_empty());
    assert!(token.contains('.'), "JWT should have dot separators");
}

#[test]
fn test_jwt_token_validation() {
    let user_id = "test_user_456";
    let token = create_token(user_id).expect("Failed to create token");

    let claims_result = validate_token(&token);
    assert!(claims_result.is_ok());

    let claims = claims_result.unwrap();
    assert_eq!(claims.sub, user_id);
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
    assert!(
        claims.exp > claims.iat,
        "Expiration should be after issued time"
    );
}

#[test]
fn test_jwt_token_expiration_time() {
    let token = create_token("user").expect("Failed to create token");
    let claims = validate_token(&token).expect("Failed to validate token");

    // Token should expire approximately 24 hours from now (86400 seconds)
    let time_until_expiration = claims.exp - claims.iat;

    // Allow 10 second tolerance (24 hours = 86400 seconds)
    assert!(
        (86390..=86410).contains(&time_until_expiration),
        "Token expiration should be approximately 24 hours"
    );
}

#[test]
fn test_jwt_invalid_token() {
    let invalid_tokens = vec![
        "invalid_token",
        "not.a.jwt",
        "",
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature",
    ];

    for token in invalid_tokens {
        let result = validate_token(token);
        assert!(result.is_err(), "Expected error for invalid token: {token}");
    }
}

#[test]
fn test_jwt_different_users_different_tokens() {
    let token1 = create_token("user1").unwrap();
    let token2 = create_token("user2").unwrap();

    assert_ne!(
        token1, token2,
        "Different users should have different tokens"
    );

    let claims1 = validate_token(&token1).unwrap();
    let claims2 = validate_token(&token2).unwrap();

    assert_eq!(claims1.sub, "user1");
    assert_eq!(claims2.sub, "user2");
}

#[test]
fn test_jwt_with_special_characters() {
    let user_ids = vec![
        "user@example.com",
        "user-123",
        "user_name",
        "user 123",
        "user@domain.com",
    ];

    for user_id in user_ids {
        let token = create_token(user_id).expect("Should create token");
        let claims = validate_token(&token).expect("Should validate token");
        assert_eq!(claims.sub, user_id);
    }
}

// Password Hashing Tests

#[test]
fn test_password_hashing() {
    let password = "secure_password_123";
    let hash = User::hash_password(password);

    assert!(!hash.is_empty());
    assert!(hash.starts_with("$argon2"), "Should be Argon2 hash");
    assert_ne!(hash, password, "Hash should not equal plaintext password");
}

#[test]
fn test_password_hashing_same_password_different_hashes() {
    let password = "same_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    assert_ne!(
        hash1, hash2,
        "Same password should produce different hashes due to random salt"
    );
}

#[test]
fn test_password_verification_correct() {
    let password = "correct_password";
    let user = common::create_test_user(1, "testuser", password);

    assert!(
        user.verify_password(password),
        "Should verify correct password"
    );
}

#[test]
fn test_password_verification_incorrect() {
    let password = "correct_password";
    let user = common::create_test_user(1, "testuser", password);

    assert!(
        !user.verify_password("wrong_password"),
        "Should not verify incorrect password"
    );
    assert!(
        !user.verify_password(""),
        "Should not verify empty password"
    );
    assert!(
        !user.verify_password("correct"),
        "Should not verify partial password"
    );
}

#[test]
fn test_password_verification_empty_password() {
    let empty_password = "";
    let user = common::create_test_user(1, "testuser", empty_password);

    assert!(user.verify_password(empty_password));
    assert!(!user.verify_password("not_empty"));
}

#[test]
fn test_password_verification_special_characters() {
    let passwords = vec![
        "p@ssw0rd!",
        "password with spaces",
        "пароль",     // Cyrillic
        "密码",       // Chinese
        "パスワード", // Japanese
        "emoji🔐password",
        "tab\ttab",
        "newline\nnewline",
    ];

    for password in passwords {
        let user = common::create_test_user(1, "testuser", password);
        assert!(
            user.verify_password(password),
            "Failed to verify password: {password}"
        );
        assert!(
            !user.verify_password("wrong"),
            "Should not verify wrong password for: {password}"
        );
    }
}

#[test]
fn test_password_verification_long_password() {
    let long_password = "a".repeat(1000);
    let user = common::create_test_user(1, "testuser", &long_password);

    assert!(user.verify_password(&long_password));
    assert!(!user.verify_password("a"));
}

#[test]
fn test_password_verification_case_sensitive() {
    let password = "Password123";
    let user = common::create_test_user(1, "testuser", password);

    assert!(user.verify_password("Password123"));
    assert!(!user.verify_password("password123"));
    assert!(!user.verify_password("PASSWORD123"));
}

#[test]
fn test_password_hash_invalid_format() {
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "invalid_hash_format".to_string(),
    };

    // Should return false, not panic
    assert!(!user.verify_password("any_password"));
}

// Integration Tests: Full Auth Flow

#[test]
fn test_complete_authentication_flow() {
    // Step 1: User registers with password
    let password = "user_secure_password";
    let user = common::create_test_user(1, "john_doe", password);

    // Step 2: User logs in - password is verified
    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));

    // Step 3: JWT token is created for authenticated user
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // Step 4: Token is validated on subsequent requests
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, user.id.to_string());
}

#[test]
fn test_authentication_flow_multiple_users() {
    let user1 = common::create_test_user(1, "alice", "alice_password");
    let user2 = common::create_test_user(2, "bob", "bob_password");

    // Each user has their own password
    assert!(user1.verify_password("alice_password"));
    assert!(user2.verify_password("bob_password"));

    // Each user gets their own token
    let token1 = create_token(&user1.id.to_string()).unwrap();
    let token2 = create_token(&user2.id.to_string()).unwrap();

    assert_ne!(token1, token2);

    // Tokens contain correct user IDs
    let claims1 = validate_token(&token1).unwrap();
    let claims2 = validate_token(&token2).unwrap();

    assert_eq!(claims1.sub, "1");
    assert_eq!(claims2.sub, "2");
}

#[test]
fn test_authentication_security_no_password_leaks() {
    let password = "secret_password";
    let user = common::create_test_user(1, "testuser", password);

    // Serialize user to JSON (as would happen in API response)
    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Password hash should NOT be in JSON
    assert!(!json.contains("password_hash"));
    assert!(!json.contains(&user.password_hash));
    assert!(!json.contains(password));

    // User data should be in JSON
    assert!(json.contains("testuser"));
    assert!(json.contains("testuser@example.com"));
}
