//! User model and authentication data structures.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

/// User entity with authentication information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Password hash (never serialized to JSON)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// `true` if the password matches, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let pwd = "example123";
    /// let hash = User::hash_password(pwd);
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "test".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password(pwd));
    /// assert!(!user.verify_password("wrong"));
    /// ```
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }

    /// Hashes a password using Argon2 with a random salt.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// The Argon2 password hash string
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails (extremely rare)
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let hash1 = User::hash_password("example123");
    /// let hash2 = User::hash_password("example123");
    ///
    /// // Hashes are different due to random salt
    /// assert_ne!(hash1, hash2);
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

/// Login request DTO.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// User credentials for authentication
    pub password: String,
}

/// Registration request DTO.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// User email address
    pub email: String,
    /// User credentials
    pub password: String,
}

/// Authentication response DTO.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for authentication
    pub token: String,
    /// User ID
    pub user_id: i32,
    /// Username
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let pwd = "example123";
        let hash1 = User::hash_password(pwd);
        let hash2 = User::hash_password(pwd);

        // Hashes should be different (due to random salt)
        assert_ne!(hash1, hash2);

        // Both should verify correctly
        let user1 = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash1,
        };

        assert!(user1.verify_password(pwd));
        assert!(!user1.verify_password("wrong"));
    }

    #[test]
    fn test_password_verification_failure() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password("correct123"),
        };

        assert!(!user.verify_password("wrong456"));
    }

    #[test]
    fn test_password_hash_not_serialized() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("secret_hash"));
        assert!(!json.contains("password_hash"));
    }

    #[test]
    fn test_unique_salts() {
        let pwd = "same123";
        let hash1 = User::hash_password(pwd);
        let hash2 = User::hash_password(pwd);
        let hash3 = User::hash_password(pwd);

        assert_ne!(hash1, hash2);
        assert_ne!(hash2, hash3);
        assert_ne!(hash1, hash3);
    }
}
