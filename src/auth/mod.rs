//! Authentication module providing JWT token validation.
//!
//! This module provides JWT token creation and validation functionality
//! required for securing API endpoints.

pub mod jwt;

pub use self::jwt::{create_token, validate_token, Claims};
