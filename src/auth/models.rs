use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user
///
/// # Security Features
/// - Password hash is never serialized in JSON responses
/// - Uses Argon2 for password hashing (memory-hard, GPU-resistant)
/// - Each password gets a unique random salt
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login and display
    pub username: String,
    /// User's email address
    pub email: String,
    /// Argon2 password hash (never exposed in API responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a plain text password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `true` - Password matches the stored hash
    /// * `false` - Password does not match or verification failed
    ///
    /// # Security
    /// Uses Argon2's constant-time comparison to prevent timing attacks
    ///
    /// # Example
    /// ```
    /// # use cto_parallel_test::auth::User;
    /// let hash = User::hash_password("secure_password");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password("secure_password"));
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

    /// Hashes a plain text password using Argon2
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// Argon2-encoded hash string containing the algorithm parameters, salt, and hash
    ///
    /// # Security Features
    /// - Generates a random 32-byte salt for each password
    /// - Uses Argon2 default configuration (balanced security/performance)
    /// - Produces a self-contained encoded string with all parameters
    ///
    /// # Example
    /// ```
    /// # use cto_parallel_test::auth::User;
    /// let hash1 = User::hash_password("mypassword");
    /// let hash2 = User::hash_password("mypassword");
    ///
    /// // Same password produces different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    ///
    /// # Panics
    /// Panics if password hashing fails (extremely rare with valid argon2 configuration)
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "secure_test_password";
        let hash = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        // Correct password should verify
        assert!(user.verify_password(password));

        // Wrong password should not verify
        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "same_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: hash1,
        };
        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: hash2,
        };

        assert!(user1.verify_password(password));
        assert!(user2.verify_password(password));
    }

    #[test]
    fn test_user_serialization_skips_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "test_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize user");

        // Password hash should not appear in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("test_hash_value"));

        // Other fields should be present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_verify_password_with_invalid_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false for invalid hash, not panic
        assert!(!user.verify_password("any_password"));
    }
}
