//! API error types and error response handling
//!
//! This module defines standardized error types and their conversion to HTTP responses.
//! All API errors are serialized to JSON format with consistent structure.

use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

/// Standard error response structure for API endpoints
///
/// All error responses follow this format for consistency across the API.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error type identifier (e.g., `"NOT_FOUND"`, `"BAD_REQUEST"`)
    pub error: String,
    /// Human-readable error message
    pub message: String,
    /// HTTP status code
    pub status: u16,
}

/// API error types
///
/// Represents all possible error conditions in the API. Each variant
/// corresponds to a specific HTTP status code and error type.
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),
    /// Invalid request data (400)
    BadRequest(String),
    /// Internal server error (500)
    InternalError(String),
    /// Unauthorized access (401)
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::error::ResponseError;

    #[test]
    fn test_error_display() {
        let err = ApiError::NotFound("User not found".to_string());
        assert_eq!(err.to_string(), "Not Found: User not found");

        let err = ApiError::BadRequest("Invalid input".to_string());
        assert_eq!(err.to_string(), "Bad Request: Invalid input");
    }

    #[test]
    fn test_error_response_status_codes() {
        let err = ApiError::NotFound("test".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let err = ApiError::BadRequest("test".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let err = ApiError::InternalError("test".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let err = ApiError::Unauthorized("test".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
