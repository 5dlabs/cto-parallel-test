//! API module providing HTTP endpoints for the shopping cart service
//!
//! This module contains the API routes and handlers for cart operations.
//! All endpoints require JWT authentication.

pub mod cart_routes;

pub use cart_routes::configure_cart_routes;
