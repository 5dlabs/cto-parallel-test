use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User model representing an authenticated user.
///
/// The `password_hash` field is never serialized in JSON responses
/// to prevent accidental exposure of password hashes.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a plain-text password against the stored password hash.
    ///
    /// Uses Argon2 with constant-time comparison to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plain-text password to verify
    ///
    /// # Returns
    ///
    /// Returns `true` if the password matches, `false` otherwise
    ///
    /// # Examples
    ///
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
    ///
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

    /// Hashes a plain-text password using Argon2.
    ///
    /// Generates a random 32-byte salt for each password.
    /// Uses Argon2 default configuration (suitable for most applications).
    ///
    /// # Arguments
    ///
    /// * `password` - The plain-text password to hash
    ///
    /// # Returns
    ///
    /// Returns the Argon2-encoded hash string (includes algorithm, parameters, salt, and hash)
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare, only if system resources are exhausted).
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::User;
    ///
    /// let hash1 = User::hash_password("mypassword");
    /// let hash2 = User::hash_password("mypassword");
    ///
    /// // Each hash is different due to random salt
    /// assert_ne!(hash1, hash2);
    ///
    /// // But both verify correctly
    /// let user1 = User {
    ///     id: 1,
    ///     username: "test".to_string(),
    ///     email: "test@test.com".to_string(),
    ///     password_hash: hash1,
    /// };
    /// assert!(user1.verify_password("mypassword"));
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
    fn test_each_hash_is_unique() {
        let password = "same_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Each hash should be different due to random salt
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

    #[test]
    fn test_verify_returns_false_for_invalid_hash() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false instead of panicking
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_wrong_password_returns_false() {
        let password = "correct_password";
        let hashed = User::hash_password(password);
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };

        assert!(!user.verify_password("wrong_password"));
        assert!(!user.verify_password(""));
        assert!(!user.verify_password("correct_passwor")); // Missing one character
    }
}
