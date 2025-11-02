//! Shopping cart module for e-commerce functionality
//!
//! This module provides:
//! - Thread-safe in-memory cart storage per user
//! - CRUD operations for cart items
//! - Integration with product catalog for validation
//! - Inventory checking before adding items

pub mod service;

pub use self::service::{Cart, CartItem, CartService};
