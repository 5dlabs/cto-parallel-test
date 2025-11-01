//! Product catalog module providing inventory management and filtering capabilities.
//!
//! This module offers thread-safe product management with:
//! - CRUD operations for products
//! - Inventory tracking and updates
//! - Flexible filtering by name, price, and stock status
//! - Decimal precision for prices
//! - Auto-incrementing product IDs

pub mod models;
pub mod service;

pub use self::models::{NewProduct, Product, ProductFilter};
pub use self::service::ProductService;
