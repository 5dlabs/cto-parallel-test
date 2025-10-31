//! Product catalog module
//!
//! Provides product management, inventory tracking, and filtering capabilities.

pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
