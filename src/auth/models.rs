//! User authentication models and password hashing
//!
//! This module provides user data structures and secure password handling using Argon2.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Argon2 hashed password (never included in JSON serialization)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the password matches, `false` otherwise
    ///
    /// # Security
    ///
    /// This method uses constant-time comparison through Argon2's verify function
    /// to prevent timing attacks. Returns `false` on any error instead of panicking.
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("my_password");
    /// let user = User {
    ///     id: 1,
    ///     username: "john".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("my_password"));
    /// assert!(!user.verify_password("wrong_password"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Parse the password hash from the stored string
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        // Verify the password using Argon2
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hashes a password using Argon2 with a random salt
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// * `String` - The Argon2 encoded hash string
    ///
    /// # Security
    ///
    /// - Uses Argon2 (winner of Password Hashing Competition)
    /// - Generates a unique random salt for each password
    /// - Uses default Argon2 configuration (memory-hard)
    /// - Intentionally slow to resist brute force attacks (~100ms)
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (rare, usually indicates system issues)
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password");
    /// let hash2 = User::hash_password("password");
    ///
    /// // Different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}

/// Request body for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password
    pub password: String,
}

/// Request body for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password to be hashed
    pub password: String,
}

/// Response body for successful authentication
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for subsequent requests
    pub token: String,
    /// User ID
    pub user_id: i32,
    /// Username
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_produces_different_hashes() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verification_success() {
        let password = "test_password_123";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_password_verification_failure() {
        let password = "test_password_123";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_password_hash_is_not_empty() {
        let hash = User::hash_password("password");
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_password_hash_format() {
        let hash = User::hash_password("password");
        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_empty_password_handled() {
        let password = "";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_very_long_password() {
        let password = "a".repeat(1000);
        let hash = User::hash_password(&password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&password));
        assert!(!user.verify_password("wrong"));
    }

    #[test]
    fn test_special_characters_in_password() {
        let password = "P@ssw0rd!#$%^&*()";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_unicode_password() {
        let password = "пароль密码🔐";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_whitespace_in_password_preserved() {
        let password = "pass word with spaces";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("passwordwithspaces"));
    }

    #[test]
    fn test_invalid_hash_format_returns_false() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "test_hash_should_not_appear_in_json".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Ensure password_hash is not in the JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("test_hash_should_not_appear_in_json"));

        // Ensure other fields are present
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"username\":\"test\""));
        assert!(json.contains("\"email\":\"test@example.com\""));
    }

    #[test]
    fn test_multiple_passwords_have_unique_hashes() {
        let passwords = vec!["password1", "password2", "password3", "password1"];
        let mut hashes = Vec::new();

        for password in &passwords {
            hashes.push(User::hash_password(password));
        }

        // Even the same password should produce different hashes
        assert_ne!(hashes[0], hashes[3]);

        // All hashes should be unique
        for i in 0..hashes.len() {
            for j in (i + 1)..hashes.len() {
                assert_ne!(hashes[i], hashes[j]);
            }
        }
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"john","password":"test_pass_123"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize LoginRequest");

        assert_eq!(request.username, "john");
        assert_eq!(request.password, "test_pass_123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"john","email":"john@example.com","password":"test_pass_123"}"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(request.username, "john");
        assert_eq!(request.email, "john@example.com");
        assert_eq!(request.password, "test_pass_123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "john".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("\"token\":\"jwt_token_here\""));
        assert!(json.contains("\"user_id\":42"));
        assert!(json.contains("\"username\":\"john\""));
    }
}
