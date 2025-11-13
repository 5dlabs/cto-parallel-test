use crate::auth::User;
use crate::auth::jwt::{AuthError, create_token, validate_token};
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn test_password_hashing_and_verification() {
    let password = "test_password_123";
    let hash1 = User::hash_password(password).expect("hash1");
    let hash2 = User::hash_password(password).expect("hash2");

    // Hashes should be different (due to random salt)
    assert_ne!(hash1, hash2);

    // Both should verify correctly
    let user1 = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash1,
    };

    assert!(user1.verify_password(password));
    assert!(!user1.verify_password("wrong_password"));
}

#[test]
fn test_jwt_creation_and_validation() {
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    unsafe { std::env::set_var("JWT_SECRET", "integration_secret_for_tests_only_change_me") };
    // Use default 24h TTL if JWT_EXP_SECONDS is not set
    let user_id = "123";
    let token = create_token(user_id).expect("token created");

    let claims = validate_token(&token).expect("valid token");
    assert_eq!(claims.sub, user_id);
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
    assert!(claims.exp >= claims.iat);
}

#[test]
fn test_invalid_token_is_rejected() {
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    unsafe { std::env::set_var("JWT_SECRET", "integration_secret_for_tests_only_change_me") };
    let invalid_token = "invalid.token.here";
    assert!(validate_token(invalid_token).is_err());
}

#[test]
fn test_rejects_too_short_secret() {
    // Too short: < 32 bytes
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    unsafe { std::env::set_var("JWT_SECRET", "short_secret_123") };
    let err = create_token("uid").expect_err("must reject weak secret");
    assert!(matches!(err, AuthError::WeakSecret));
}
