//! Shopping cart module
//!
//! Provides cart management with user isolation and inventory validation.

pub mod service;

pub use service::{Cart, CartItem, CartService};
