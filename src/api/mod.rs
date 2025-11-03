//! API module
//!
//! Provides HTTP endpoints for the application with JWT authentication.

pub mod cart_routes;

pub use self::cart_routes::configure_cart_routes;
