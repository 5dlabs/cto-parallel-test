use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a plain text password against the stored hash.
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `bool` - True if the password matches, false otherwise
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash = User::hash_password("my_password");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
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

    /// Hashes a plain text password using Argon2 with a random salt.
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// * `String` - The encoded password hash
    ///
    /// # Panics
    /// Panics if password hashing fails (should not occur with valid input)
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("password123");
    /// let hash2 = User::hash_password("password123");
    /// // Each hash is different due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        // Generate a random salt using the system's cryptographic random number generator
        let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "secure_password";
        let hashed = User::hash_password(password);
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_user_serialization_skips_password() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash123".to_string(),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("password_hash"));
        assert!(json.contains("username"));
    }

    #[test]
    fn test_different_salts_produce_different_hashes() {
        let password = "same_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);
        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_wrong_password_returns_false() {
        let password = "correct_password";
        let hashed = User::hash_password(password);
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };
        assert!(!user.verify_password("wrong_password"));
        assert!(!user.verify_password(""));
        assert!(!user.verify_password("correct_passwor")); // Missing last character
    }

    #[test]
    fn test_user_deserialization_with_password_hash() {
        let json = r#"{
            "id": 1,
            "username": "testuser",
            "email": "test@example.com",
            "password_hash": "hash123"
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hash123");
    }

    #[test]
    fn test_hash_format_is_valid_argon2() {
        let password = "test_password";
        let hash = User::hash_password(password);
        // Argon2 hashes start with $argon2
        assert!(hash.starts_with("$argon2"));
    }
}
