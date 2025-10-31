//! API module with HTTP endpoints
//!
//! This module provides REST API endpoints for the application including:
//! - Cart management endpoints
//! - Authentication middleware
//! - Error handling

pub mod cart_routes;

pub use cart_routes::configure_cart_routes;
