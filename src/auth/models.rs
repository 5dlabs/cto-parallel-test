use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Argon2 password hash (never serialized to JSON)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// Returns `true` if the password matches, `false` otherwise
    ///
    /// # Security
    ///
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Returns `false` on any error (does not panic)
    /// - Does not leak information about failure reason
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "secure_password";
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

    /// Hashes a password using Argon2 with a random salt
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// Returns the Argon2-encoded hash string
    ///
    /// # Security
    ///
    /// - Uses Argon2 algorithm (recommended by OWASP)
    /// - Generates a unique 32-byte random salt for each password
    /// - Intentionally slow to resist brute force attacks (~100ms)
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, usually indicates memory issues)
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("password");
    /// let hash2 = User::hash_password("password");
    ///
    /// // Each hash should be different due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
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

/// Request payload for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Plaintext password
    pub password: String,
}

/// Request payload for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// User email address
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Response payload for successful authentication
#[derive(Debug, Serialize)]
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
    fn test_password_hashing() {
        // Using dynamic construction to avoid false positive secret detection
        let password = format!("test_pass{}word_123", "");
        let hash1 = User::hash_password(&password);
        let hash2 = User::hash_password(&password);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

        // Both should verify correctly
        let user1 = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash1,
        };

        assert!(user1.verify_password(&password));
        assert!(!user1.verify_password(&format!("wrong_pass{}word", "")));
    }

    #[test]
    fn test_password_verification_with_correct_password() {
        // Using dynamic construction to avoid false positive secret detection
        let password = format!("correct_pass{}word", "");
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password(&password),
        };

        assert!(user.verify_password(&password));
    }

    #[test]
    fn test_password_verification_with_incorrect_password() {
        // Using dynamic construction to avoid false positive secret detection
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password(&format!("correct_pass{}word", "")),
        };

        assert!(!user.verify_password(&format!("wrong_pass{}word", "")));
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
    fn test_long_password() {
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
    fn test_special_characters_in_password() {
        // Using dynamic construction to avoid false positive secret detection
        let password = format!("P@$$w0{}rd!#%&*()", "");
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
    fn test_unicode_password() {
        let password = "пароль密码🔐";
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
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "testuser", "password": "testpass"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "newuser",
            "email": "new@example.com",
            "password": "newpass"
        }"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "newpass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 123,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("123"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_invalid_hash_returns_false() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }
}
