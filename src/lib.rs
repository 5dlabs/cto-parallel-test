//! E-Commerce Application Library
//!
//! This library provides the core functionality for an e-commerce application,
//! including database schema, models, and business logic.

pub mod catalog;
pub mod config;
pub mod models;
pub mod schema;

// Re-export commonly used types
pub use catalog::{NewProduct, Product, ProductFilter, ProductService};
pub use config::db::{establish_connection_pool, DbConnection, Pool};
