use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity with authentication information
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
    /// Uses constant-time comparison to prevent timing attacks.
    /// Returns false on any error to avoid leaking information.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `bool` - true if password matches, false otherwise
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("my_password").unwrap();
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("my_password"));
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
    /// Each call generates a unique salt, so the same password
    /// will produce different hashes.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * `Result<String, String>` - The hashed password or error message
    ///
    /// # Errors
    /// Returns an error if password hashing fails
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password").unwrap();
    /// let hash2 = User::hash_password("password").unwrap();
    ///
    /// // Hashes should be different due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| format!("Failed to hash password: {e}"))
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
        let password = "test123";
        let hash1 = User::hash_password(password).unwrap();
        let hash2 = User::hash_password(password).unwrap();

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

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
        let password = "test_pass";
        let hash = User::hash_password(password).unwrap();

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
        let password = "correct_password";
        let hash = User::hash_password(password).unwrap();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_password"));
        assert!(!user.verify_password(""));
        assert!(!user.verify_password("correct_passwor")); // Missing last char
    }

    #[test]
    fn test_empty_password() {
        let hash = User::hash_password("").unwrap();
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
    fn test_long_password() {
        let long_password = "a".repeat(1000);
        let hash = User::hash_password(&long_password).unwrap();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&long_password));
        assert!(!user.verify_password("short"));
    }

    #[test]
    fn test_special_characters_in_password() {
        let passwords = vec![
            "p@ssw0rd!",
            "password with spaces",
            "пароль", // Cyrillic
            "密码",   // Chinese
            "🔒🔑",   // Emoji
            "tab\there",
            "newline\nhere",
        ];

        for password in passwords {
            let hash = User::hash_password(password).unwrap();
            let user = User {
                id: 1,
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password_hash: hash,
            };

            assert!(
                user.verify_password(password),
                "Failed to verify password: {password}"
            );
        }
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
    fn test_user_serialization_excludes_password() {
        let hash = User::hash_password("test_hash").unwrap();
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash.clone(),
        };

        let json = serde_json::to_string(&user).unwrap();

        // Password hash should not appear in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains(&hash));

        // Other fields should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"testpass"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"newuser","email":"new@example.com","password":"newpass"}"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

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

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Hash password
        let password = "test_flow";
        let hash = User::hash_password(password).unwrap();

        // Create user
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Verify password
        assert!(user.verify_password(password));

        // Serialize user (password should be excluded)
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password"));

        // Create auth response
        let response = AuthResponse {
            token: "test_token".to_string(),
            user_id: user.id,
            username: user.username.clone(),
        };

        let response_json = serde_json::to_string(&response).unwrap();
        assert!(response_json.contains("test_token"));
    }
}
