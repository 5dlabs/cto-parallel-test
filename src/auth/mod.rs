//! User authentication module
//!
//! Provides JWT token-based authentication and secure password hashing.
//! Uses industry-standard Argon2 for password hashing and JWT for stateless authentication.

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
