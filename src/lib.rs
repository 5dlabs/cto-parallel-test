//! E-Commerce Application Library
//!
//! This library provides the core functionality for an e-commerce application,
//! including authentication, database schema, models, and business logic.

pub mod auth;

// Re-export commonly used authentication types
pub use auth::{create_token, validate_token, Claims, User};
