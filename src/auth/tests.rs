use crate::auth::jwt::{create_token, validate_token, AuthError};
use crate::auth::User;
use rand::rngs::OsRng;
use rand::RngCore;
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

#[test]
fn test_expired_token_is_rejected() {
    use crate::auth::jwt::{create_token_with, Clock};

    struct FixedClock(u64);
    impl Clock for FixedClock {
        fn now_seconds(&self) -> u64 {
            self.0
        }
    }

    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    // SAFETY: See note above regarding ENV_LOCK
    unsafe {
        // Set a strong random secret so validation uses the same key
        std::env::set_var("JWT_SECRET", gen_hex(48));
    }

    // Create a token with a timestamp far in the past so it is definitely expired
    let clock = FixedClock(1); // iat = 1
    let secret = std::env::var("JWT_SECRET").unwrap();
    let token = create_token_with(
        "expired_user",
        &clock,
        secret.as_bytes(),
        1, /* 1s TTL */
    )
    .expect("token created");

    // Validating against current system time should reject this token as expired
    let res = validate_token(&token);
    assert!(res.is_err(), "expired token must be rejected");
}
