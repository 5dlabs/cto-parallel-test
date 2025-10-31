//! Authentication and security tests
//!
//! This module tests JWT token creation/validation and password hashing/verification.
//! These tests are designed to work with the auth module once it's implemented.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String, // Subject (user ID)
    exp: usize,  // Expiration time
    iat: usize,  // Issued at
}

const JWT_SECRET: &str = "test_secret_key_for_testing_only_do_not_use_in_production";

/// Helper function to create a JWT token
fn create_test_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 86400, // 24 hours from now
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// Helper function to validate a JWT token
fn validate_test_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Helper function to hash a password
fn hash_test_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Helper function to verify a password
fn verify_test_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

#[test]
fn test_jwt_creation_and_validation() {
    // Test creating a valid JWT token
    let user_id = "test_user_123";
    let token_result = create_test_token(user_id);

    assert!(token_result.is_ok(), "JWT token creation should succeed");

    let token = token_result.unwrap();
    assert!(!token.is_empty(), "Token should not be empty");

    // Test validating the token
    let validation_result = validate_test_token(&token);
    assert!(validation_result.is_ok(), "Token validation should succeed");

    let claims = validation_result.unwrap();
    assert_eq!(claims.sub, user_id, "Token should contain correct user ID");
}

#[test]
fn test_jwt_validation_with_invalid_token() {
    // Test with completely invalid token
    let invalid_token = "this.is.not.a.valid.jwt.token";
    let result = validate_test_token(invalid_token);

    assert!(result.is_err(), "Invalid token should fail validation");
}

#[test]
fn test_jwt_validation_with_wrong_signature() {
    // Create a token with one secret
    let token = create_test_token("user123").expect("Failed to create token");

    // Try to validate with a different secret
    let wrong_secret = "different_secret_key";
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(wrong_secret.as_ref()),
        &Validation::default(),
    );

    assert!(
        result.is_err(),
        "Token with wrong signature should fail validation"
    );
}

