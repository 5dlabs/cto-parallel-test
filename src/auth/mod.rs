//! Authentication Module
//!
//! This module provides JWT-based authentication and password hashing functionality
//! for the e-commerce application.
//!
//! # Features
//! - JWT token creation and validation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - User model with password verification
//! - Request/Response DTOs for authentication endpoints

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
