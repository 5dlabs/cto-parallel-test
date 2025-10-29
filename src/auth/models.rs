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
    /// Verifies a plain text password against the stored hash
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

    /// Hashes a password using Argon2 with a random salt
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely unlikely with valid Argon2 configuration)
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

        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);

        // But both hashes should verify the same password
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

    #[test]
    fn test_empty_password_handling() {
        let password = "";
        let hashed = User::hash_password(password);
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };
        assert!(user.verify_password(password));
        assert!(!user.verify_password("not_empty"));
    }
}
