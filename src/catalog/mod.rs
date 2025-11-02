//! Product Catalog Module
//!
//! This module provides product management functionality including CRUD operations,
//! inventory tracking, and product filtering.

pub mod models;
pub mod service;

pub use self::models::{NewProduct, Product, ProductFilter};
pub use self::service::ProductService;
