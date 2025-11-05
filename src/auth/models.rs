use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

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
    /// * `password` - The password to verify
    ///
    /// # Returns
    /// * `bool` - true if the password is correct, false otherwise
    ///
    /// # Examples
    /// ```
    /// use ecommerce_api::auth::models::User;
    ///
    /// let hash = User::hash_password("example password");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("example password"));
    /// assert!(!user.verify_password("different password"));
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
    /// * `password` - The password to hash
    ///
    /// # Returns
    /// * `String` - The hashed password in Argon2 encoded format
    ///
    /// # Panics
    /// Panics if password hashing fails (should never happen in normal operation)
    ///
    /// # Examples
    /// ```
    /// use ecommerce_api::auth::models::User;
    ///
    /// let hash1 = User::hash_password("example");
    /// let hash2 = User::hash_password("example");
    ///
    /// // Hashes should be different due to random salt
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

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

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
        let input_text = "my test password";
        let hash1 = User::hash_password(input_text);
        let hash2 = User::hash_password(input_text);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

        // Both should verify correctly
        let user1 = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash1,
        };

        assert!(user1.verify_password(input_text));
        assert!(!user1.verify_password("different password"));
    }

    #[test]
    fn test_password_verification() {
        let valid_input = "correct password";
        let hash = User::hash_password(valid_input);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(valid_input));
        assert!(!user.verify_password("wrong password"));
        assert!(!user.verify_password(""));
    }

    #[test]
    fn test_empty_password() {
        let empty_text = "";
        let hash = User::hash_password(empty_text);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(""));
        assert!(!user.verify_password("nonempty"));
    }

    #[test]
    fn test_special_characters_in_password() {
        let special_text = "Sp3c!@l#Ch@rs*()";
        let hash = User::hash_password(special_text);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(special_text));
        assert!(!user.verify_password("Sp3c!@l"));
    }

    #[test]
    fn test_unicode_password() {
        let unicode_text = "テキスト🔒";
        let hash = User::hash_password(unicode_text);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(unicode_text));
        assert!(!user.verify_password("different"));
    }

    #[test]
    fn test_long_password() {
        let long_text = "a".repeat(1000);
        let hash = User::hash_password(&long_text);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&long_text));
        assert!(!user.verify_password("short"));
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "dummy hash value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("dummy hash value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_invalid_hash_format() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any password"));
    }

    #[test]
    fn test_complete_auth_flow() {
        use crate::auth::jwt::{create_token, validate_token};

        // Hash input
        let auth_input = "complete flow test";
        let hash = User::hash_password(auth_input);

        // Create user
        let user = User {
            id: 42,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Verify input
        assert!(user.verify_password(auth_input));

        // Create token
        let token = create_token(&user.id.to_string()).expect("Failed to create token");

        // Validate token
        let claims = validate_token(&token).expect("Failed to validate token");
        assert_eq!(claims.sub, "42");
    }
}
