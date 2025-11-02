//! Product catalog module
//!
//! Provides product management with thread-safe in-memory storage,
//! CRUD operations, inventory tracking, and filtering capabilities.

pub mod models;
pub mod service;

pub use self::models::{NewProduct, Product, ProductFilter};
pub use self::service::ProductService;
