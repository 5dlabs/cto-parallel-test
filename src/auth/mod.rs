//! Authentication module with JWT token handling
//!
//! This module provides JWT-based authentication including:
//! - Token creation and validation
//! - User claims extraction
//! - Middleware for protected routes

pub mod jwt;

pub use jwt::{create_token, validate_token, Claims};
