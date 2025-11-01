//! # Authentication Models
//!
//! This module provides data structures for user authentication including:
//! - User model with secure password handling
//! - Authentication request/response DTOs
//! - Argon2 password hashing with random salt

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity with authentication credentials
///
/// # Security
/// - `password_hash` is never serialized to JSON (marked with `skip_serializing`)
/// - Passwords are hashed using Argon2 with random salt
/// - Password verification is constant-time to prevent timing attacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login and display
    pub username: String,
    /// User email address
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
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("mypassword");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("mypassword"));
    /// assert!(!user.verify_password("wrongpassword"));
    /// ```
    ///
    /// # Security Notes
    /// - Uses constant-time comparison to prevent timing attacks
    /// - Returns false on any error (doesn't leak information)
    /// - Argon2 verification is intentionally slow (~100ms) to resist brute force
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
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * Argon2-encoded password hash string
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password");
    /// let hash2 = User::hash_password("password");
    ///
    /// // Same password produces different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    ///
    /// # Security Notes
    /// - Uses cryptographically secure random 32-byte salt
    /// - Each password hash is unique due to random salt
    /// - Argon2 is memory-hard to resist GPU attacks
    /// - Hash includes algorithm parameters for future compatibility
    ///
    /// # Panics
    /// Panics if password hashing fails (extremely rare, usually indicates system issues)
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}

/// Login request data transfer object
///
/// # Example
/// ```
/// use cto_parallel_test::auth::models::LoginRequest;
///
/// let request = LoginRequest {
///     username: "john_doe".to_string(),
///     password: "secure_password".to_string(),
/// };
/// ```
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username or email for login
    pub username: String,
    /// Plaintext password (transmitted over HTTPS only)
    pub password: String,
}

/// User registration request data transfer object
///
/// # Example
/// ```
/// use cto_parallel_test::auth::models::RegisterRequest;
///
/// let request = RegisterRequest {
///     username: "john_doe".to_string(),
///     email: "john@example.com".to_string(),
///     password: "secure_password".to_string(),
/// };
/// ```
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username (must be unique)
    pub username: String,
    /// Email address (must be unique and valid)
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Authentication response data transfer object
///
/// Returned after successful login or registration
///
/// # Example
/// ```
/// use cto_parallel_test::auth::models::AuthResponse;
///
/// let response = AuthResponse {
///     token: "eyJhbGciOiJIUzI1NiIs...".to_string(),
///     user_id: 123,
///     username: "john_doe".to_string(),
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT authentication token (24-hour expiration)
    pub token: String,
    /// User's unique identifier
    pub user_id: i32,
    /// Username for display
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_uniqueness() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

        // Both hashes should start with $argon2 prefix
        assert!(hash1.starts_with("$argon2"));
        assert!(hash2.starts_with("$argon2"));
    }

    #[test]
    fn test_password_verification_success() {
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
    fn test_password_verification_failure() {
        let password = "correct_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_password_with_special_characters() {
        let password = "p@ssw0rd!#$%^&*()";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("p@ssw0rd"));
    }

    #[test]
    fn test_empty_password() {
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
    fn test_very_long_password() {
        let password = "a".repeat(1000);
        let hash = User::hash_password(&password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&password));
        assert!(!user.verify_password(&"a".repeat(999)));
    }

    #[test]
    fn test_unicode_password() {
        let password = "пароль🔒密码";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("password"));
    }

    #[test]
    fn test_invalid_hash_format() {
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
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("secret_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "john_doe", "password": "secret123"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "john_doe");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "john_doe",
            "email": "john@example.com",
            "password": "secret123"
        }"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "john_doe");
        assert_eq!(request.email, "john@example.com");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 123,
            username: "john_doe".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("123"));
        assert!(json.contains("john_doe"));
    }
}
