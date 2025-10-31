//! API module for HTTP endpoints and error handling
//!
//! This module provides the web API layer for the e-commerce application,
//! including route configuration, error handling, and HTTP request/response
//! serialization.

pub mod errors;
pub mod routes;

pub use self::errors::{ApiError, ErrorResponse};
pub use self::routes::configure_routes;
