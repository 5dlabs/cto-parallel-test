//! # Authentication Module
//!
//! This module provides secure user authentication functionality including:
//! - JWT token creation and validation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - User model with password verification
//! - Authentication request/response DTOs

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::User;
