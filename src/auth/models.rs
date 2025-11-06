use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with secure password hashing
///
/// # Security Notes
/// - Password hash is excluded from JSON serialization via `#[serde(skip_serializing)]`
/// - Uses Argon2 algorithm for password hashing (OWASP recommended)
/// - Random 32-byte salt generated for each password
/// - Never store or transmit plaintext passwords
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
    /// * `bool` - True if password matches, false otherwise
    ///
    /// # Security Notes
    /// - Uses constant-time comparison via Argon2
    /// - Returns false on any error (doesn't leak information)
    /// - Safe against timing attacks
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
    /// # Security Notes
    /// - Uses Argon2 default configuration (secure for most use cases)
    /// - Generates unique random salt for each password
    /// - Same password will produce different hashes (due to random salt)
    /// - Hash includes all information needed for verification
    ///
    /// # Panics
    /// Panics if password hashing fails (rare, usually indicates system issues)
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
    pub username: String,
    pub password: String,
}

/// Request payload for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Response payload for successful authentication
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

        // Same password should produce different hashes (due to random salt)
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verification_success() {
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
    fn test_password_verification_failure() {
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
    fn test_empty_password() {
        let password = "";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(""));
        assert!(!user.verify_password("something"));
    }

    #[test]
    fn test_special_characters_in_password() {
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
    fn test_unicode_password() {
        let password = "пароль🔒";
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
    fn test_long_password() {
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
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();

        // Password hash should NOT be in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Other fields should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"id\":1"));
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
        assert!(json.contains("\"user_id\":42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_whitespace_in_password() {
        let password = "  password with spaces  ";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Whitespace should be preserved
        assert!(user.verify_password(password));
        assert!(!user.verify_password("password with spaces"));
    }
}
