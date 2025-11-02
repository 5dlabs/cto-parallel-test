use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: i32,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Hashed password (excluded from JSON serialization)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash using constant-time comparison
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// `true` if the password matches, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("mypassword");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
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

    /// Hashes a password using Argon2 with a random salt
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// The Argon2 encoded password hash
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, only on system failure)
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash = User::hash_password("secure_password");
    /// assert!(!hash.is_empty());
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

/// Login request containing username and password
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Registration request containing user details and password
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Password
    pub password: String,
}

/// Authentication response containing JWT token and user information
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token
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
    fn test_hash_password_generates_unique_hashes() {
        let password = "testpassword";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes due to random salt"
        );
    }

    #[test]
    fn test_verify_password_with_correct_password() {
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
    fn test_verify_password_with_incorrect_password() {
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
    fn test_verify_password_with_empty_password() {
        let hash = User::hash_password("");

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
    fn test_hash_empty_password() {
        let hash = User::hash_password("");
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_long_password() {
        let long_password = "a".repeat(1000);
        let hash = User::hash_password(&long_password);
        assert!(!hash.is_empty());

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&long_password));
    }

    #[test]
    fn test_hash_unicode_password() {
        let unicode_password = "密码123🔒";
        let hash = User::hash_password(unicode_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(unicode_password));
    }

    #[test]
    fn test_hash_special_characters_password() {
        let special_password = "p@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?";
        let hash = User::hash_password(special_password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(special_password));
    }

    #[test]
    fn test_hash_whitespace_password() {
        let password_with_spaces = "password with spaces";
        let hash = User::hash_password(password_with_spaces);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password_with_spaces));
        assert!(!user.verify_password("passwordwithspaces"));
    }

    #[test]
    fn test_invalid_hash_format_returns_false() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let hash = User::hash_password("secret_password");
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash.clone(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        assert!(!json.contains("password_hash"));
        assert!(!json.contains(&hash));
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username": "john", "password": "secret123"}"#;
        let login_req: LoginRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(login_req.username, "john");
        assert_eq!(login_req.password, "secret123");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username": "john", "email": "john@example.com", "password": "secret123"}"#;
        let reg_req: RegisterRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(reg_req.username, "john");
        assert_eq!(reg_req.email, "john@example.com");
        assert_eq!(reg_req.password, "secret123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "jwt_token_here".to_string(),
            user_id: 42,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");

        assert!(json.contains("jwt_token_here"));
        assert!(json.contains("42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Hash password
        let password = "my_secure_password";
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

        // Create token
        let token =
            crate::auth::jwt::create_token(&user.id.to_string()).expect("Failed to create token");

        // Validate token
        let claims = crate::auth::jwt::validate_token(&token).expect("Failed to validate token");

        assert_eq!(claims.sub, "1");
    }

    #[test]
    fn test_hash_format_is_argon2() {
        let hash = User::hash_password("test");

        // Argon2 encoded format starts with $argon2
        assert!(hash.starts_with("$argon2"));
    }
}
