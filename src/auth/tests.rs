use crate::auth::User;
use crate::auth::jwt::{AuthError, create_token, validate_token};
use rand::RngCore;
use rand::rngs::OsRng;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn gen_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    OsRng.fill_bytes(&mut buf);
    let mut s = String::with_capacity(bytes * 2);
    for b in buf {
        use std::fmt::Write as _;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

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
    // Generate a fresh, random secret for this test (>= 32 bytes when hex-encoded)
    // SAFETY: Setting process env vars is not thread-safe; guard with ENV_LOCK
    unsafe {
        std::env::set_var("JWT_SECRET", gen_hex(48));
    }
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
    // SAFETY: See note above regarding ENV_LOCK
    unsafe {
        std::env::set_var("JWT_SECRET", gen_hex(48));
    }
    let invalid_token = "invalid.token.here";
    assert!(validate_token(invalid_token).is_err());
}

#[test]
fn test_rejects_too_short_secret() {
    // Too short: < 32 bytes
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    // Generate a short random secret (< 32 hex chars)
    // SAFETY: See note above regarding ENV_LOCK
    unsafe {
        std::env::set_var("JWT_SECRET", gen_hex(8));
    }
    let err = create_token("uid").expect_err("must reject weak secret");
    assert!(matches!(err, AuthError::WeakSecret));
}
