//! API module for HTTP endpoints and routing
//!
//! This module contains all API-related functionality including:
//! - Route configuration
//! - Error handling
//! - Request/response types

pub mod errors;
pub mod routes;

pub use errors::ApiError;
pub use routes::configure_routes;
