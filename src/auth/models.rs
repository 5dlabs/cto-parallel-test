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
    /// This method uses constant-time comparison through Argon2's verify function
    /// to prevent timing attacks. Returns `false` on any error to avoid leaking
    /// information about the hash validity.
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "secure_password";
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
    /// * `String` - The Argon2-encoded password hash
    ///
    /// # Security
    /// - Uses Argon2 algorithm (winner of Password Hashing Competition)
    /// - Generates a unique random 32-byte salt for each password
    /// - Uses default Argon2 configuration (suitable for most use cases)
    /// - Intentionally slow (~100ms) to resist brute force attacks
    ///
    /// # Panics
    /// Panics if password hashing fails (extremely rare, usually indicates system issues)
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let password = "my_secure_password";
    /// let hash1 = User::hash_password(password);
    /// let hash2 = User::hash_password(password);
    ///
    /// // Hashes are different due to random salt
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
    fn test_password_hashing_produces_unique_hashes() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verification_with_correct_password() {
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
    fn test_password_verification_with_wrong_password() {
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
        let password = "パスワード123";
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
    fn test_password_hash_not_serialized() {
        let test_hash = format!("{}{}", "hash", "ed_test_value");
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: test_hash.clone(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains(&test_hash));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "user1", "password": "pass123"}"#;
        let request: LoginRequest =
            serde_json::from_str(json).expect("Failed to deserialize LoginRequest");

        assert_eq!(request.username, "user1");
        assert_eq!(request.password, "pass123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{
            "username": "newuser",
            "email": "new@example.com",
            "password": "securepass"
        }"#;
        let request: RegisterRequest =
            serde_json::from_str(json).expect("Failed to deserialize RegisterRequest");

        assert_eq!(request.username, "newuser");
        assert_eq!(request.email, "new@example.com");
        assert_eq!(request.password, "securepass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "john_doe".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("john_doe"));
    }

    #[test]
    fn test_invalid_hash_returns_false() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "not_a_valid_argon2_hash".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Step 1: Hash password during registration
        let password = "user_password_123";
        let hash = User::hash_password(password);

        // Step 2: Create user with hashed password
        let user = User {
            id: 1,
            username: "john".to_string(),
            email: "john@example.com".to_string(),
            password_hash: hash,
        };

        // Step 3: Verify password during login
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));

        // Step 4: Ensure password hash is never exposed
        let json = serde_json::to_string(&user).expect("Failed to serialize");
        assert!(!json.contains(&user.password_hash));
    }
}
