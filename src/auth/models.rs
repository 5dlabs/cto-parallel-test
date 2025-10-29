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
    /// Verifies if the provided password matches the stored password hash.
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
    /// let hash = User::hash_password("mypassword");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    /// assert!(user.verify_password("mypassword"));
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
    /// Panics if Argon2 hashing fails, which should never occur in practice
    /// with valid inputs. This is acceptable for a test authentication module.
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::User;
    /// let hash = User::hash_password("secure_password");
    /// assert!(!hash.is_empty());
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

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);

        // But both should verify successfully
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
        assert!(json.contains("email"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_verify_password_returns_false_on_invalid_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash".to_string(),
        };
        // Should return false instead of panicking
        assert!(!user.verify_password("any_password"));
    }
}
