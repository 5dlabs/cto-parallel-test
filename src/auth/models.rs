//! Authentication Models
//!
//! This module provides data models for user authentication, including
//! secure password hashing with Argon2 and request/response DTOs.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity with authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Hashed password (never serialized to JSON)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// Uses Argon2 password verification with constant-time comparison
    /// to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// Returns `true` if the password matches the stored hash, `false` otherwise.
    /// Returns `false` on any error (invalid hash format, verification failure, etc.)
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "abc123xyz";
    /// let hash = User::hash_password(password);
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password(password));
    /// assert!(!user.verify_password("def456uvw"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Parse the stored hash
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        // Verify the password using Argon2
        // Uses constant-time comparison to prevent timing attacks
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// Uses Argon2id algorithm (OWASP recommended) with:
    /// - Random 32-byte salt generated using cryptographically secure RNG
    /// - Default Argon2 parameters (memory, iterations, parallelism)
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// Returns the password hash in PHC string format
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare - would indicate
    /// system resource exhaustion or RNG failure)
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "abc123xyz";
    /// let hash1 = User::hash_password(password);
    /// let hash2 = User::hash_password(password);
    ///
    /// // Each hash should be different due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    pub fn hash_password(password: &str) -> String {
        // Generate a random salt using cryptographically secure RNG
        let salt = SaltString::generate(&mut OsRng);

        // Hash the password using Argon2id (default)
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password");

        password_hash.to_string()
    }
}

/// Request DTO for user login
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password
    pub password: String,
}

/// Request DTO for user registration
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password
    pub password: String,
}

/// Response DTO for successful authentication
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct AuthResponse {
    /// JWT token for authenticated requests
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
    fn test_hash_password_produces_different_hashes() {
        let password = "abc123xyz";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct_password() {
        let password = "abc123xyz";
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
    fn test_verify_password_incorrect_password() {
        let password = "abc123xyz";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("def456uvw"));
    }

    #[test]
    fn test_verify_password_empty_password() {
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
        let password = "t3st!@#$%^&*()";
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
        let password = "тест🔐测试";
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
    fn test_verify_password_long_password() {
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
    fn test_verify_password_invalid_hash_format() {
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
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Password hash should not appear in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("secret_hash_value"));

        // Other fields should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"testpass"}"#;
        let request: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"newuser","email":"new@example.com","password":"newpass"}"#;
        let request: RegisterRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "newpass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate registration: hash a test value
        let password = "testval123";
        let hash = User::hash_password(password);

        // Create user with hashed password
        let user = User {
            id: 1,
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password_hash: hash,
        };

        // Verify the password works
        assert!(user.verify_password(password));

        // Wrong value should fail
        assert!(!user.verify_password("wrongval"));

        // Serialize user (password should be excluded)
        let json = serde_json::to_string(&user).expect("Failed to serialize");
        assert!(!json.contains(&user.password_hash));
    }

    #[test]
    fn test_hash_format_is_argon2() {
        let password = "test";
        let hash = User::hash_password(password);

        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_multiple_users_different_hashes() {
        let password = "test123";

        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: User::hash_password(password),
        };

        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: User::hash_password(password),
        };

        // Even with same password, hashes should differ
        assert_ne!(user1.password_hash, user2.password_hash);

        // Both should verify correctly
        assert!(user1.verify_password(password));
        assert!(user2.verify_password(password));
    }
}
