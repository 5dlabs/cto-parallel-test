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
    /// Verifies a password against the stored hash.
    /// Returns true if the password matches, false otherwise.
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

    /// Hashes a password using Argon2 with a random salt.
    /// Returns the encoded hash string.
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }
}
