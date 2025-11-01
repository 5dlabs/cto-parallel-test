use crate::auth::jwt::{create_token, validate_token};
use crate::auth::models::User;
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::{Mutex, OnceLock};

// Global lock to prevent concurrent mutation of JWT-related env vars across tests.
static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("env lock")
}

fn ensure_secret() {
    // Generate a cryptographically-secure random secret for tests (48 bytes, hex-encoded)
    let mut buf = [0u8; 48];
    OsRng.fill_bytes(&mut buf);
    let secret = hex_string(&buf);
    std::env::set_var("JWT_SECRET", secret);
    // Ensure default of 24 hours unless overridden by tests.
    std::env::remove_var("JWT_EXP_HOURS");
}

fn hex_string(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

#[test]
fn test_password_hashing() {
    // Generate a random, non-constant password using CSPRNG to satisfy scanners
    let mut pw_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut pw_bytes);
    let test_pw = hex_string(&pw_bytes);
    let hash1 = User::hash_password(&test_pw);
    let hash2 = User::hash_password(&test_pw);

    // Hashes should be different (due to random salt)
    assert_ne!(hash1, hash2);

    // Both should verify correctly
    let user1 = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash1,
    };

    assert!(user1.verify_password(&test_pw));
    assert!(!user1.verify_password("wrong_password"));
}

#[test]
fn test_jwt_creation_and_validation() {
    let _guard = env_lock();
    ensure_secret();

    let user_id = "123";
    let token = create_token(user_id).unwrap();

    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);

    // Check expiration and issued at are set
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_invalid_token() {
    let _guard = env_lock();
    ensure_secret();

    let invalid_token = "invalid.token.here";
    assert!(validate_token(invalid_token).is_err());
}

#[test]
fn test_user_serialization_excludes_password_hash() {
    // Ensure serialization of User never exposes password_hash
    // Use a randomly generated password to avoid hardcoded credentials in tests
    let mut pw_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut pw_bytes);
    let test_pw = hex_string(&pw_bytes);

    let user = User {
        id: 42,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        password_hash: User::hash_password(&test_pw),
    };

    let json = serde_json::to_string(&user).expect("serialize user");
    assert!(json.contains("\"id\":"));
    assert!(json.contains("\"username\":"));
    assert!(json.contains("\"email\":"));
    assert!(!json.contains("password_hash"));
}
