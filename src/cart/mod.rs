//! Shopping cart module providing cart management functionality
//!
//! This module provides thread-safe shopping cart operations with user isolation.
//! Each user has their own cart, and operations validate inventory before adding items.

pub mod models;
pub mod service;

pub use self::models::{Cart, CartItem};
pub use self::service::CartService;
