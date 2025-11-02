//! Shopping Cart Module
//!
//! This module provides shopping cart functionality including:
//! - Cart CRUD operations
//! - Cart item management
//! - User-specific cart isolation
//! - Thread-safe in-memory storage

pub mod service;

pub use self::service::{Cart, CartItem, CartService};
