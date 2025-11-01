//! User authentication module
//!
//! This module provides JWT-based authentication and secure password handling using:
//! - JWT tokens with 24-hour expiration for stateless authentication
//! - Argon2 password hashing with random salt for secure password storage
//! - User models and DTOs for authentication flows

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
