//! User authentication module
//!
//! Provides JWT token creation and validation for secure API authentication.

pub mod jwt;

pub use jwt::{create_token, validate_token, Claims};
