use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password_hash", &"<redacted>")
            .finish()
    }
}

impl User {
    /// Verify a password against the stored hash
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        match PasswordHash::new(&self.password_hash) {
            Ok(parsed) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
            Err(_) => false,
        }
    }

    /// Hash a password using Argon2 with random salt
    ///
    /// # Panics
    /// Panics if the Argon2 hashing operation fails.
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        // Allow runtime tuning via environment while providing secure defaults.
        // Defaults: t=3, m=64 MiB, p=1. Enforce sane bounds to avoid denial of service.
        let mem_default_kib: u32 = 64 * 1024; // 64 MiB
        let mem_min_kib: u32 = 8 * 1024; // 8 MiB
        let mem_max_kib: u32 = 1024 * 1024; // 1 GiB
        let t_default: u32 = 3; // iterations
        let t_min: u32 = 1;
        let t_max: u32 = 10;
        let p_default: u32 = 1; // lanes/parallelism
        let p_min: u32 = 1;
        let p_max: u32 = 8;

        let m_cost_kib = std::env::var("ARGON2_M_COST_KIB")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .map_or(mem_default_kib, |v| v.clamp(mem_min_kib, mem_max_kib));

        let t_cost = std::env::var("ARGON2_T_COST")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .map_or(t_default, |v| v.clamp(t_min, t_max));

        let p_cost = std::env::var("ARGON2_P_COST")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .map_or(p_default, |v| v.clamp(p_min, p_max));

        let params =
            Params::new(m_cost_kib, t_cost, p_cost, None).expect("argon2 params should be valid");
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map_or_else(
                |err| panic!("Failed to hash password: {err}"),
                |hash| hash.to_string(),
            )
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}
