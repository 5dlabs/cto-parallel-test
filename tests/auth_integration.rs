use cto_parallel_test::auth::{create_token, validate_token, User};
use serial_test::serial;

#[test]
#[serial]
fn test_complete_auth_flow() {
    // Set up environment
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");

    // 1. Hash password
    let password = "MySecureP@ssw0rd123";
    let hash = User::hash_password(password);

    // 2. Create user with hashed password
    let user = User {
        id: 42,
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        password_hash: hash,
    };

    // 3. Verify password
    assert!(
        user.verify_password(password),
        "Correct password should verify"
    );
    assert!(
        !user.verify_password("wrong_password"),
        "Wrong password should not verify"
    );

    // 4. Create JWT token
    let token = create_token(&user.id.to_string()).expect("create token");

    // 5. Validate JWT token
    let claims = validate_token(&token).expect("validate token");
    assert_eq!(claims.sub, "42");
    assert!(claims.exp > claims.iat, "Expiration should be after issue");

    // 6. Verify token TTL is ~24 hours
    let ttl = claims.exp - claims.iat;
    assert_eq!(
        ttl, 86_400,
        "TTL should be exactly 24 hours (86400 seconds), got {ttl}"
    );
}

#[test]
#[serial]
fn test_password_hash_uniqueness() {
    let password = "same_password";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);
    let hash3 = User::hash_password(password);

    // All hashes should be different due to random salt
    assert_ne!(hash1, hash2);
    assert_ne!(hash2, hash3);
    assert_ne!(hash1, hash3);

    // But all should verify correctly
    let user1 = User {
        id: 1,
        username: "user1".to_string(),
        email: "user1@test.com".to_string(),
        password_hash: hash1,
    };
    let user2 = User {
        id: 2,
        username: "user2".to_string(),
        email: "user2@test.com".to_string(),
        password_hash: hash2,
    };
    let user3 = User {
        id: 3,
        username: "user3".to_string(),
        email: "user3@test.com".to_string(),
        password_hash: hash3,
    };

    assert!(user1.verify_password(password));
    assert!(user2.verify_password(password));
    assert!(user3.verify_password(password));
}

#[test]
#[serial]
fn test_user_serialization_safety() {
    let hash = User::hash_password("secret_password");
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash.clone(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("serialize");

    // Verify password_hash is NOT in JSON
    assert!(
        !json.contains("password_hash"),
        "JSON should not contain 'password_hash' field"
    );
    assert!(
        !json.contains("$argon2"),
        "JSON should not contain Argon2 hash data"
    );
    assert!(
        !json.contains(&hash),
        "JSON should not contain the actual hash"
    );

    // Verify it does contain other fields
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
    assert!(json.contains("\"id\":1"));
}

#[test]
#[serial]
fn test_token_validation_edge_cases() {
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");

    // Empty token
    assert!(validate_token("").is_err());

    // Malformed token
    assert!(validate_token("not.a.jwt").is_err());

    // Token with wrong signature (altered payload)
    let valid_token = create_token("test_user").expect("create token");
    let mut parts: Vec<&str> = valid_token.split('.').collect();
    if parts.len() == 3 {
        // Modify the payload part to invalidate signature
        parts[1] = "invalid_payload_data";
        let tampered_token = parts.join(".");
        assert!(validate_token(&tampered_token).is_err());
    }

    // Valid token creation and validation
    let token = create_token("test_user").expect("create token");
    let claims = validate_token(&token).expect("validate token");
    assert_eq!(claims.sub, "test_user");
}

#[test]
#[serial]
fn test_password_edge_cases() {
    // Empty password
    let empty_hash = User::hash_password("");
    let user = User {
        id: 1,
        username: "test".to_string(),
        email: "test@test.com".to_string(),
        password_hash: empty_hash,
    };
    assert!(user.verify_password(""));
    assert!(!user.verify_password("not_empty"));

    // Very long password
    let long_password = "a".repeat(1000);
    let long_hash = User::hash_password(&long_password);
    let user2 = User {
        id: 2,
        username: "test2".to_string(),
        email: "test2@test.com".to_string(),
        password_hash: long_hash,
    };
    assert!(user2.verify_password(&long_password));
    assert!(!user2.verify_password("short"));

    // Special characters
    let special_password = "P@$$w0rd!#%^&*()[]{}|;':\"<>,.?/~`";
    let special_hash = User::hash_password(special_password);
    let user3 = User {
        id: 3,
        username: "test3".to_string(),
        email: "test3@test.com".to_string(),
        password_hash: special_hash,
    };
    assert!(user3.verify_password(special_password));
    assert!(!user3.verify_password("different"));

    // Unicode/emoji
    let unicode_password = "🔐🔑密码🚀";
    let unicode_hash = User::hash_password(unicode_password);
    let user4 = User {
        id: 4,
        username: "test4".to_string(),
        email: "test4@test.com".to_string(),
        password_hash: unicode_hash,
    };
    assert!(user4.verify_password(unicode_password));
    assert!(!user4.verify_password("ascii"));
}

#[test]
#[serial]
fn test_multiple_users_do_not_interfere() {
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");

    // Create multiple users with different passwords
    let user1 = User {
        id: 1,
        username: "user1".to_string(),
        email: "user1@test.com".to_string(),
        password_hash: User::hash_password("password1"),
    };

    let user2 = User {
        id: 2,
        username: "user2".to_string(),
        email: "user2@test.com".to_string(),
        password_hash: User::hash_password("password2"),
    };

    let user3 = User {
        id: 3,
        username: "user3".to_string(),
        email: "user3@test.com".to_string(),
        password_hash: User::hash_password("password3"),
    };

    // Verify each user's password
    assert!(user1.verify_password("password1"));
    assert!(!user1.verify_password("password2"));
    assert!(!user1.verify_password("password3"));

    assert!(user2.verify_password("password2"));
    assert!(!user2.verify_password("password1"));
    assert!(!user2.verify_password("password3"));

    assert!(user3.verify_password("password3"));
    assert!(!user3.verify_password("password1"));
    assert!(!user3.verify_password("password2"));

    // Create tokens for each user
    let token1 = create_token(&user1.id.to_string()).expect("token1");
    let token2 = create_token(&user2.id.to_string()).expect("token2");
    let token3 = create_token(&user3.id.to_string()).expect("token3");

    // Validate each token
    let claims1 = validate_token(&token1).expect("validate1");
    let claims2 = validate_token(&token2).expect("validate2");
    let claims3 = validate_token(&token3).expect("validate3");

    assert_eq!(claims1.sub, "1");
    assert_eq!(claims2.sub, "2");
    assert_eq!(claims3.sub, "3");
}
