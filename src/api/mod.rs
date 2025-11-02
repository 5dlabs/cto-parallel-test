//! API module for HTTP endpoints and routing
//!
//! This module provides the HTTP server infrastructure including:
//! - Route configuration
//! - Error handling
//! - Request/response handling

pub mod errors;
pub mod routes;

pub use errors::ApiError;
pub use routes::configure_routes;
