//! Authentication module for user authentication and JWT token management
//!
//! This module provides:
//! - JWT token creation and validation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - User models with secure password verification
//! - Request/Response DTOs for authentication endpoints

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
