use serde::{Deserialize, Serialize};
use std::fmt;

use argon2::{
    password_hash::{
        Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use rand_core::OsRng;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // Never serialize or deserialize password hashes from untrusted input.
    // This avoids clients injecting arbitrary hashes.
    #[serde(skip_serializing, skip_deserializing)]
    pub password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password_hash", &"<redacted>")
            .finish()
    }
}

impl User {
    /// Verify a password against the stored hash
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        match PasswordHash::new(&self.password_hash) {
            Ok(parsed) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
            Err(_) => false,
        }
    }

    /// Hash a password using Argon2id with secure, opinionated parameters
    /// (Argon2 v0x13, t=3, m=64 MiB, p=1) and a random salt.
    ///
    /// # Errors
    /// Returns an error if the Argon2 hashing operation fails due to resource
    /// limitations or internal errors. This should not happen under normal
    /// operating conditions.
    pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);
        // Configure Argon2id with stronger defaults than the crate defaults
        // to align with current recommendations (OWASP, libsodium guidance).
        // m_cost is in KiB; 64 MiB = 65536 KiB.
        let params = Params::new(65_536, 3, 1, None).map_err(|_| PasswordHashError::Password)?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|ph| ph.to_string())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}
