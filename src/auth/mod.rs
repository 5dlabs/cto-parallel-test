//! Authentication module providing JWT token functionality
//!
//! This module provides JWT-based authentication for the shopping cart API.
//! It includes token creation and validation functions.

pub mod jwt;

pub use self::jwt::{create_token, validate_token, Claims};
