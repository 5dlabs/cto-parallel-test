use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with secure password handling
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
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `bool` - `true` if password matches, `false` otherwise
    ///
    /// # Security
    /// Returns `false` on any error to prevent timing attacks and information leakage
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "example_pass";
    /// let hash = User::hash_password(password);
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password(password));
    /// assert!(!user.verify_password("wrong_password"));
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
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * `String` - The Argon2 encoded hash (includes algorithm, parameters, salt, and hash)
    ///
    /// # Security
    /// - Uses Argon2id algorithm (memory-hard, resistant to GPU attacks)
    /// - Generates unique 32-byte random salt for each password
    /// - Uses default configuration optimized for security
    ///
    /// # Panics
    /// Panics if password hashing fails (should never happen in normal operation)
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "example_pass";
    /// let hash1 = User::hash_password(password);
    /// let hash2 = User::hash_password(password);
    ///
    /// // Hashes are different due to random salt
    /// assert_ne!(hash1, hash2);
    ///
    /// // But both verify correctly
    /// let user = User {
    ///     id: 1,
    ///     username: "test".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash1,
    /// };
    /// assert!(user.verify_password(password));
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
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Registration request DTO
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication response DTO
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
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);
        assert!(!hash1.is_empty());
        assert!(!hash2.is_empty());

        // Both should verify correctly
        let user1 = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash1,
        };

        assert!(user1.verify_password(password));
        assert!(!user1.verify_password("wrong_password"));
    }

    #[test]
    fn test_password_verification_with_correct_password() {
        let password = "example_pass";
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
    fn test_password_verification_with_wrong_password() {
        let password = "example_pass";
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
    fn test_password_hash_format() {
        let hash = User::hash_password("test");

        // Argon2 encoded hash should start with algorithm identifier
        assert!(hash.starts_with("$argon2"));
        assert!(hash.len() > 50); // Reasonable length for encoded hash
    }

    #[test]
    fn test_empty_password() {
        let password = "";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_special_characters_in_password() {
        let password = "test!@#$%";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_unicode_in_password() {
        let password = "パスワード🔐密码";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "test".to_string(),
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
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&password));
    }

    #[test]
    fn test_invalid_hash_format() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash".to_string(),
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
            password_hash: "test_hash_abc123".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("test_hash_abc123"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"userpass"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize login request");

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "userpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"newuser","email":"new@example.com","password":"newpass"}"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize register request");

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "newpass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "test_token".to_string(),
            user_id: 123,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize auth response");

        assert!(json.contains("test_token"));
        assert!(json.contains("123"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Hash password
        let password = "example_pass";
        let hash = User::hash_password(password);

        // Create user
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Verify password
        assert!(user.verify_password(password));

        // Create auth response (simulating successful login)
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: user.id,
            username: user.username.clone(),
        };

        // Verify response can be serialized
        let json = serde_json::to_string(&response).expect("Failed to serialize response");
        assert!(json.contains("jwt_token_here"));
    }
}
