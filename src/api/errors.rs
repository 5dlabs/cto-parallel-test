//! API error handling module
//!
//! Provides a unified error type for API endpoints with proper HTTP status codes
//! and JSON error responses.

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

/// API error types with corresponding HTTP status codes
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),
    /// Invalid request data (400)
    BadRequest(String),
    /// Internal server error (500)
    InternalError(String),
    /// Authentication required or failed (401)
    Unauthorized(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not Found: {msg}"),
            Self::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal Error: {msg}"),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (error_type, message) = match self {
            Self::NotFound(msg) => ("not_found", msg),
            Self::BadRequest(msg) => ("bad_request", msg),
            Self::InternalError(msg) => ("internal_error", msg),
            Self::Unauthorized(msg) => ("unauthorized", msg),
        };

        HttpResponse::build(self.status_code()).json(json!({
            "error": error_type,
            "message": message
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
