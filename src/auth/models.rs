use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user in the system.
/// The password hash is never serialized to JSON for security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// `true` if the password matches, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: User::hash_password("mypassword"),
    /// };
    ///
    /// assert!(user.verify_password("mypassword"));
    /// assert!(!user.verify_password("wrongpassword"));
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

    /// Hash a password using Argon2 with a random salt.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// An Argon2 encoded hash string
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (should not happen in practice)
    ///
    /// # Example
    ///
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("password");
    /// let hash2 = User::hash_password("password");
    /// assert_ne!(hash1, hash2); // Different due to random salt
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

/// Request body for user login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Request body for user registration.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Response body for successful authentication.
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
    fn test_hash_password_different_salts() {
        let password = "test_password_123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "test_password_123";
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
    fn test_verify_password_incorrect() {
        let password = "test_password_123";
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
    fn test_verify_password_empty() {
        let password = "";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("nonempty"));
    }

    #[test]
    fn test_verify_password_special_chars() {
        let password = "p@ssw0rd!#$%^&*()";
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
    fn test_verify_password_unicode() {
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
    fn test_verify_password_very_long() {
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
    fn test_verify_password_invalid_hash_format() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("password"));
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
        let json =
            r#"{"username": "testuser", "email": "test@example.com", "password": "testpass"}"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "testuser");
        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "testpass");
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
    fn test_complete_auth_flow() {
        // Hash password
        let password = "mypassword";
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

        // Create token (using the jwt module)
        let token = crate::auth::jwt::create_token(&user.id.to_string()).unwrap();

        // Validate token
        let claims = crate::auth::jwt::validate_token(&token).unwrap();
        assert_eq!(claims.sub, "1");

        // Verify user can be serialized without password
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password_hash"));
    }
}
