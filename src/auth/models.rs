use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with authentication capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Hashed password (excluded from serialization for security)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Creates a new User instance
    ///
    /// # Arguments
    /// * `id` - Unique user identifier
    /// * `username` - Username for the user
    /// * `email` - Email address
    /// * `password_hash` - Pre-hashed password (use `hash_password` to generate)
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let password_hash = User::hash_password("example123").expect("Failed to hash");
    /// let user = User::new(1, "john_doe".to_string(), "john@example.com".to_string(), password_hash);
    /// assert_eq!(user.username, "john_doe");
    /// ```
    #[must_use]
    pub fn new(id: i32, username: String, email: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
        }
    }

    /// Verifies a plaintext password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `true` if the password matches
    /// * `false` if the password doesn't match or verification fails
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let password = "example123";
    /// let hash = User::hash_password(password).expect("Failed to hash");
    /// let user = User::new(1, "test".to_string(), "test@example.com".to_string(), hash);
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

    /// Hashes a plaintext password using Argon2 with a random salt
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * `Ok(String)` - The hashed password
    /// * `Err` - If hashing fails
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash = User::hash_password("example123").expect("Failed to hash password");
    /// assert!(!hash.is_empty());
    /// assert_ne!(hash, "example123"); // Should be hashed, not plaintext
    /// ```
    ///
    /// # Errors
    /// Returns an error if the password hashing operation fails
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "testvalue123";
        let result = User::hash_password(password);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password); // Hash should not be the plaintext password
    }

    #[test]
    fn test_hash_password_generates_unique_hashes() {
        let password = "samevalue";
        let hash1 = User::hash_password(password).expect("Failed to hash password");
        let hash2 = User::hash_password(password).expect("Failed to hash password");

        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "testvalue";
        let hash = User::hash_password(password).expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "testvalue";
        let hash = User::hash_password(password).expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        assert!(!user.verify_password("wrongvalue"));
    }

    #[test]
    fn test_verify_password_empty() {
        let password = "testvalue";
        let hash = User::hash_password(password).expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        assert!(!user.verify_password(""));
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            "invalid_hash".to_string(),
        );

        assert!(!user.verify_password("anyvalue"));
    }

    #[test]
    fn test_user_new() {
        let user = User::new(
            42,
            "john_doe".to_string(),
            "john@example.com".to_string(),
            "hashed_value".to_string(),
        );

        assert_eq!(user.id, 42);
        assert_eq!(user.username, "john_doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.password_hash, "hashed_value");
    }

    #[test]
    fn test_user_serialization_excludes_password() {
        let hash = User::hash_password("testvalue").expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        let serialized = serde_json::to_string(&user).expect("Failed to serialize");
        assert!(!serialized.contains("password_hash"));
        assert!(serialized.contains("username"));
        assert!(serialized.contains("email"));
        assert!(serialized.contains("id"));
    }

    #[test]
    fn test_user_clone() {
        let hash = User::hash_password("testvalue").expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        let cloned = user.clone();
        assert_eq!(user, cloned);
    }

    #[test]
    fn test_hash_password_with_special_characters() {
        let password = "sp3c!@l#$%^&*()";
        let hash = User::hash_password(password).expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_hash_password_with_unicode() {
        let password = "тест测试🔐";
        let hash = User::hash_password(password).expect("Failed to hash password");
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            hash,
        );

        assert!(user.verify_password(password));
    }

    #[test]
    fn test_hash_long_password() {
        let password = "a".repeat(1000);
        let result = User::hash_password(&password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_equality() {
        let hash = "same_hash".to_string();
        let user1 = User::new(
            1,
            "user".to_string(),
            "user@example.com".to_string(),
            hash.clone(),
        );
        let user2 = User::new(1, "user".to_string(), "user@example.com".to_string(), hash);

        assert_eq!(user1, user2);
    }
}
