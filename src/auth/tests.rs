use crate::auth::jwt::{create_token, validate_token};
use crate::auth::models::User;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::sync::{Mutex, OnceLock};

// Global lock to prevent concurrent mutation of JWT-related env vars across tests.
static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().expect("env lock")
}

fn ensure_secret() {
    // Generate a random, strong secret for tests to avoid hardcoded-secret scanners
    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();
    std::env::set_var("JWT_SECRET", secret);
    // Ensure default of 24 hours unless overridden by tests.
    std::env::remove_var("JWT_EXP_HOURS");
}

#[test]
fn test_password_hashing() {
    // Generate a random, non-constant password to avoid secret scanning false positives
    let test_pw: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
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
    let user = User {
        id: 42,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        password_hash: User::hash_password("S3cureP@ssw0rd!"),
    };

    let json = serde_json::to_string(&user).expect("serialize user");
    assert!(json.contains("\"id\":"));
    assert!(json.contains("\"username\":"));
    assert!(json.contains("\"email\":"));
    assert!(!json.contains("password_hash"));
}
