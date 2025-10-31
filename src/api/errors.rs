//! Error handling for API endpoints
//!
//! This module provides standardized error types and JSON responses
//! for all API operations. All errors implement `ResponseError` to provide
//! consistent HTTP error responses.

use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

/// Standardized JSON error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error type identifier (e.g., `NOT_FOUND`, `BAD_REQUEST`)
    pub error: String,
    /// Human-readable error message with context
    pub message: String,
    /// HTTP status code
    pub status: u16,
}

/// API error types with automatic HTTP response mapping
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),
    /// Invalid request data (400)
    BadRequest(String),
    /// Server-side error (500)
    InternalError(String),
    /// Authentication required or failed (401)
    Unauthorized(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not Found: {msg}"),
            Self::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal Error: {msg}"),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            Self::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            Self::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            Self::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            status: status.as_u16(),
        })
    }
}
