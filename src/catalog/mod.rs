//! Product catalog module
//!
//! This module provides product catalog and inventory management functionality,
//! including models, service logic, and filtering capabilities.

pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
