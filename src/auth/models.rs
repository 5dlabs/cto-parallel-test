use argon2::{PasswordHasher, PasswordVerifier};
use rand::{RngCore, rngs::OsRng};
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
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Argon2 0.5 API: parse PHC string and verify
        let Ok(parsed) = argon2::password_hash::PasswordHash::new(&self.password_hash) else {
            return false;
        };
        argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    #[must_use]
    pub fn hash_password(password: &str) -> String {
        let mut salt = [0u8; 32];
        // Use OS RNG for cryptographically secure salt generation
        OsRng.fill_bytes(&mut salt);
        let Ok(salt_str) = argon2::password_hash::SaltString::encode_b64(&salt) else {
            return String::new();
        };
        match argon2::Argon2::default().hash_password(password.as_bytes(), &salt_str) {
            Ok(ph) => ph.to_string(),
            Err(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_and_verify() {
        let password = "s3cr3t";
        let hash = User::hash_password(password);
        assert!(!hash.is_empty());

        let user = User {
            id: 1,
            username: "alice".into(),
            email: "a@example.com".into(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("not-it"));
    }

    #[test]
    fn user_serialization_skips_password() {
        let user = User {
            id: 1,
            username: "bob".into(),
            email: "b@example.com".into(),
            password_hash: "hash".into(),
        };
        let json = serde_json::to_string(&user).expect("json");
        assert!(!json.contains("password_hash"));
        assert!(json.contains("username"));
    }
}
