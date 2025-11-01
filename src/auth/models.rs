use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity representing an authenticated user
///
/// # Security
/// * Password hash is never serialized to JSON (uses `#[serde(skip_serializing)]`)
/// * Passwords are hashed using Argon2 with random salt
/// * Each password hash is unique even for the same password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
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
    /// * `bool` - `true` if the password matches, `false` otherwise
    ///
    /// # Security
    /// * Uses constant-time comparison (provided by Argon2)
    /// * Returns `false` on any error (doesn't leak information about hash validity)
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
    /// * `String` - The Argon2-encoded password hash
    ///
    /// # Security
    /// * Each call generates a unique random salt using OS RNG
    /// * Uses Argon2 default configuration (resistant to brute force attacks)
    /// * Hash includes algorithm parameters and salt (PHC string format)
    ///
    /// # Panics
    /// * Panics if hashing fails (e.g., out of memory)
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

/// Request body for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Request body for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Response body for successful authentication
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
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verification_with_correct_password() {
        let password = "secure_password_456";
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
    fn test_password_verification_fails_with_wrong_password() {
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
    fn test_empty_password_is_handled() {
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
        let password = "p@$$w0rd!#%&*()[]{}";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("different"));
    }

    #[test]
    fn test_unicode_password() {
        let password = "密码🔒";
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
    fn test_invalid_hash_returns_false() {
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
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Password hash should not appear in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Other fields should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "john", "password": "secret123"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize LoginRequest");

        assert_eq!(request.username, "john");
        assert_eq!(request.password, "secret123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username": "jane", "email": "jane@example.com", "password": "pass456"}"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(request.username, "jane");
        assert_eq!(request.email, "jane@example.com");
        assert_eq!(request.password, "pass456");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "alice".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("alice"));
    }
}
