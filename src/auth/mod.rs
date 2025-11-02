//! User Authentication Module
//!
//! This module provides JWT-based authentication functionality including:
//! - JWT token creation and validation
//! - Token claims management
//! - Authentication utilities

pub mod jwt;

pub use self::jwt::{create_token, validate_token, Claims};
