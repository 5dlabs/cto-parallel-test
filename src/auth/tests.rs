use super::*;
use crate::auth::jwt::Claims;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serial_test::serial;

fn clear_auth_env() {
    std::env::remove_var("JWT_SECRET");
}

#[test]
#[serial]
fn test_password_hashing() {
    clear_auth_env();
    let password = "test_password_123";
    let hash1 = User::hash_password(password);
    let hash2 = User::hash_password(password);

    // Hashes should be different (due to random salt)
    assert_ne!(hash1, hash2);

    // Both should verify correctly
    let user = User {
        id: 1,
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash1,
    };

    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));
}

#[test]
#[serial]
fn test_password_hashing_supports_edge_cases() {
    clear_auth_env();

    let empty_hash = User::hash_password("");
    let empty_user = User {
        id: 2,
        username: "empty".to_string(),
        email: "empty@example.com".to_string(),
        password_hash: empty_hash.clone(),
    };
    assert!(empty_user.verify_password(""));

    let special_password = "p@ßw0rd🔥";
    let special_hash = User::hash_password(special_password);
    assert_ne!(empty_hash, special_hash);
    let special_user = User {
        id: 3,
        username: "special".to_string(),
        email: "special@example.com".to_string(),
        password_hash: special_hash,
    };
    assert!(special_user.verify_password(special_password));
    assert!(!special_user.verify_password("p@ßw0rd"));
}

#[test]
#[serial]
fn test_jwt_creation_and_validation_with_required_key() {
    clear_auth_env();
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");
    let user_id = "123";
    let token = crate::auth::jwt::create_token(user_id).expect("token");
    assert!(!token.is_empty());

    let claims = crate::auth::jwt::validate_token(&token).expect("validate");
    assert_eq!(claims.sub, user_id);

    // Check expiration and issued-at timestamps
    assert!(claims.exp > claims.iat);
    let ttl = claims.exp - claims.iat;
    assert_eq!(ttl, 86_400);
}

#[test]
#[serial]
fn test_jwt_creation_and_validation_with_custom_key() {
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");

    let token = crate::auth::jwt::create_token("user42").expect("token");
    let claims = crate::auth::jwt::validate_token(&token).expect("validate");
    assert_eq!(claims.sub, "user42");

    clear_auth_env();
}

#[test]
#[serial]
fn test_invalid_token() {
    clear_auth_env();
    let invalid_token = "invalid.token.here";
    assert!(crate::auth::jwt::validate_token(invalid_token).is_err());
}

#[test]
#[serial]
#[allow(clippy::disallowed_methods)]
fn test_expired_token_is_rejected() {
    clear_auth_env();
    std::env::set_var("JWT_SECRET", "dev_only_signing_key_min_32_chars________");

    // Manually craft an expired token using the same Claims type
    let now = usize::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
    .unwrap();
    let claims = Claims {
        sub: "expired_user".to_string(),
        iat: now.saturating_sub(120),
        exp: now.saturating_sub(60), // definitely in the past
        iss: None,
        aud: None,
    };
    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some("JWT".to_string());
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(b"dev_only_signing_key_min_32_chars________"),
    )
    .expect("encode expired token");

    // With validation enforcing exp (with small leeway), this must fail
    assert!(crate::auth::jwt::validate_token(&token).is_err());
}

#[test]
fn test_dto_serialization_and_deserialization() {
    // LoginRequest deserialization
    let login_json = r#"{ "username": "alice", "password": "s3cr3t" }"#;
    let login: crate::auth::models::LoginRequest = serde_json::from_str(login_json).unwrap();
    assert_eq!(login.username, "alice");
    assert_eq!(login.password, "s3cr3t");

    // RegisterRequest deserialization
    let reg_json = r#"{ "username": "bob", "email": "b@example.com", "password": "p@ss" }"#;
    let reg: crate::auth::models::RegisterRequest = serde_json::from_str(reg_json).unwrap();
    assert_eq!(reg.username, "bob");
    assert_eq!(reg.email, "b@example.com");
    assert_eq!(reg.password, "p@ss");

    // AuthResponse serialization
    let resp = crate::auth::models::AuthResponse {
        token: "token123".into(),
        user_id: 7,
        username: "charlie".into(),
    };
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("token123"));
    assert!(json.contains("\"user_id\":7"));
    assert!(json.contains("charlie"));
}
