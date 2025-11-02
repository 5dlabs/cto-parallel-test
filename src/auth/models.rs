//! User models and password hashing.
//!
//! This module provides secure password hashing using Argon2 and
//! user authentication data transfer objects (DTOs).

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity with authentication information.
///
/// The password hash is never serialized to JSON for security reasons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Argon2 password hash (never serialized)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash.
    ///
    /// Uses constant-time comparison to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// `true` if the password is correct, `false` otherwise
    ///
    /// # Security
    ///
    /// Returns `false` on any error to prevent information leakage.
    /// Uses Argon2's built-in constant-time comparison.
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("secret123");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("secret123"));
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

    /// Hashes a password using Argon2 with a random salt.
    ///
    /// Each call generates a unique hash even for the same password,
    /// due to the random salt generation.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// An Argon2 encoded hash string
    ///
    /// # Panics
    ///
    /// Panics if hashing fails (rare, usually indicates system issues)
    ///
    /// # Security
    ///
    /// - Uses Argon2 (winner of Password Hashing Competition)
    /// - Generates random salt for each password
    /// - Intentionally slow to resist brute force attacks
    /// - Default Argon2 config balances security and performance
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password123");
    /// let hash2 = User::hash_password("password123");
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

/// Request body for user login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password
    pub password: String,
}

/// Request body for user registration.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Response body for successful authentication.
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
    fn test_password_hashing_produces_unique_hashes() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_correct_password_verifies() {
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
    fn test_incorrect_password_fails() {
        let password = "secure_password";
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
    fn test_empty_password_handled() {
        let password = "";
        let hash = User::hash_password(password);

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
    fn test_special_characters_in_password() {
        let password = "p@ssw0rd!#$%^&*()_+-=[]{}|;:',.<>?/";
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
    fn test_unicode_password() {
        let password = "пароль密码🔒";
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
    }

    #[test]
    fn test_whitespace_preserved() {
        let password = "  pass  word  ";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("pass  word"));
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_invalid_hash_format_returns_false() {
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
    fn test_login_request_deserialization() {
        let json = r#"{"username": "testuser", "password": "testpass"}"#;
        let request: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username": "newuser", "email": "new@example.com", "password": "newpass"}"#;
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
}
