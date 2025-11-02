//! Authentication Models and DTOs
//!
//! This module defines the User model with password hashing functionality
//! and request/response DTOs for authentication endpoints.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with authentication capabilities
///
/// Represents a user in the system with secure password storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User's email address
    pub email: String,
    /// Argon2 password hash (never exposed in JSON)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `true` - If the password matches the stored hash
    /// * `false` - If the password doesn't match or verification fails
    ///
    /// # Security
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Returns false on any error (no information leakage)
    /// - Never panics on invalid hash format
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash = User::hash_password("secure_password");
    /// let user = User {
    ///     id: 1,
    ///     username: "john".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("secure_password"));
    /// assert!(!user.verify_password("wrong_password"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Parse the stored hash
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        // Verify password against hash
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// The Argon2-encoded password hash string
    ///
    /// # Security
    /// - Uses Argon2 algorithm (OWASP recommended)
    /// - Generates unique random salt for each password
    /// - Same password produces different hashes (due to random salt)
    /// - Intentionally slow to resist brute-force attacks (~100ms)
    ///
    /// # Panics
    /// Panics if password hashing fails (extremely rare, indicates system issues)
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("my_password");
    /// let hash2 = User::hash_password("my_password");
    ///
    /// // Hashes are different due to random salt
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

/// Login request DTO
///
/// Used for user authentication endpoints.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password (will be verified against hash)
    pub password: String,
}

/// Registration request DTO
///
/// Used for new user registration endpoints.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username (must be unique)
    pub username: String,
    /// User's email address
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Authentication response DTO
///
/// Returned after successful login or registration.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for authenticated requests
    pub token: String,
    /// User's unique identifier
    pub user_id: i32,
    /// User's username
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
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes"
        );
    }

    #[test]
    fn test_password_verification_success() {
        let password = "secure_password_456";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Correct password should verify successfully"
        );
    }

    #[test]
    fn test_password_verification_failure() {
        let password = "secure_password_789";
        let hash = User::hash_password(password);

        let user = User {
            id: 2,
            username: "testuser2".to_string(),
            email: "test2@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            !user.verify_password("wrong_password"),
            "Incorrect password should fail verification"
        );
    }

    #[test]
    fn test_empty_password() {
        let empty_password = "";
        let hash = User::hash_password(empty_password);

        let user = User {
            id: 3,
            username: "testuser3".to_string(),
            email: "test3@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(empty_password),
            "Empty password should hash and verify correctly"
        );
        assert!(
            !user.verify_password("not_empty"),
            "Wrong password should fail even if stored password is empty"
        );
    }

    #[test]
    fn test_special_characters_in_password() {
        let password = "p@ssw0rd!#$%^&*()";
        let hash = User::hash_password(password);

        let user = User {
            id: 4,
            username: "testuser4".to_string(),
            email: "test4@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Password with special characters should verify correctly"
        );
    }

    #[test]
    fn test_long_password() {
        let password = "a".repeat(1000);
        let hash = User::hash_password(&password);

        let user = User {
            id: 5,
            username: "testuser5".to_string(),
            email: "test5@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(&password),
            "Long password should hash and verify correctly"
        );
    }

    #[test]
    fn test_unicode_password() {
        let password = "пароль密码🔒";
        let hash = User::hash_password(password);

        let user = User {
            id: 6,
            username: "testuser6".to_string(),
            email: "test6@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Unicode password should hash and verify correctly"
        );
    }

    #[test]
    fn test_invalid_hash_format() {
        let user = User {
            id: 7,
            username: "testuser7".to_string(),
            email: "test7@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(
            !user.verify_password("any_password"),
            "Invalid hash format should return false, not panic"
        );
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let password = "secret_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 8,
            username: "testuser8".to_string(),
            email: "test8@example.com".to_string(),
            password_hash: hash.clone(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Verify password_hash field is not in JSON
        assert!(
            !json.contains("password_hash"),
            "JSON should not contain password_hash field"
        );
        assert!(
            !json.contains(&hash),
            "JSON should not contain password hash value"
        );

        // Verify other fields are present
        assert!(json.contains("testuser8"), "JSON should contain username");
        assert!(
            json.contains("test8@example.com"),
            "JSON should contain email"
        );
        assert!(json.contains("\"id\":8"), "JSON should contain id");
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "john_doe", "password": "secret123"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize LoginRequest");

        assert_eq!(request.username, "john_doe");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "jane_doe",
            "email": "jane@example.com",
            "password": "secure_pass"
        }"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(request.username, "jane_doe");
        assert_eq!(request.email, "jane@example.com");
        assert_eq!(request.password, "secure_pass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
            user_id: 42,
            username: "test_user".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("token"));
        assert!(json.contains("user_id"));
        assert!(json.contains("username"));
        assert!(json.contains("test_user"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate registration: hash password
        let password = "user_password_123";
        let hash = User::hash_password(password);

        // Create user with hashed password
        let user = User {
            id: 100,
            username: "flow_test_user".to_string(),
            email: "flowtest@example.com".to_string(),
            password_hash: hash,
        };

        // Simulate login: verify password
        assert!(
            user.verify_password(password),
            "Password verification should succeed in complete flow"
        );

        // Verify wrong password fails
        assert!(
            !user.verify_password("wrong_password"),
            "Wrong password should fail in complete flow"
        );

        // Verify serialization is safe
        let json = serde_json::to_string(&user).expect("Failed to serialize user");
        assert!(
            !json.contains("password_hash"),
            "Serialized user should not expose password hash"
        );
    }
}
