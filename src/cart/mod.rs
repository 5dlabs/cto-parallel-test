//! Shopping cart module with user isolation and inventory validation
//!
//! This module provides cart management functionality including:
//! - Cart creation and retrieval
//! - Adding/removing items from cart
//! - Inventory validation
//! - User-specific cart isolation

pub mod service;

pub use service::CartService;
