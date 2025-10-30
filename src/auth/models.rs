use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user.
/// The `password_hash` field is never serialized to JSON for security.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a plain password against the stored password hash.
    /// Uses Argon2 verification which is resistant to timing attacks.
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `bool` - true if the password matches, false otherwise
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash = User::hash_password("mypassword");
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    /// assert!(user.verify_password("mypassword"));
    /// assert!(!user.verify_password("wrongpassword"));
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

    /// Hashes a plain password using Argon2 with a random salt.
    /// Each call generates a unique hash even for the same password.
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// * `String` - The Argon2 encoded hash string
    ///
    /// # Panics
    /// Panics if the Argon2 hashing algorithm fails (should never happen with valid inputs).
    ///
    /// # Example
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("password123");
    /// let hash2 = User::hash_password("password123");
    /// // Hashes are different due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
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
    fn test_different_hashes_for_same_password() {
        let password = "test123";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);
        // Each hash should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_wrong_password_returns_false() {
        let hash = User::hash_password("correct_password");
        let user = User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password_hash: hash,
        };
        assert!(!user.verify_password("wrong_password"));
    }
}
