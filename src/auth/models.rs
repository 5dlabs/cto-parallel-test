use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model with password hashing capabilities.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Password hash (never serialized)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash.
    ///
    /// Returns `true` if the password matches, `false` otherwise.
    /// Uses constant-time comparison to prevent timing attacks.
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hashes a password using Argon2 with a random salt.
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails, which should not happen under normal circumstances.
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
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_with_wrong_password() {
        let password = "correct_password";
        let hashed = User::hash_password(password);
        let user = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: hashed,
        };
        assert!(!user.verify_password("incorrect_password"));
    }
}
