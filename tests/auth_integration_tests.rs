use ecommerce_catalog::auth::{create_token, validate_token, User};

#[test]
fn test_complete_auth_flow() {
    // Step 1: Hash a password
    let password = "user_secure_password_123";
    let hash = User::hash_password(password);

    // Step 2: Create a user with the hash
    let user = User {
        id: 42,
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        password_hash: hash,
    };

    // Step 3: Verify the password works
    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));

    // Step 4: Create a JWT token for the user
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // Step 5: Validate the token
    let claims = validate_token(&token).expect("Failed to validate token");

    // Step 6: Verify claims match the user
    assert_eq!(claims.sub, user.id.to_string());

    // Step 7: Ensure token has proper expiration
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_multiple_users_dont_interfere() {
    // Create two users with different passwords
    let user1_password = "user1_password";
    let user2_password = "user2_password";

    let user1 = User {
        id: 1,
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        password_hash: User::hash_password(user1_password),
    };

    let user2 = User {
        id: 2,
        username: "user2".to_string(),
        email: "user2@example.com".to_string(),
        password_hash: User::hash_password(user2_password),
    };

    // Each user should only verify their own password
    assert!(user1.verify_password(user1_password));
    assert!(!user1.verify_password(user2_password));

    assert!(user2.verify_password(user2_password));
    assert!(!user2.verify_password(user1_password));

    // Create tokens for both users
    let token1 = create_token(&user1.id.to_string()).expect("Failed to create token1");
    let token2 = create_token(&user2.id.to_string()).expect("Failed to create token2");

    // Tokens should be different
    assert_ne!(token1, token2);

    // Each token should validate to the correct user
    let claims1 = validate_token(&token1).expect("Failed to validate token1");
    let claims2 = validate_token(&token2).expect("Failed to validate token2");

    assert_eq!(claims1.sub, "1");
    assert_eq!(claims2.sub, "2");
}

#[test]
fn test_auth_flow_with_serialization() {
    // Create a user
    let user = User {
        id: 99,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        password_hash: User::hash_password("alice_password"),
    };

    // Serialize the user to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize user");

    // Verify password hash is not in the JSON
    assert!(!json.contains("password_hash"));

    // Verify other fields are present
    assert!(json.contains("alice"));
    assert!(json.contains("alice@example.com"));
    assert!(json.contains("99"));

    // Create a token
    let token = create_token(&user.id.to_string()).expect("Failed to create token");

    // Validate it
    let claims = validate_token(&token).expect("Failed to validate token");
    assert_eq!(claims.sub, "99");
}
