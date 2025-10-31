//! API module for REST endpoints
//!
//! This module provides the HTTP API layer for the e-commerce application:
//! - Route configuration and handlers
//! - Error handling with standardized JSON responses
//! - Health check for monitoring
//! - Placeholder routes for future modules

pub mod errors;
pub mod routes;

pub use self::errors::{ApiError, ErrorResponse};
pub use self::routes::configure_routes;
