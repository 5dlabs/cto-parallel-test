//! Authentication Module
//!
//! This module provides JWT-based authentication and secure password hashing
//! for user authentication in the e-commerce platform.

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
