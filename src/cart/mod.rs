//! Shopping cart module providing cart management functionality.
//!
//! This module provides data structures and business logic for managing
//! shopping carts with user isolation and product integration.

pub mod service;

pub use self::service::{Cart, CartItem, CartService};
