//! Product Catalog Module
//!
//! This module provides product management functionality including:
//! - Product CRUD operations
//! - Inventory management
//! - Product filtering and search
//! - Thread-safe in-memory storage
pub mod models;
pub mod service;

pub use self::models::{NewProduct, Product, ProductFilter};
pub use self::service::ProductService;
