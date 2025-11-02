//! Shopping Cart Module
//!
//! This module provides shopping cart functionality including cart management,
//! item operations, and integration with product catalog.

pub mod service;

pub use self::service::{Cart, CartItem, CartService};
