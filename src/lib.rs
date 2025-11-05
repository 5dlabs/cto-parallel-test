//! E-commerce API - Database Schema and ORM Models
//!
//! This crate provides the foundational database layer for an e-commerce REST API,
//! including schema definitions, ORM models, and database connection management.
//!
//! ## Database Schema
//!
//! The schema includes four main tables:
//! - **users**: User authentication and profile information
//! - **products**: Product catalog with inventory tracking
//! - **carts**: Shopping carts associated with users
//! - **`cart_items`**: Line items within shopping carts
//!
//! ## Features
//!
//! - Diesel ORM integration with `PostgreSQL`
//! - Connection pooling with r2d2
//! - Type-safe schema definitions
//! - Comprehensive model structs with proper traits
//! - Foreign key relationships with cascade delete
//!
//! ## Example
//!
//! ```rust,no_run
//! use ecommerce_api::config::db::establish_connection_pool;
//! use ecommerce_api::models::NewUser;
//!
//! let pool = establish_connection_pool();
//! let mut conn = pool.get().expect("Failed to get connection");
//!
//! // Use the connection for database operations
//! ```

pub mod config;
pub mod models;
pub mod schema;
