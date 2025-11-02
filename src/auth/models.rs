use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with secure password handling
///
/// # Security Notes
///
/// - Password hash is never serialized to JSON (`serde(skip_serializing)`)
/// - Each password gets a unique random 32-byte salt
/// - Uses Argon2 algorithm for password hashing
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
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// * `bool` - true if password matches, false otherwise
    ///
    /// # Security Notes
    ///
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Returns false on any error (never panics)
    /// - Safe against timing attacks
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        PasswordHash::new(&self.password_hash)
            .ok()
            .and_then(|parsed_hash| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .ok()
            })
            .is_some()
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// * `String` - The Argon2 encoded password hash
    ///
    /// # Security Notes
    ///
    /// - Uses cryptographically secure random salt (unique for each password)
    /// - Argon2 is intentionally slow (~100ms) to resist brute force
    /// - Hash includes algorithm, parameters, salt, and hash value
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, usually indicates system issues)
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

/// Response body for authentication endpoints
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
    fn test_password_hashing_unique_hashes() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

        // Both hashes should be in Argon2 format
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
    fn test_special_characters_in_password() {
        let password = "p@ssw0rd!#$%^&*()_+-=[]{}|;:',.<>?/~`";
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
    fn test_whitespace_in_password() {
        let password = "  password with spaces  ";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("password with spaces")); // Without leading/trailing spaces
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
        assert!(json.contains(r#""id":1"#));
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
            user_id: 123,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("testuser"));
        assert!(json.contains(r#""user_id":123"#));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate registration: hash password
        let password = "mypassword123";
        let hash = User::hash_password(password);

        // Create user
        let user = User {
            id: 1,
            username: "newuser".to_string(),
            email: "new@example.com".to_string(),
            password_hash: hash,
        };

        // Simulate login: verify password
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrongpassword"));

        // Verify serialization safety
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password"));
    }

    #[test]
    fn test_multiple_users_independent_hashes() {
        let password = "samepassword";

        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: User::hash_password(password),
        };

        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: User::hash_password(password),
        };

        // Different hashes for same password
        assert_ne!(user1.password_hash, user2.password_hash);

        // Both verify correctly
        assert!(user1.verify_password(password));
        assert!(user2.verify_password(password));
    }
}
