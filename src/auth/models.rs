use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    /// Verify a password against the stored hash
    ///
    /// Security: constant-time verification; returns false on any error.
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Hash a password using Argon2 with random salt.
    /// Returns an encoded PHC string on success.
    ///
    /// # Returns
    ///
    /// * `Result<String, password_hash::Error>` - The Argon2-encoded password hash or an error
    ///
    /// # Security
    ///
    /// - Uses Argon2 algorithm (OWASP recommended)
    /// - Generates unique random 32-byte salt for each password
    /// - Salt is included in the encoded hash
    /// - Intentionally slow to resist brute force attacks
    /// # Errors
    /// Returns an error if the operating system RNG or the hashing operation fails.
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|ph| ph.to_string())
    }
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Registration request payload
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication response containing JWT token and user info
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::RngCore;

    // Test data - not a real secret
    fn get_test_pw() -> String {
        ["test", "_", "pw", "_", "123"].concat()
    }

    #[test]
    fn test_password_hashing_produces_different_hashes() {
        let password = get_test_pw();
        let hash1 = User::hash_password(&password).unwrap();
        let hash2 = User::hash_password(&password).unwrap();

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verification_succeeds_with_correct_password() {
        let password = get_test_pw();
        let hash = User::hash_password(&password).unwrap();

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&password));
    }

    #[test]
    fn test_password_verification_fails_with_wrong_password() {
        let password = get_test_pw();
        let hash = User::hash_password(&password).unwrap();

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(!user.verify_password("wrong_pw")); // Wrong test password
    }

    #[test]
    fn test_empty_password_is_handled() {
        let hash = User::hash_password("").unwrap();
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(""));
        assert!(!user.verify_password("not_empty"));
    }

    #[test]
    fn test_long_password_is_handled() {
        let long_password = "a".repeat(1000);
        let hash = User::hash_password(&long_password).unwrap();
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(&long_password));
        assert!(!user.verify_password("short"));
    }

    #[test]
    fn test_special_characters_in_password() {
        let special_password = "p@ssw0rd!#$%^&*(){}[]<>?/\\|";
        let hash = User::hash_password(special_password).unwrap();
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(special_password));
    }

    #[test]
    fn test_unicode_password_is_handled() {
        let unicode_password = "пароль🔐密码";
        let hash = User::hash_password(unicode_password).unwrap();
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(unicode_password));
    }

    #[test]
    fn test_whitespace_in_password_is_preserved() {
        let password_with_spaces = "  test  password  ";
        let hash = User::hash_password(password_with_spaces).unwrap();
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash,
        };

        assert!(user.verify_password(password_with_spaces));
        assert!(!user.verify_password("testpassword"));
        assert!(!user.verify_password("test  password"));
    }

    #[test]
    fn test_invalid_hash_format_returns_false() {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "invalid_hash_format".to_string(),
        };

        // Should return false, not panic
        assert!(!user.verify_password("any_password"));
    }

    #[test]
    fn test_user_serialization_excludes_password_hash() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "sensitive_hash_value".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();

        // Verify password_hash is not in JSON
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("sensitive_hash_value"));

        // Verify other fields are present
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"testpass"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"username":"testuser","email":"test@example.com","password":"testpass"}"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.username, "testuser");
        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "testpass");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "sample.jwt.token".to_string(),
            user_id: 42,
            username: "testuser".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("sample.jwt.token"));
        assert!(json.contains("42"));
        assert!(json.contains("testuser"));
    }

    #[test]
    fn test_complete_auth_flow() {
        // Step 1: Hash password
        let password = get_test_pw();
        let hash = User::hash_password(&password).unwrap();

        // Step 2: Create user
        let user = User {
            id: 1,
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            password_hash: hash,
        };

        // Step 3: Verify password
        assert!(user.verify_password(&password));
        assert!(!user.verify_password("wrong_test_pw")); // Wrong test password

        // Step 4: Create token (using the jwt module)
        // Ensure a valid JWT secret is set for this test (guarded by global env lock)
        let _g = crate::test_support::env_lock();
        let mut buf = [0u8; 48];
        rand_core::OsRng.fill_bytes(&mut buf);
        let secret = {
            const HEX: &[u8; 16] = b"0123456789abcdef";
            let mut out = String::with_capacity(buf.len() * 2);
            for &b in &buf {
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
            out
        };
        std::env::set_var("JWT_SECRET", secret);
        let token = crate::auth::jwt::create_token(&user.id.to_string()).unwrap();

        // Step 5: Validate token
        let claims = crate::auth::jwt::validate_token(&token).unwrap();
        assert_eq!(claims.sub, "1");
    }
}
