//! Authentication integration tests
//!
//! This module tests JWT token creation, validation, and password hashing/verification.

mod common;

use common::auth::{
    create_test_token, hash_test_password, validate_test_token, verify_test_password,
};

// ============================================================================
// JWT Token Tests
// ============================================================================

#[test]
fn test_create_jwt_token() {
    let token = create_test_token(1);
    assert!(!token.is_empty());
    assert!(token.contains("mock_token_user_1"));
}

#[test]
fn test_create_token_for_different_users() {
    let token1 = create_test_token(1);
    let token2 = create_test_token(2);

    assert_ne!(token1, token2);
    assert!(token1.contains("user_1"));
    assert!(token2.contains("user_2"));
}

#[test]
fn test_validate_jwt_token_success() {
    let token = create_test_token(42);
    let result = validate_test_token(&token);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_validate_jwt_token_extracts_correct_user_id() {
    let user_id = 12345;
    let token = create_test_token(user_id);
    let extracted_id = validate_test_token(&token);

    assert!(extracted_id.is_ok());
    assert_eq!(extracted_id.unwrap(), user_id);
}

#[test]
fn test_validate_invalid_token() {
    let invalid_tokens = vec![
        "invalid_token",
        "bearer_token_xyz",
        "",
        "mock_token_user_abc_exp_123", // Non-numeric user ID
    ];

    for token in invalid_tokens {
        let result = validate_test_token(token);
        assert!(result.is_err(), "Token '{token}' should be invalid");
    }
}

#[test]
fn test_validate_malformed_token() {
    let result = validate_test_token("completely_random_string");
    assert!(result.is_err());
}

#[test]
fn test_validate_empty_token() {
    let result = validate_test_token("");
    assert!(result.is_err());
}

#[test]
fn test_token_contains_expiration() {
    let token = create_test_token(1);
    assert!(token.contains("exp_"), "Token should contain expiration");
}

// ============================================================================
// Password Hashing Tests
// ============================================================================

#[test]
fn test_hash_password() {
    let password = "my_secure_password";
    let hash = hash_test_password(password);

    assert!(!hash.is_empty());
    assert_ne!(hash, password);
    assert!(hash.starts_with("hashed_"));
}

#[test]
fn test_hash_password_different_inputs() {
    let hash1 = hash_test_password("password1");
    let hash2 = hash_test_password("password2");

    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_empty_password() {
    let hash = hash_test_password("");
    assert_eq!(hash, "hashed_");
}

#[test]
fn test_hash_long_password() {
    let long_password = "a".repeat(1000);
    let hash = hash_test_password(&long_password);
    assert!(!hash.is_empty());
}

#[test]
fn test_hash_special_characters() {
    let password = "p@ssw0rd!#$%^&*()";
    let hash = hash_test_password(password);
    assert!(hash.contains("p@ssw0rd!#$%^&*()"));
}

// ============================================================================
// Password Verification Tests
// ============================================================================

#[test]
fn test_verify_password_success() {
    let password = "correct_password";
    let hash = hash_test_password(password);

    assert!(verify_test_password(password, &hash));
}

#[test]
fn test_verify_password_failure() {
    let password = "correct_password";
    let hash = hash_test_password(password);

    assert!(!verify_test_password("wrong_password", &hash));
}

#[test]
fn test_verify_password_case_sensitive() {
    let password = "MyPassword";
    let hash = hash_test_password(password);

    assert!(verify_test_password("MyPassword", &hash));
    assert!(!verify_test_password("mypassword", &hash));
    assert!(!verify_test_password("MYPASSWORD", &hash));
}

#[test]
fn test_verify_password_empty() {
    let hash = hash_test_password("");
    assert!(verify_test_password("", &hash));
    assert!(!verify_test_password("something", &hash));
}

#[test]
fn test_verify_with_wrong_hash() {
    let password = "password";
    let hash = hash_test_password(password);
    let wrong_hash = "not_a_real_hash";

    assert!(!verify_test_password(password, wrong_hash));
    assert!(verify_test_password(password, &hash));
}

// ============================================================================
// Integration Tests - Auth Flow
// ============================================================================

#[test]
fn test_complete_auth_flow() {
    // 1. User registers with password
    let user_id = 1;
    let password = "secure_password_123";
    let password_hash = hash_test_password(password);

    // 2. Verify password is hashed
    assert_ne!(password, password_hash);

    // 3. User logs in with correct password
    assert!(verify_test_password(password, &password_hash));

    // 4. Create JWT token after successful login
    let token = create_test_token(user_id);
    assert!(!token.is_empty());

    // 5. Validate token for protected routes
    let extracted_user_id = validate_test_token(&token);
    assert!(extracted_user_id.is_ok());
    assert_eq!(extracted_user_id.unwrap(), user_id);
}

#[test]
fn test_failed_login_flow() {
    // In a real implementation, user_id would be used after successful login
    let password = "correct_password";
    let password_hash = hash_test_password(password);

    // Attempt login with wrong password
    assert!(!verify_test_password("wrong_password", &password_hash));

    // Token should not be created
    // (In real implementation, we wouldn't create token here)
}

#[test]
fn test_multiple_user_tokens() {
    let user1_token = create_test_token(1);
    let user2_token = create_test_token(2);
    let user3_token = create_test_token(3);

    // Validate each token returns correct user ID
    assert_eq!(validate_test_token(&user1_token).unwrap(), 1);
    assert_eq!(validate_test_token(&user2_token).unwrap(), 2);
    assert_eq!(validate_test_token(&user3_token).unwrap(), 3);

    // Tokens are different
    assert_ne!(user1_token, user2_token);
    assert_ne!(user2_token, user3_token);
}

#[test]
fn test_token_invalidation() {
    let valid_token = create_test_token(1);
    let result = validate_test_token(&valid_token);
    assert!(result.is_ok());

    // In a real implementation, we'd test token expiration here
    // For now, we test that an invalid token is rejected
    let invalid_result = validate_test_token("expired_or_invalid_token");
    assert!(invalid_result.is_err());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_validate_token_error_messages() {
    let result = validate_test_token("invalid");
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(!error.is_empty());
}

#[test]
fn test_password_verification_with_null_bytes() {
    let password = "password\0with\0nulls";
    let hash = hash_test_password(password);
    assert!(verify_test_password(password, &hash));
}

#[test]
fn test_unicode_password() {
    let password = "пароль密码🔐";
    let hash = hash_test_password(password);
    assert!(verify_test_password(password, &hash));
    assert!(!verify_test_password("wrong", &hash));
}
