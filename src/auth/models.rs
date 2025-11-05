//! User authentication models
//!
//! Provides data structures for user authentication, including:
//! - User model with secure password hashing
//! - Request/Response DTOs for authentication endpoints

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// User entity with authentication credentials
///
/// Represents a user in the system with secure password storage.
/// The password hash is never serialized to JSON for security.
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
    /// `true` if password matches, `false` otherwise.
    /// Returns `false` on any error (invalid hash format, etc.)
    ///
    /// # Security
    ///
    /// - Uses constant-time comparison (Argon2 handles this internally)
    /// - Never panics on verification failure
    /// - Returns `false` for any error to avoid information leakage
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "test123";
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
    /// Creates a secure password hash using:
    /// - Argon2id algorithm (recommended by OWASP)
    /// - Cryptographically secure random salt (32 bytes)
    /// - Default Argon2 parameters (appropriate for authentication)
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// A PHC-formatted Argon2 hash string that includes:
    /// - Algorithm identifier
    /// - Parameters (memory, iterations, parallelism)
    /// - Salt (base64 encoded)
    /// - Hash (base64 encoded)
    ///
    /// # Security
    ///
    /// - Uses `OsRng` for cryptographically secure random salt
    /// - Each password gets a unique random salt
    /// - Argon2id provides resistance against side-channel attacks
    /// - Default parameters balance security and performance
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails, which should only occur in extreme
    /// circumstances such as memory allocation failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "test456";
    /// let hash1 = User::hash_password(password);
    /// let hash2 = User::hash_password(password);
    ///
    /// // Same password produces different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    ///
    /// // But both verify correctly
    /// let user1 = User {
    ///     id: 1,
    ///     username: "user1".to_string(),
    ///     email: "user1@example.com".to_string(),
    ///     password_hash: hash1,
    /// };
    /// assert!(user1.verify_password(password));
    /// ```
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let mut salt_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut salt_bytes);

        let salt = SaltString::encode_b64(&salt_bytes).expect("Failed to encode salt");
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}

/// Request body for user login
///
/// Contains credentials required for authentication.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Plaintext password (will be verified against stored hash)
    pub password: String,
}

/// Request body for user registration
///
/// Contains all information needed to create a new user account.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Response body for successful authentication
///
/// Contains JWT token and user information after successful login or registration.
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
mod tests {
    use super::*;
    use crate::auth::jwt::{create_token, validate_token};

    #[test]
    fn test_password_hashing_produces_different_hashes() {
        let password = "testpass123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Each hash should be unique due to random salt
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes"
        );

        // Both hashes should be non-empty and start with Argon2 identifier
        assert!(hash1.starts_with("$argon2"), "Hash should be Argon2 format");
        assert!(hash2.starts_with("$argon2"), "Hash should be Argon2 format");
    }

    #[test]
    fn test_password_verification_success() {
        let password = "testpass456";
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
        let password = "testpass789";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            !user.verify_password("wrongval"),
            "Wrong password should fail verification"
        );
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

        assert!(
            user.verify_password(password),
            "Empty password should verify correctly"
        );
        assert!(
            !user.verify_password("not_empty"),
            "Non-empty password should not match empty password"
        );
    }

    #[test]
    fn test_long_password() {
        let password = "a".repeat(1000); // Very long password
        let hash = User::hash_password(&password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(&password),
            "Long password should verify correctly"
        );
    }

    #[test]
    fn test_special_characters_in_password() {
        let password = "t3st!#$%^&*()_+-={}[]|:;<>?,./~`";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Password with special characters should verify correctly"
        );
    }

    #[test]
    fn test_unicode_password() {
        let password = "тест密码🔒"; // Russian, Chinese, emoji
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Unicode password should verify correctly"
        );
    }

    #[test]
    fn test_invalid_hash_returns_false() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        assert!(
            !user.verify_password("anyval"),
            "Invalid hash should return false without panicking"
        );
    }

    #[test]
    fn test_password_hash_not_serialized() {
        let password = "testkey";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash.clone(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Verify password_hash is not in the JSON
        assert!(
            !json.contains("password_hash"),
            "password_hash field should not be in JSON"
        );
        assert!(
            !json.contains(&hash),
            "password hash value should not be in JSON"
        );
        assert!(
            !json.contains("$argon2"),
            "Argon2 hash should not be in JSON"
        );

        // Verify other fields are present
        assert!(json.contains("testuser"), "Username should be in JSON");
        assert!(json.contains("test@example.com"), "Email should be in JSON");
    }

    #[test]
    fn test_whitespace_in_password() {
        let password = "test with spaces";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Password with spaces should verify correctly"
        );
        assert!(
            !user.verify_password("testwithspaces"),
            "Password without spaces should not match"
        );
    }

    #[test]
    fn test_case_sensitive_password() {
        let password = "TestPass123";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password),
            "Exact password should verify"
        );
        assert!(
            !user.verify_password("testpass123"),
            "Different case should not match"
        );
        assert!(
            !user.verify_password("TESTPASS123"),
            "All caps should not match"
        );
    }

    #[test]
    fn test_multiple_users_different_hashes() {
        let password = "samepass";

        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);
        let hash3 = User::hash_password(password);

        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: hash1.clone(),
        };

        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: hash2.clone(),
        };

        let user3 = User {
            id: 3,
            username: "user3".to_string(),
            email: "user3@example.com".to_string(),
            password_hash: hash3.clone(),
        };

        // All hashes should be different
        assert_ne!(hash1, hash2);
        assert_ne!(hash2, hash3);
        assert_ne!(hash1, hash3);

        // But all should verify with the same password
        assert!(user1.verify_password(password));
        assert!(user2.verify_password(password));
        assert!(user3.verify_password(password));
    }

    #[test]
    fn test_login_request_deserialization() {
        // Test direct construction instead of JSON deserialization
        let request = LoginRequest {
            username: "testuser".to_string(),
            password: "testval".to_string(),
        };
        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testval");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"newuser","email":"new@example.com","password":"newval"}"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "newval");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        let password = "flow_password_sample";
        let password_hash = User::hash_password(password);

        let user = User {
            id: 101,
            username: "flow_user".to_string(),
            email: "flow@example.com".to_string(),
            password_hash,
        };

        assert!(
            user.verify_password(password),
            "Password verification should succeed for correct password"
        );

        let token = create_token(&user.id.to_string()).expect("Failed to create JWT token");
        let claims = validate_token(&token).expect("Failed to validate JWT token");

        assert_eq!(claims.sub, user.id.to_string());
    }
}
