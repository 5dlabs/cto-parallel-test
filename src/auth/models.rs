//! User authentication models
//!
//! Provides user data structures and secure password hashing using Argon2.
//! Password hashes are never serialized or exposed in API responses.

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

/// User entity with authentication credentials
///
/// Represents a user in the system with secure password storage.
/// The password hash is never included in JSON serialization for security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username (unique)
    pub username: String,
    /// Email address (unique)
    pub email: String,
    /// Argon2 password hash (excluded from JSON serialization)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// Uses constant-time comparison to prevent timing attacks.
    /// Returns false on any error (invalid hash format, verification failure, etc.)
    /// to avoid leaking information about the failure reason.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if password matches, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "secure_password";
    /// let hash = User::hash_password(password);
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "john_doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password(password));
    /// assert!(!user.verify_password("wrong_password"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Parse the stored hash
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        // Verify password against the hash
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hashes a password using Argon2 with a random salt
    ///
    /// Each call generates a unique random salt, ensuring that identical
    /// passwords produce different hashes. Uses Argon2id with default configuration
    /// which provides strong protection against brute-force attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// * `String` - Argon2 encoded hash string (includes algorithm, parameters, salt, and hash)
    ///
    /// # Panics
    ///
    /// Panics if hashing fails (extremely rare, typically memory allocation failure).
    ///
    /// # Security
    ///
    /// - Uses Argon2id algorithm (winner of Password Hashing Competition)
    /// - Random salt per password generated with cryptographically secure RNG
    /// - Default configuration optimized for security (OWASP recommended)
    /// - Intentionally slow to resist brute force attacks
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password");
    /// let hash2 = User::hash_password("password");
    ///
    /// // Same password produces different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}

/// Request payload for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Plaintext password
    pub password: String,
}

/// Request payload for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password
    pub password: String,
}

/// Response payload for authentication operations
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT authentication token
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
    fn test_hash_password_creates_valid_hash() {
        let password = "test_password_123";
        let hash = User::hash_password(password);

        // Argon2 hash should start with $argon2
        assert!(hash.starts_with("$argon2"));
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_same_password_different_hashes() {
        let password = "password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "correct_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "correct_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_password"));
        assert!(!user.verify_password(""));
        assert!(!user.verify_password("correct"));
    }

    #[test]
    fn test_verify_password_empty() {
        let password = "";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_verify_password_special_characters() {
        let passwords = vec![
            "p@ssw0rd!",
            "пароль",     // Cyrillic
            "密码",       // Chinese
            "パスワード", // Japanese
            "emoji🔐password",
            "with spaces",
            "with\ttabs\nand\nnewlines",
        ];

        for password in passwords {
            let hash = User::hash_password(password);
            let user = User {
                id: 1,
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password_hash: hash,
            };

            assert!(
                user.verify_password(password),
                "Failed to verify password: {password}"
            );
        }
    }

    #[test]
    fn test_verify_password_long() {
        let password = "a".repeat(1000);
        let hash = User::hash_password(&password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&password));
    }

    #[test]
    fn test_verify_with_invalid_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_user_serialization_excludes_password() {
        let password = "secret_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash.clone(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // JSON should NOT contain password_hash field or its value
        assert!(!json.contains("password_hash"));
        assert!(!json.contains(&hash));
        assert!(!json.contains("secret"));

        // JSON should contain other fields
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "john", "password": "secret"}"#;
        let request: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "john");
        assert_eq!(request.password, "secret");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username": "jane", "email": "jane@example.com", "password": "secret123"}"#;
        let request: RegisterRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "jane");
        assert_eq!(request.email, "jane@example.com");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "alice".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("\"user_id\":42"));
        assert!(json.contains("alice"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate registration
        let password = "user_password_123";
        let hash = User::hash_password(password);

        let user = User {
            id: 99,
            username: "newuser".to_string(),
            email: "newuser@example.com".to_string(),
            password_hash: hash,
        };

        // Simulate login verification
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong"));

        // Ensure serialization is safe
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password"));
        assert!(!json.contains(&user.password_hash));
    }
}
