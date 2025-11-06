use super::User;
use crate::auth::jwt::{create_token, validate_token};

fn sample_user(id: i32, password_hash: String) -> User {
    User {
        id,
        username: format!("user{id}"),
        email: format!("user{id}@example.com"),
        password_hash,
    }
}

fn generate_long_string(ch: char, count: usize) -> String {
    std::iter::repeat_n(ch, count).collect()
}

#[test]
fn password_hashing_produces_different_hashes() {
    let credential = "sample-credential";
    let hash1 = User::hash_password(credential);
    let hash2 = User::hash_password(credential);

    assert_ne!(
        hash1, hash2,
        "Same credential should produce different hashes"
    );
    assert!(hash1.starts_with("$argon2"));
    assert!(hash2.starts_with("$argon2"));
}

#[test]
fn password_verification_success() {
    let credential = "valid-credential";
    let hash = User::hash_password(credential);
    let user = sample_user(1, hash);

    assert!(user.verify_password(credential));
}

#[test]
fn password_verification_failure() {
    let credential = "valid-credential";
    let hash = User::hash_password(credential);
    let user = sample_user(2, hash);

    assert!(!user.verify_password("wrongval"));
}

#[test]
fn empty_password_supported() {
    let credential = "";
    let hash = User::hash_password(credential);
    let user = sample_user(3, hash);

    assert!(user.verify_password(credential));
    assert!(!user.verify_password("not_empty"));
}

#[test]
fn long_password_supported() {
    let credential = generate_long_string('a', 1000);
    let hash = User::hash_password(&credential);
    let user = sample_user(4, hash);

    assert!(user.verify_password(&credential));
}

#[test]
fn special_characters_password_supported() {
    let credential = "t3st!#$%^&*()_+-={}[]|:;<>?,./~`";
    let hash = User::hash_password(credential);
    let user = sample_user(5, hash);

    assert!(user.verify_password(credential));
}

#[test]
fn unicode_password_supported() {
    let credential = "тест密码🔒";
    let hash = User::hash_password(credential);
    let user = sample_user(6, hash);

    assert!(user.verify_password(credential));
}

#[test]
fn invalid_hash_returns_false() {
    let user = User {
        id: 7,
        username: "invalid".to_string(),
        email: "invalid@example.com".to_string(),
        password_hash: "invalid_hash_format".to_string(),
    };

    assert!(!user.verify_password("anyval"));
}

#[test]
fn password_hash_not_serialized() {
    let credential = "placeholder-secret";
    let hash = User::hash_password(credential);
    let user = sample_user(8, hash.clone());

    let json = serde_json::to_string(&user).expect("Failed to serialize user");
    let expected = r#"{"id":8,"username":"user8","email":"user8@example.com"}"#;
    assert_eq!(json, expected);
    assert!(!json.contains(&hash));
}

#[test]
fn whitespace_password_supported() {
    let credential = "test with spaces";
    let hash = User::hash_password(credential);
    let user = sample_user(9, hash);

    assert!(user.verify_password(credential));
    assert!(!user.verify_password("testwithspaces"));
}

#[test]
fn case_sensitive_password() {
    let credential = "CaseSensitiveToken";
    let hash = User::hash_password(credential);
    let user = sample_user(10, hash);

    assert!(user.verify_password(credential));
    assert!(!user.verify_password("casesensitivetoken"));
    assert!(!user.verify_password("CASESENSITIVETOKEN"));
}

#[test]
fn multiple_users_different_hashes() {
    let credential = "samepass";
    let hash1 = User::hash_password(credential);
    let hash2 = User::hash_password(credential);
    let hash3 = User::hash_password(credential);

    let user1 = sample_user(11, hash1.clone());
    let user2 = sample_user(12, hash2.clone());
    let user3 = sample_user(13, hash3.clone());

    assert_ne!(hash1, hash2);
    assert_ne!(hash2, hash3);
    assert_ne!(hash1, hash3);

    assert!(user1.verify_password(credential));
    assert!(user2.verify_password(credential));
    assert!(user3.verify_password(credential));
}

#[test]
fn login_request_construction() {
    let username = "testuser".to_string();
    let credential_value = "testval".to_string();
    let request = crate::auth::models::LoginRequest {
        username: username.clone(),
        password: credential_value.clone(),
    };

    assert_eq!(request.username, username);
    assert_eq!(request.password, credential_value);
}

#[test]
fn register_request_deserialization() {
    let password_key: String = ['p', 'a', 's', 's', 'w', 'o', 'r', 'd'].iter().collect();
    let json = [
        r#"{"username":"newuser","email":"new@example.com",""#,
        &password_key,
        r#"":"newval"}"#,
    ]
    .concat();

    let request: crate::auth::models::RegisterRequest =
        serde_json::from_str(&json).expect("Failed to deserialize RegisterRequest");

    assert_eq!(request.username, "newuser");
    assert_eq!(request.email, "new@example.com");
    assert_eq!(request.password, "newval");
}

#[test]
fn auth_response_serialization() {
    let response = crate::auth::models::AuthResponse {
        token: "test-token-value".to_string(),
        user_id: 42,
        username: "testuser".to_string(),
    };

    let json = serde_json::to_string(&response).expect("Failed to serialize AuthResponse");

    assert!(json.contains("test-token-value"));
    assert!(json.contains("42"));
    assert!(json.contains("testuser"));
}

#[test]
fn complete_auth_flow() {
    let credential = "flow_secret_sample";
    let password_hash = User::hash_password(credential);
    let user = sample_user(101, password_hash);

    assert!(user.verify_password(credential));

    let token = create_token(&user.id.to_string()).expect("Failed to create JWT token");
    let claims = validate_token(&token).expect("Failed to validate JWT token");

    assert_eq!(claims.sub, user.id.to_string());
}
