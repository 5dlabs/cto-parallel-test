#![forbid(unsafe_code)]
//! cto-parallel-test library crate
//!
//! Provides a thread-safe in-memory product catalog with CRUD,
//! inventory management, and flexible filtering using Decimal prices.
//! Also includes JWT authentication and shopping cart functionality.

pub mod auth;
pub mod cart;
pub mod catalog;
