//! API module for HTTP routes and error handling

pub mod errors;
pub mod routes;

pub use errors::ApiError;
pub use routes::configure_routes;
