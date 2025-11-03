#![forbid(unsafe_code)]
//! cto-parallel-test library crate
//!
//! Provides a thread-safe in-memory product catalog with CRUD,
//! inventory management, and flexible filtering using Decimal prices.
//! Also includes secure user authentication with JWT tokens and Argon2 password hashing.
//! Shopping cart functionality with JWT authentication and inventory validation.
//! Database schema and ORM models for `PostgreSQL` with Diesel.

pub mod api;
pub mod auth;
pub mod cart;
pub mod catalog;
pub mod config;
pub mod models;
pub mod schema;

// Re-export commonly used items for convenience
pub use config::db::{establish_connection_pool, DbConnection, Pool};
pub use models::{Cart, CartItem, NewCart, NewCartItem, NewProduct, NewUser, Product, User};
