//! Integration tests for the authentication module.
//!
//! These tests verify the complete authentication flow including:
//! - Password hashing and verification
//! - JWT token creation and validation
//! - End-to-end authentication workflows

#[cfg(test)]
mod integration_tests {
    use crate::auth::jwt::{create_token, validate_token};
    use crate::auth::models::User;

    #[test]
    fn test_complete_auth_flow() {
        // Step 1: Hash password for new user
        let password = "mySecurePassword123!";
        let hash = User::hash_password(password);

        // Step 2: Create user with hashed password
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Step 3: Verify password (simulates login)
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));

        // Step 4: Create JWT token after successful verification
        let token = create_token(&user.id.to_string()).expect("Failed to create token");

        // Step 5: Validate token (simulates protected route access)
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "1");

        // Step 6: Verify token claims are correct
        assert!(claims.exp > claims.iat);
        let expected_exp = claims.iat + 24 * 3600;
        assert_eq!(claims.exp, expected_exp);
    }

    #[test]
    fn test_multiple_users_dont_interfere() {
        // Create first user
        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: User::hash_password("password1"),
        };

        // Create second user
        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: User::hash_password("password2"),
        };

        // Each user verifies their own password
        assert!(user1.verify_password("password1"));
        assert!(user2.verify_password("password2"));

        // Cross-verification fails
        assert!(!user1.verify_password("password2"));
        assert!(!user2.verify_password("password1"));

        // Create tokens for both users
        let token1 = create_token(&user1.id.to_string()).expect("Failed to create token1");
        let token2 = create_token(&user2.id.to_string()).expect("Failed to create token2");

        // Tokens are different
        assert_ne!(token1, token2);

        // Each token validates to correct user ID
        let claims1 = validate_token(&token1).expect("Failed to validate token1");
        let claims2 = validate_token(&token2).expect("Failed to validate token2");

        assert_eq!(claims1.sub, "1");
        assert_eq!(claims2.sub, "2");
    }

    #[test]
    fn test_token_reuse_after_creation() {
        let user_id = "42";
        let token = create_token(user_id).expect("Failed to create token");

        // Validate multiple times - token should remain valid
        for _ in 0..5 {
            let claims = validate_token(&token).expect("Token should remain valid");
            assert_eq!(claims.sub, user_id);
        }
    }

    #[test]
    fn test_password_change_invalidates_old_hash() {
        let original_password = "old_password";
        let new_password = "new_password";

        // Create user with original password
        let mut user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password(original_password),
        };

        // Verify original password works
        assert!(user.verify_password(original_password));

        // Change password (hash new password)
        user.password_hash = User::hash_password(new_password);

        // Old password no longer works
        assert!(!user.verify_password(original_password));

        // New password works
        assert!(user.verify_password(new_password));
    }

    #[test]
    fn test_serialization_security() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password("secret"),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Password hash should not be present
        assert!(!json.contains("password_hash"));

        // User data should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_edge_case_empty_strings() {
        // Empty user ID in token
        let token = create_token("").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "");

        // Empty password (valid but not recommended)
        let hash = User::hash_password("");
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };
        assert!(user.verify_password(""));
    }

    #[test]
    fn test_error_handling_invalid_inputs() {
        // Invalid token format
        assert!(validate_token("not.a.valid.token").is_err());
        assert!(validate_token("").is_err());
        assert!(validate_token("random_string").is_err());

        // Invalid hash format
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash".to_string(),
        };
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    #[allow(clippy::cast_possible_truncation)]
    fn test_token_expiration_time() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let token = create_token("123").expect("Failed to create token");
        let claims = validate_token(&token).expect("Failed to validate token");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Token should expire in approximately 24 hours
        let time_until_expiry = claims.exp - now;

        // Should be close to 24 hours (86400 seconds), within 1 minute tolerance
        assert!(time_until_expiry > 86340 && time_until_expiry <= 86400);
    }
}
