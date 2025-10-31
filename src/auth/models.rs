use argon2::{self, Config};
use rand::Rng;
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
    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `bool` - True if the password matches, false otherwise
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        argon2::verify_encoded(&self.password_hash, password.as_bytes()).unwrap_or(false)
    }

    /// Hashes a password using Argon2
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// * `String` - The hashed password
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (should not happen with valid inputs)
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
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
    fn test_different_hashes_for_same_password() {
        let password = "test_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        let user1 = User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@test.com".to_string(),
            password_hash: hash1,
        };
        let user2 = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@test.com".to_string(),
            password_hash: hash2,
        };

        assert!(user1.verify_password(password));
        assert!(user2.verify_password(password));
    }
}
