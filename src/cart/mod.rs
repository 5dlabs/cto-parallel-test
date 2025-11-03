//! Shopping Cart Module
//!
//! This module provides shopping cart functionality including:
//! - Thread-safe in-memory cart storage per user
//! - Cart CRUD operations (create, read, update, delete)
//! - Integration with Product Catalog for validation
//! - Inventory checking before adding items

pub mod service;

pub use self::service::{Cart, CartItem, CartService};
