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
    /// Verifies a plain password against the stored password hash
    ///
    /// # Arguments
    /// * `password` - The plain password to verify
    ///
    /// # Returns
    /// * `bool` - True if the password matches, false otherwise
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

    /// Hashes a plain password using Argon2 with a random salt
    ///
    /// # Arguments
    /// * `password` - The plain password to hash
    ///
    /// # Returns
    /// * `String` - The encoded password hash
    ///
    /// # Panics
    /// Panics if password hashing fails (should never happen with valid input)
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
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
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes due to random salts"
        );
    }
}
