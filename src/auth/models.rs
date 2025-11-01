//! User authentication models and password hashing
//!
//! This module provides:
//! - User model with secure password verification
//! - Argon2 password hashing with random salt
//! - Request/Response DTOs for authentication endpoints

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity representing an authenticated user
///
/// The `password_hash` field is excluded from serialization for security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User's email address
    pub email: String,
    /// Argon2 password hash (never serialized to JSON)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// Returns `true` if the password matches, `false` otherwise.
    /// Returns `false` on any error to prevent timing attacks.
    ///
    /// # Security
    ///
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Never panics on verification failure
    /// - Returns `false` for any error condition
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "alice".to_string(),
    ///     email: "alice@example.com".to_string(),
    ///     password_hash: User::hash_password("secret123"),
    /// };
    ///
    /// assert!(user.verify_password("secret123"));
    /// assert!(!user.verify_password("wrong"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// Returns an Argon2 encoded hash string including the salt.
    ///
    /// # Security
    ///
    /// - Uses Argon2 (winner of Password Hashing Competition)
    /// - Generates 32 bytes of random salt for each password
    /// - Each call produces a different hash (due to unique salt)
    /// - Intentionally slow to resist brute force attacks
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, would indicate system issues).
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password123");
    /// let hash2 = User::hash_password("password123");
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
    /// Desired username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Response body for successful authentication
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for subsequent authenticated requests
    pub token: String,
    /// User ID
    pub user_id: i32,
    /// Username
    pub username: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]  // Allow unwrap in tests
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_generates_unique_hashes() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // Both should be non-empty
        assert!(!hash1.is_empty());
        assert!(!hash2.is_empty());
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "secure_password";
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
        assert!(!user.verify_password("CORRECT_PASSWORD"));
    }

    #[test]
    fn test_verify_password_empty() {
        let empty_password = "";
        let hash = User::hash_password(empty_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Empty password should still hash and verify
        assert!(user.verify_password(empty_password));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_verify_password_special_characters() {
        let password = "p@ssw0rd!#$%^&*()";
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
    fn test_verify_password_unicode() {
        let password = "пароль🔐密码";
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
    fn test_verify_password_very_long() {
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
    fn test_verify_password_invalid_hash() {
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
        let user = User {
            id: 42,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("secret_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "alice", "password": "secret"}"#;
        let request: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "alice");
        assert_eq!(request.password, "secret");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username": "bob", "email": "bob@example.com", "password": "secret123"}"#;
        let request: RegisterRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "bob");
        assert_eq!(request.email, "bob@example.com");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 123,
            username: "charlie".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("123"));
        assert!(json.contains("charlie"));
    }

    #[test]
    fn test_password_hash_format() {
        let password = "test123";
        let hash = User::hash_password(password);

        // Argon2 hash should start with $argon2
        assert!(
            hash.starts_with("$argon2"),
            "Hash doesn't match Argon2 format"
        );
    }
}
