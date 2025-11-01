//! User model and authentication data transfer objects
//!
//! This module provides:
//! - User model with secure password hashing using Argon2
//! - Password verification with constant-time comparison
//! - Authentication request/response DTOs for login and registration
//!
//! # Security Features
//! - Argon2 password hashing with random 32-byte salt
//! - Password hash is never serialized to JSON
//! - Verification failures return false instead of panicking
//! - Each password gets a unique salt to prevent rainbow table attacks

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with secure password handling
///
/// The password hash is never included in serialization to prevent
/// accidental exposure in API responses or logs.
///
/// # Examples
/// ```
/// use cto_parallel_test::auth::models::User;
///
/// let pass = "secure_pass";
/// let hash = User::hash_password(pass);
///
/// let user = User {
///     id: 1,
///     username: "john_doe".to_string(),
///     email: "john@example.com".to_string(),
///     password_hash: hash,
/// };
///
/// assert!(user.verify_password(pass));
/// assert!(!user.verify_password("wrong_pass"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    /// Password hash - never serialized to JSON for security
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// Uses constant-time comparison provided by Argon2 to prevent
    /// timing attacks. Returns false if verification fails or encounters
    /// an error, never panics.
    ///
    /// # Arguments
    /// - `password`: The plaintext password to verify
    ///
    /// # Returns
    /// - `true`: Password matches the stored hash
    /// - `false`: Password doesn't match or verification error occurred
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("correct_pass");
    /// let user = User {
    ///     id: 1,
    ///     username: "test".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("correct_pass"));
    /// assert!(!user.verify_password("wrong_pass"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Parse the stored password hash
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        // Verify the password against the hash
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// Each invocation generates a new random 32-byte salt, ensuring that
    /// identical passwords produce different hashes. This prevents rainbow
    /// table attacks and makes it impossible to identify users with the
    /// same password.
    ///
    /// # Arguments
    /// - `password`: The plaintext password to hash
    ///
    /// # Returns
    /// The Argon2-encoded hash string (includes algorithm, parameters, salt, and hash)
    ///
    /// # Panics
    /// Panics if hashing fails (extremely rare, usually indicates system issues)
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let pass = "my_secure_pass";
    /// let hash1 = User::hash_password(pass);
    /// let hash2 = User::hash_password(pass);
    ///
    /// // Same input produces different hashes
    /// assert_ne!(hash1, hash2);
    ///
    /// // Both hashes verify correctly
    /// let user1 = User {
    ///     id: 1,
    ///     username: "user1".to_string(),
    ///     email: "user1@example.com".to_string(),
    ///     password_hash: hash1,
    /// };
    ///
    /// assert!(user1.verify_password(pass));
    /// ```
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash the password with Argon2
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password");

        password_hash.to_string()
    }
}

/// Login request DTO
///
/// Used for user authentication endpoints
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Registration request DTO
///
/// Used for new user creation endpoints
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication response DTO
///
/// Returned after successful login or registration
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_produces_different_hashes() {
        let password = "test_pass_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes"
        );

        // Both should be valid Argon2 hashes (start with $argon2)
        assert!(
            hash1.starts_with("$argon2"),
            "Hash should be in Argon2 format"
        );
        assert!(
            hash2.starts_with("$argon2"),
            "Hash should be in Argon2 format"
        );
    }

    #[test]
    fn test_password_verification_with_correct_password() {
        let password = "correct_pass";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Correct password should verify successfully"
        );
    }

    #[test]
    fn test_password_verification_with_wrong_password() {
        let password = "correct_pass";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            !user.verify_password("wrong_password"),
            "Wrong password should fail verification"
        );
    }

    #[test]
    fn test_empty_password() {
        let empty_password = "";
        let hash = User::hash_password(empty_password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(empty_password),
            "Empty password should hash and verify"
        );
        assert!(
            !user.verify_password("not_empty"),
            "Non-empty password should not match empty password hash"
        );
    }

    #[test]
    fn test_password_with_special_characters() {
        let password = "p@ssw0rd!#$%^&*()";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Password with special characters should work"
        );
    }

    #[test]
    fn test_password_with_unicode() {
        let password = "пароль密码🔐";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Unicode password should work"
        );
    }

    #[test]
    fn test_long_password() {
        let password = "a".repeat(1000);
        let hash = User::hash_password(&password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(&password),
            "Very long password should work"
        );
    }

    #[test]
    fn test_password_with_whitespace() {
        let password = "  password with spaces  ";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Password with whitespace should work"
        );
        assert!(
            !user.verify_password("password with spaces"),
            "Whitespace should be preserved"
        );
    }

    #[test]
    fn test_invalid_hash_returns_false() {
        let user = User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        assert!(
            !user.verify_password("any_password"),
            "Invalid hash should return false, not panic"
        );
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 42,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_value_should_not_appear".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Serialization should succeed");

        // Verify password_hash is not in JSON
        assert!(
            !json.contains("password_hash"),
            "JSON should not contain password_hash field"
        );
        assert!(
            !json.contains("hashed_value_should_not_appear"),
            "JSON should not contain the hash value"
        );

        // Verify other fields are present
        assert!(json.contains("test_user"), "JSON should contain username");
        assert!(
            json.contains("test@example.com"),
            "JSON should contain email"
        );
        assert!(json.contains("42"), "JSON should contain id");
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"testpass"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Should deserialize LoginRequest");

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"newuser","email":"new@example.com","password":"newpass"}"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Should deserialize RegisterRequest");

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "newpass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt.token.here".to_string(),
            user_id: 123,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Should serialize AuthResponse");

        assert!(json.contains("jwt.token.here"));
        assert!(json.contains("123"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_multiple_users_different_passwords() {
        let pass1 = "pass1";
        let pass2 = "pass2";

        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: User::hash_password(pass1),
        };

        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: User::hash_password(pass2),
        };

        // Each user should only verify their own password
        assert!(user1.verify_password(pass1));
        assert!(!user1.verify_password(pass2));
        assert!(user2.verify_password(pass2));
        assert!(!user2.verify_password(pass1));
    }
}
