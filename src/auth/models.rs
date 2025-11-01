//! User authentication models and password hashing
//!
//! This module provides:
//! - User model with secure password verification
//! - Argon2 password hashing with random salt
//! - Authentication request/response DTOs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity representing an authenticated user
///
/// The `password_hash` field is never serialized to JSON for security.
/// Use `hash_password()` to create hashes and `verify_password()` to validate them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User's unique identifier
    pub id: i32,
    /// Username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Argon2 password hash (never serialized)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// Uses Argon2 verification which includes constant-time comparison
    /// to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// Returns `true` if the password matches the hash, `false` otherwise.
    /// Returns `false` on any error (including invalid hash format).
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("secure_password");
    /// let user = User {
    ///     id: 1,
    ///     username: "test".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("secure_password"));
    /// assert!(!user.verify_password("wrong_password"));
    /// ```
    ///
    /// # Security Notes
    ///
    /// - Returns `false` on error to prevent information leakage
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Never panics on invalid input
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

    /// Hashes a password using Argon2 with a random salt
    ///
    /// Each call generates a new random 32-byte salt, ensuring that
    /// identical passwords produce different hashes.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// Returns an Argon2 encoded hash string containing the algorithm
    /// parameters, salt, and hash.
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, usually indicates
    /// system resource issues like insufficient memory).
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let user_pw = "my_secure_password";
    /// let hash1 = User::hash_password(user_pw);
    /// let hash2 = User::hash_password(user_pw);
    ///
    /// // Different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    ///
    /// # Performance
    ///
    /// Argon2 is intentionally slow (~100ms per hash) to resist brute force
    /// attacks. Consider using `tokio::task::spawn_blocking` for async contexts.
    #[must_use]
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
/// Used for user authentication via username and password.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password (will be verified against stored hash)
    pub password: String,
}

/// User registration request DTO
///
/// Contains all required information to create a new user account.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Authentication response DTO
///
/// Returned after successful login or registration.
/// Contains the JWT token and user information.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for subsequent authenticated requests
    pub token: String,
    /// User's unique identifier
    pub user_id: i32,
    /// Username
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_generates_different_hashes() {
        let test_pw = "test_password_123";
        let hash1 = User::hash_password(test_pw);
        let hash2 = User::hash_password(test_pw);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // Both hashes should be non-empty
        assert!(!hash1.is_empty());
        assert!(!hash2.is_empty());
    }

    #[test]
    fn test_verify_password_correct() {
        let test_pw = "my_secure_password";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(test_pw));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let test_pw = "correct_password";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_verify_password_empty_password() {
        let test_pw = "";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(""));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_password_hash_not_serialized() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "id": 42,
            "username": "alice",
            "email": "alice@example.com",
            "password_hash": "hash123"
        }"#;

        let user: User = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(user.id, 42);
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.password_hash, "hash123");
    }

    #[test]
    fn test_password_with_special_characters() {
        let test_pw = "p@ssw0rd!#$%^&*()";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(test_pw));
        assert!(!user.verify_password("p@ssw0rd"));
    }

    #[test]
    fn test_password_with_unicode() {
        let test_pw = "пароль123";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(test_pw));
    }

    #[test]
    fn test_very_long_password() {
        let test_pw = "a".repeat(1000);
        let hash = User::hash_password(&test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&test_pw));
    }

    #[test]
    fn test_password_with_whitespace() {
        let test_pw = "  password with spaces  ";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Whitespace should be preserved
        assert!(user.verify_password(test_pw));
        assert!(!user.verify_password("password with spaces"));
    }

    #[test]
    fn test_verify_with_invalid_hash_format() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "not_a_valid_argon2_hash".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{
            "username": "alice",
            "password": "testpass123"
        }"#;

        let request: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "alice");
        assert_eq!(request.password, "testpass123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "bob",
            "email": "bob@example.com",
            "password": "bobpass456"
        }"#;

        let request: RegisterRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "bob");
        assert_eq!(request.email, "bob@example.com");
        assert_eq!(request.password, "bobpass456");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 123,
            username: "alice".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("123"));
        assert!(json.contains("alice"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate user registration flow
        let test_pw = "user_password_123";
        let hash = User::hash_password(test_pw);

        let user = User {
            id: 1,
            username: "newuser".to_string(),
            email: "newuser@example.com".to_string(),
            password_hash: hash,
        };

        // Simulate login - verify password
        assert!(user.verify_password(test_pw));

        // Generate token
        let token =
            crate::auth::jwt::create_token(&user.id.to_string()).expect("Failed to create token");

        // Validate token
        let claims = crate::auth::jwt::validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "1");

        // Create auth response
        let response = AuthResponse {
            token,
            user_id: user.id,
            username: user.username.clone(),
        };

        // Verify response can be serialized
        let json = serde_json::to_string(&response).expect("Failed to serialize response");
        assert!(!json.contains("password_hash"));
    }
}
