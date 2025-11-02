//! API routes module for cart endpoints
//!
//! This module provides HTTP endpoints for shopping cart operations
//! with JWT authentication.

pub mod cart_routes;

pub use self::cart_routes::configure_cart_routes;
