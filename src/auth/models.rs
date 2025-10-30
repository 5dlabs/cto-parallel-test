use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
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
    /// Hashes a plain text password using Argon2 with a random salt
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// * `String` - The encoded password hash
    ///
    /// # Panics
    /// Panics if password hashing fails (extremely rare)
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }

    /// Verifies a plain text password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `bool` - true if the password matches, false otherwise
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.password_hash);
        if let Ok(hash) = parsed_hash {
            Argon2::default()
                .verify_password(password.as_bytes(), &hash)
                .is_ok()
        } else {
            false
        }
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
    fn test_different_hashes_for_same_password() {
        let password = "test_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Due to random salt, same password should produce different hashes
        assert_ne!(hash1, hash2);

        // Both should verify correctly
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
        assert!(json.contains("testuser"));
        assert!(json.contains("email"));
    }

    #[test]
    fn test_verify_password_with_wrong_password() {
        let correct_password = "correct_password";
        let wrong_password = "wrong_password";

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password(correct_password),
        };

        assert!(user.verify_password(correct_password));
        assert!(!user.verify_password(wrong_password));
        assert!(!user.verify_password(""));
    }
}
