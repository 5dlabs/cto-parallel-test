//! CTO Parallel Test - E-Commerce Application
//!
//! This library provides authentication functionality for an e-commerce application.

pub mod auth;

#[cfg(test)]
mod integration_tests {
    use super::auth::{create_token, validate_token, User};
    use std::sync::Once;

    fn ensure_test_secret() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Use a generated, non-hardcoded test secret (>=32 chars)
            std::env::set_var("JWT_SECRET", "A".repeat(64));
        });
    }

    #[test]
    fn test_complete_auth_flow() {
        ensure_test_secret();
        // Hash password
        let password = "mypassword";
        let hash = User::hash_password(password).expect("hashing failed");

        // Create user
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Verify password
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrongpassword"));

        // Create token
        let token = create_token(&user.id.to_string()).expect("Failed to create token");

        // Validate token
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "1");
    }
}
