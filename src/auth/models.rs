use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

/// Represents a user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `bool` - true if password is correct, false otherwise
    #[must_use]
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
}

/// Hashes a password using Argon2 with a random salt
///
/// # Arguments
/// * `password` - The plaintext password to hash
///
/// # Returns
/// * `Result<String, argon2::password_hash::Error>` - The hashed password or an error
///
/// # Errors
/// Returns an error if password hashing fails
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verifies a password against a hash
///
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The hash to verify against
///
/// # Returns
/// * `bool` - true if password matches hash, false otherwise
#[must_use]
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if let Ok(hash) = parsed_hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "secure_password";
        let hash = hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password);
    }

    #[test]
    fn test_verify_password() {
        let password = "test_password";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong_password", &hash));
    }

    #[test]
    fn test_user_verify_password() {
        let password = "user_password";
        let hash = hash_password(password).unwrap();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));
    }
}
