//! Authentication module providing JWT token handling and password hashing.
//!
//! This module provides:
//! - JWT token creation and validation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - User model with password verification
//! - Authentication DTOs for requests and responses

pub mod jwt;
pub mod models;

#[cfg(test)]
mod tests;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
