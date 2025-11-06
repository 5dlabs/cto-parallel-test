use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    /// Password hash - excluded from JSON serialization for security
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `bool` - true if the password matches, false otherwise
    ///
    /// # Security Notes
    /// - Uses constant-time comparison (provided by Argon2)
    /// - Returns false on any error to prevent timing attacks
    /// - Never panics on invalid input
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hashes a password using Argon2 with random salt
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * `String` - The hashed password in Argon2 encoded format
    ///
    /// # Security Notes
    /// - Generates a new random salt for each password
    /// - Uses Argon2 default configuration (memory-hard, CPU-hard)
    /// - Each call produces a different hash even for the same password
    ///
    /// # Panics
    /// Panics if password hashing fails (e.g., memory allocation failure)
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

        // Hashes should be different due to random salt
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes"
        );
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

        assert!(
            user.verify_password(password),
            "Correct password should verify successfully"
        );
    }

    #[test]
    fn test_password_verification_with_incorrect_password() {
        let password = "correct_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            !user.verify_password("wrong_password"),
            "Incorrect password should fail verification"
        );
    }

    #[test]
    fn test_empty_password_handling() {
        let empty_password = "";
        let hash = User::hash_password(empty_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(empty_password),
            "Empty password should hash and verify correctly"
        );
        assert!(
            !user.verify_password("not_empty"),
            "Non-empty password should not match empty password hash"
        );
    }

    #[test]
    fn test_long_password_handling() {
        let long_password = "a".repeat(1000);
        let hash = User::hash_password(&long_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(&long_password),
            "Long password should verify correctly"
        );
    }

    #[test]
    fn test_special_characters_in_password() {
        let special_password = "p@ssw0rd!#$%^&*()_+-=[]{}|;:',.<>?/~`";
        let hash = User::hash_password(special_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(special_password),
            "Password with special characters should verify correctly"
        );
    }

    #[test]
    fn test_unicode_password_handling() {
        let unicode_password = "пароль密码🔒";
        let hash = User::hash_password(unicode_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(unicode_password),
            "Unicode password should verify correctly"
        );
    }

    #[test]
    fn test_whitespace_in_password_preserved() {
        let password_with_spaces = "pass word with spaces";
        let hash = User::hash_password(password_with_spaces);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(
            user.verify_password(password_with_spaces),
            "Password with spaces should verify correctly"
        );
        assert!(
            !user.verify_password("passwordwithspaces"),
            "Password without spaces should not match"
        );
    }

    #[test]
    fn test_invalid_hash_format_returns_false() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        assert!(
            !user.verify_password("any_password"),
            "Invalid hash format should return false, not panic"
        );
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "sensitive_hash_value_should_not_appear".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Verify password_hash field is not in JSON
        assert!(
            !json.contains("password_hash"),
            "Serialized JSON should not contain 'password_hash' field"
        );
        assert!(
            !json.contains("sensitive_hash_value_should_not_appear"),
            "Serialized JSON should not contain password hash value"
        );

        // Verify other fields are present
        assert!(json.contains("testuser"), "Username should be in JSON");
        assert!(json.contains("test@example.com"), "Email should be in JSON");
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "johndoe", "password": "secret123"}"#;
        let login_request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize LoginRequest");

        assert_eq!(login_request.username, "johndoe");
        assert_eq!(login_request.password, "secret123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "newuser",
            "email": "newuser@example.com",
            "password": "newpass456"
        }"#;
        let register_request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(register_request.username, "newuser");
        assert_eq!(register_request.email, "newuser@example.com");
        assert_eq!(register_request.password, "newpass456");
    }

    #[test]
    fn test_auth_response_serialization() {
        let auth_response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "activeuser".to_string(),
        };

        let json = serde_json::to_string(&auth_response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("activeuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Simulate user registration
        let password = "registration_password";
        let hash = User::hash_password(password);

        // Create user with hashed password
        let user = User {
            id: 100,
            username: "flowtest".to_string(),
            email: "flow@example.com".to_string(),
            password_hash: hash,
        };

        // Simulate login - verify password
        assert!(
            user.verify_password(password),
            "User should be able to login with correct password"
        );

        // Verify wrong password fails
        assert!(
            !user.verify_password("wrong_password"),
            "Login should fail with wrong password"
        );

        // Verify user can be serialized without exposing password
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password"));
    }
}
