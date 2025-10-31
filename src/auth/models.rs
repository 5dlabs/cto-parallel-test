use argon2::{PasswordHasher, PasswordVerifier};
use rand::RngCore;
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
    pub fn verify_password(&self, password: &str) -> bool {
        // Argon2 0.5 API: parse PHC string and verify
        let parsed = match argon2::password_hash::PasswordHash::new(&self.password_hash) {
            Ok(p) => p,
            Err(_) => return false,
        };
        argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    pub fn hash_password(password: &str) -> String {
        let mut salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        let salt_str = match argon2::password_hash::SaltString::encode_b64(&salt) {
            Ok(s) => s,
            Err(_) => return String::new(),
        };
        match argon2::Argon2::default().hash_password(password.as_bytes(), &salt_str) {
            Ok(ph) => ph.to_string(),
            Err(_) => String::new(),
        }
    }
}
