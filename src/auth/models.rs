use bcrypt::{hash, verify, DEFAULT_COST};
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
    /// Hashes a password using bcrypt
    ///
    /// # Panics
    ///
    /// Panics if bcrypt hashing fails (extremely rare, usually due to system errors)
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).expect("Failed to hash password")
    }

    /// Verifies a password against the stored hash
    ///
    /// # Panics
    ///
    /// Panics if bcrypt verification fails (usually due to invalid hash format)
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).expect("Failed to verify password")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "secure_password";
        let hash1 = User::hash_password(password);
        let hash2 = User::hash_password(password);

        // Different hashes due to random salt
        assert_ne!(hash1, hash2);

        // Both should verify correctly
        let user1 = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash1,
        };

        assert!(user1.verify_password(password));
        assert!(!user1.verify_password("wrong_password"));
    }

    #[test]
    fn test_user_serialization_skips_password() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: User::hash_password("password"),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");
        assert!(!json.contains("password_hash"));
    }
}
