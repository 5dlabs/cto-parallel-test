use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

/// User model with authentication fields
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
    /// # Arguments
    /// * `password` - The plain text password to hash
    ///
    /// # Returns
    /// The hashed password string
    ///
    /// # Panics
    /// Panics if bcrypt hashing fails
    #[must_use]
    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).expect("Failed to hash password")
    }

    /// Verifies a password against the stored hash
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// true if password matches, false otherwise
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    /// Creates a new user with hashed password
    ///
    /// # Arguments
    /// * `id` - User ID
    /// * `username` - Username
    /// * `email` - Email address
    /// * `password` - Plain text password (will be hashed)
    ///
    /// # Returns
    /// New User instance with hashed password
    #[must_use]
    pub fn new(id: i32, username: String, email: String, password: &str) -> Self {
        Self {
            id,
            username,
            email,
            password_hash: Self::hash_password(password),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "TEST_SECURE_PASSWORD_123";
        let hashed = User::hash_password(password);

        // Hash should not equal plain password
        assert_ne!(hashed, password);

        // Create user with hashed password
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };

        // Correct password should verify
        assert!(user.verify_password(password));

        // Wrong password should fail
        assert!(!user.verify_password("WRONG_PASSWORD"));
    }

    #[test]
    fn test_user_new() {
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            "TEST_PASSWORD123",
        );

        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.verify_password("TEST_PASSWORD123"));
        assert!(!user.verify_password("WRONGPASSWORD"));
    }

    #[test]
    fn test_serialization_skips_password() {
        let user = User::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            "TEST_PASSWORD123_FOR_SERIALIZATION",
        );

        let json = serde_json::to_string(&user).expect("Failed to serialize");

        // Password hash should not be in JSON
        assert!(!json.contains("password_hash"));
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }
}
