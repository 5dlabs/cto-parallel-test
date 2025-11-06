//! User authentication models
//!
//! Provides data structures for user authentication, including:
//! - User model with secure password hashing
//! - Request/Response DTOs for authentication endpoints

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// User entity with authentication credentials
///
/// Represents a user in the system with secure password storage.
/// The password hash is never serialized to JSON for security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username for login
    pub username: String,
    /// User email address
    pub email: String,
    /// Argon2 password hash (never serialized)
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// Uses Argon2 password verification with constant-time comparison
    /// to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// `true` if password matches, `false` otherwise.
    /// Returns `false` on any error (invalid hash format, etc.)
    ///
    /// # Security
    ///
    /// - Uses constant-time comparison (Argon2 handles this internally)
    /// - Never panics on verification failure
    /// - Returns `false` for any error to avoid information leakage
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let credential = "example-credential";
    /// let hash = User::hash_password(credential);
    ///
    /// let user = User {
    ///     id: 1,
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password_hash: hash,
    /// };
    ///
    /// assert!(user.verify_password(credential));
    /// assert!(!user.verify_password("incorrect-credential"));
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

    /// Hash a password using Argon2 with random salt
    ///
    /// Creates a secure password hash using:
    /// - Argon2id algorithm (recommended by OWASP)
    /// - Cryptographically secure random salt (32 bytes)
    /// - Default Argon2 parameters (appropriate for authentication)
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    ///
    /// A PHC-formatted Argon2 hash string that includes:
    /// - Algorithm identifier
    /// - Parameters (memory, iterations, parallelism)
    /// - Salt (base64 encoded)
    /// - Hash (base64 encoded)
    ///
    /// # Security
    ///
    /// - Uses `OsRng` for cryptographically secure random salt
    /// - Each password gets a unique random salt
    /// - Argon2id provides resistance against side-channel attacks
    /// - Default parameters balance security and performance
    ///
    /// # Panics
    ///
    /// Panics if password hashing fails, which should only occur in extreme
    /// circumstances such as memory allocation failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::auth::models::User;
    ///
    /// let credential = "example-credential";
    /// let hash1 = User::hash_password(credential);
    /// let hash2 = User::hash_password(credential);
    ///
    /// // Same password produces different hashes due to random salt
    /// assert_ne!(hash1, hash2);
    ///
    /// // But both verify correctly
    /// let user1 = User {
    ///     id: 1,
    ///     username: "user1".to_string(),
    ///     email: "user1@example.com".to_string(),
    ///     password_hash: hash1,
    /// };
    /// assert!(user1.verify_password(credential));
    /// ```
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let mut salt_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut salt_bytes);

        let salt = SaltString::encode_b64(&salt_bytes).expect("Failed to encode salt");
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash credential")
            .to_string()
    }
}

/// Request body for user login
///
/// Contains credentials required for authentication.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Plaintext password (will be verified against stored hash)
    pub password: String,
}

/// Request body for user registration
///
/// Contains all information needed to create a new user account.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password (will be hashed before storage)
    pub password: String,
}

/// Response body for successful authentication
///
/// Contains JWT token and user information after successful login or registration.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for subsequent authenticated requests
    pub token: String,
    /// User ID
    pub user_id: i32,
    /// Username
    pub username: String,
}

#[cfg(test)]
mod tests;
