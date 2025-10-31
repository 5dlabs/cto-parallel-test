//! Product catalog module
//!
//! Provides product models and service for managing the product catalog.

pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
