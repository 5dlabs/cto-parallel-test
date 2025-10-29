//! Product catalog module
//!
//! Provides product management functionality including models and service logic.

pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