#[test]
fn test_jwt_expired_token() {
    // Create a token that's already expired
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let expired_claims = Claims {
        sub: "user123".to_string(),
        exp: now - 3600, // Expired 1 hour ago
        iat: now - 7200, // Issued 2 hours ago
    };

    let token = encode(
        &Header::default(),
        &expired_claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .expect("Failed to create expired token");

    // Try to validate the expired token
    let result = validate_test_token(&token);

    assert!(result.is_err(), "Expired token should fail validation");
}

#[test]
fn test_jwt_claims_extraction() {
    let user_id = "user_456";
    let token = create_test_token(user_id).expect("Failed to create token");
    let claims = validate_test_token(&token).expect("Failed to validate token");

    // Verify all claims are present and correct
    assert_eq!(claims.sub, user_id, "User ID should match");
    assert!(claims.exp > 0, "Expiration should be set");
    assert!(claims.iat > 0, "Issued at should be set");
    assert!(
        claims.exp > claims.iat,
        "Expiration should be after issued at"
    );
}

#[test]
fn test_password_hashing_and_verification() {
    let password = "secure_password_123";

    // Hash the password
    let hash_result = hash_test_password(password);
    assert!(hash_result.is_ok(), "Password hashing should succeed");

    let hash = hash_result.unwrap();
    assert!(!hash.is_empty(), "Password hash should not be empty");

    // Verify the correct password
    let verification_result = verify_test_password(password, &hash);
    assert!(
        verification_result.is_ok(),
        "Password verification should not error"
    );
    assert!(
        verification_result.unwrap(),
        "Correct password should verify successfully"
    );
}

#[test]
fn test_password_verification_with_wrong_password() {
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    // Hash the correct password
    let hash = hash_test_password(correct_password).expect("Failed to hash password");

    // Try to verify with wrong password
    let verification_result = verify_test_password(wrong_password, &hash);
    assert!(verification_result.is_ok(), "Verification should not error");
    assert!(
        !verification_result.unwrap(),
        "Wrong password should fail verification"
    );
}

#[test]
fn test_password_hash_uniqueness() {
    let password = "same_password";

    // Hash the same password twice
    let hash1 = hash_test_password(password).expect("Failed to hash password");
    let hash2 = hash_test_password(password).expect("Failed to hash password");

    // Hashes should be different due to different salts
    assert_ne!(
        hash1, hash2,
        "Same password should produce different hashes"
    );

    // But both should verify correctly
    assert!(
        verify_test_password(password, &hash1).unwrap(),
        "First hash should verify"
    );
    assert!(
        verify_test_password(password, &hash2).unwrap(),
        "Second hash should verify"
    );
}

#[test]
fn test_password_hash_format() {
    let password = "test_password";
    let hash = hash_test_password(password).expect("Failed to hash password");

    // Argon2 hash should start with $argon2
    assert!(
        hash.starts_with("$argon2"),
        "Hash should be in Argon2 format"
    );

    // Hash should be parseable
    let parse_result = PasswordHash::new(&hash);
    assert!(parse_result.is_ok(), "Hash should be parseable");
}

#[test]
fn test_empty_password_hashing() {
    let empty_password = "";

    // Even empty passwords should be hashable
    let hash_result = hash_test_password(empty_password);
    assert!(hash_result.is_ok(), "Empty password should be hashable");

    let hash = hash_result.unwrap();

    // Verify empty password
    let verification_result = verify_test_password(empty_password, &hash);
    assert!(verification_result.is_ok() && verification_result.unwrap());
}

#[test]
fn test_long_password_hashing() {
    // Test with a very long password
    let long_password = "a".repeat(1000);

    let hash_result = hash_test_password(&long_password);
    assert!(hash_result.is_ok(), "Long password should be hashable");

    let hash = hash_result.unwrap();

    // Verify long password
    let verification_result = verify_test_password(&long_password, &hash);
    assert!(
        verification_result.is_ok() && verification_result.unwrap(),
        "Long password should verify"
    );
}

#[test]
fn test_special_characters_in_password() {
    let special_password = "p@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?";

    let hash_result = hash_test_password(special_password);
    assert!(
        hash_result.is_ok(),
        "Password with special chars should be hashable"
    );

    let hash = hash_result.unwrap();

    // Verify password with special characters
    let verification_result = verify_test_password(special_password, &hash);
    assert!(
        verification_result.is_ok() && verification_result.unwrap(),
        "Password with special chars should verify"
    );
}

#[test]
fn test_unicode_password_hashing() {
    let unicode_password = "密码🔒パスワード";

    let hash_result = hash_test_password(unicode_password);
    assert!(hash_result.is_ok(), "Unicode password should be hashable");

    let hash = hash_result.unwrap();

    // Verify unicode password
    let verification_result = verify_test_password(unicode_password, &hash);
    assert!(
        verification_result.is_ok() && verification_result.unwrap(),
        "Unicode password should verify"
    );
}

#[test]
fn test_jwt_multiple_users() {
    // Create tokens for multiple users
    let user_ids = vec!["user1", "user2", "user3"];
    let mut tokens = Vec::new();

    for user_id in &user_ids {
        let token = create_test_token(user_id).expect("Failed to create token");
        tokens.push(token);
    }

    // Verify each token contains the correct user ID
    for (i, token) in tokens.iter().enumerate() {
        let claims = validate_test_token(token).expect("Failed to validate token");
        assert_eq!(
            claims.sub, user_ids[i],
            "Token should contain correct user ID"
        );
    }
}

#[test]
fn test_concurrent_password_hashing() {
    // Test that password hashing is thread-safe
    use std::thread;

    let password = "concurrent_test_password";
    let mut handles = vec![];

    for _ in 0..5 {
        let pwd = password.to_string();
        let handle = thread::spawn(move || hash_test_password(&pwd));
        handles.push(handle);
    }

    // All threads should complete successfully
    for handle in handles {
        let result = handle.join();
        assert!(result.is_ok(), "Thread should complete");
        let hash_result = result.unwrap();
        assert!(hash_result.is_ok(), "Hashing should succeed in thread");
    }
}

#[test]
fn test_jwt_token_format() {
    let token = create_test_token("user123").expect("Failed to create token");

    // JWT tokens have three parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3, "JWT should have three parts");

    // Each part should be non-empty
    for part in parts {
        assert!(!part.is_empty(), "JWT parts should not be empty");
    }
}

#[test]
fn test_password_case_sensitivity() {
    let password_lower = "password";
    let password_upper = "PASSWORD";

    let hash = hash_test_password(password_lower).expect("Failed to hash password");

    // Verify with correct case
    let correct_result = verify_test_password(password_lower, &hash);
    assert!(correct_result.is_ok() && correct_result.unwrap());

    // Verify with wrong case
    let wrong_case_result = verify_test_password(password_upper, &hash);
    assert!(
        wrong_case_result.is_ok() && !wrong_case_result.unwrap(),
        "Password verification should be case-sensitive"
    );
}

// Integration test combining JWT and password verification
#[test]
fn test_user_authentication_flow() {
    // Step 1: Hash a password (simulating user registration)
    let username = "test_user";
    let password = "user_password_123";
    let password_hash = hash_test_password(password).expect("Failed to hash password");

    // Step 2: Verify password (simulating login)
    let login_password = "user_password_123";
    let is_valid =
        verify_test_password(login_password, &password_hash).expect("Failed to verify password");

    assert!(is_valid, "Password should be valid during login");

    // Step 3: Create JWT token after successful authentication
    let token = create_test_token(username).expect("Failed to create token");

    // Step 4: Validate token for subsequent requests
    let claims = validate_test_token(&token).expect("Failed to validate token");

    assert_eq!(
        claims.sub, username,
        "Token should contain correct username"
    );
}
