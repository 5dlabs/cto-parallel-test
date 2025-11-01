//! API module for HTTP endpoints
//!
//! This module provides the REST API routing infrastructure using Actix-web framework.
//! It includes route configuration, error handling, and endpoint implementations.

pub mod errors;
pub mod routes;

pub use errors::ApiError;
pub use routes::configure_routes;
