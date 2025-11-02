#![forbid(unsafe_code)]
//! cto-parallel-test library crate
//!
//! Provides a thread-safe in-memory product catalog with CRUD,
//! inventory management, and flexible filtering using Decimal prices.
//! Also includes secure user authentication with JWT tokens and Argon2 password hashing.

pub mod auth;
pub mod catalog;
