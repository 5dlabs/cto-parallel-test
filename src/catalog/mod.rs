//! Product catalog module
//!
//! Provides product management functionality including CRUD operations,
//! inventory management, and filtering capabilities.

pub mod models;
pub mod service;

pub use self::models::{NewProduct, Product, ProductFilter};
pub use self::service::ProductService;
