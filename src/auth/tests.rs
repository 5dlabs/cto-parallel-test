use super::*;
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
