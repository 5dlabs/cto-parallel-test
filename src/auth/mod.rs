//! Authentication module providing JWT token handling and password hashing
//!
//! This module provides secure authentication functionality including:
//! - JWT token creation and validation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - User model with password verification
//! - Request/Response DTOs for authentication endpoints

pub mod jwt;
pub mod models;

// Re-export commonly used types for convenience
pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
