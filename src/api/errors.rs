//! Error handling for API endpoints
//!
//! This module provides a unified error type for API responses with
//! appropriate HTTP status codes and JSON error formatting.

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

/// API error types representing different error conditions
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

/// JSON error response structure
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
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

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message: message.clone(),
        };

        HttpResponse::build(self.status_code()).json(error_response)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let error = ApiError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not Found: User not found");

        let error = ApiError::BadRequest("Invalid input".to_string());
        assert_eq!(error.to_string(), "Bad Request: Invalid input");
    }

    #[test]
    fn test_api_error_status_codes() {
        assert_eq!(
            ApiError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::BadRequest("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::InternalError("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            ApiError::Unauthorized("test".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
    }
}
