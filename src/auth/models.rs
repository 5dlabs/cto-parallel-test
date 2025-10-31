use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use rand::rngs::OsRng;
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
    /// Uses Argon2 with constant-time comparison to prevent timing attacks.
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `bool` - true if the password matches, false otherwise
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let password = "my_password";
    /// let hash = User::hash_password(password);
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
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

    /// Hashes a plain text password using Argon2 with a random salt.
    ///
    /// Each call generates a unique hash even for the same password due to random salt.
    ///
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// * `String` - The encoded Argon2 hash
    ///
    /// # Panics
    /// Panics if the password hashing operation fails.
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("my_password");
    /// let hash2 = User::hash_password("my_password");
    ///
    /// // Each hash is unique due to random salt
    /// assert_ne!(hash1, hash2);
    /// ```
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }
}
