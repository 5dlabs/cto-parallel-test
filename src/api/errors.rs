//! API error handling
//!
//! Provides standardized error types and HTTP response formatting for API endpoints.

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

/// API error types
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),
    /// Bad request - invalid input (400)
    BadRequest(String),
    /// Internal server error (500)
    InternalError(String),
    /// Unauthorized access (401)
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
        match self {
            Self::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            Self::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            Self::InternalError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": msg
                }))
            }
            Self::Unauthorized(msg) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthorized",
                "message": msg
            })),
        }
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
    fn test_error_display() {
        let error = ApiError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not Found: User not found");

        let error = ApiError::BadRequest("Invalid input".to_string());
        assert_eq!(error.to_string(), "Bad Request: Invalid input");

        let error = ApiError::InternalError("Database error".to_string());
        assert_eq!(error.to_string(), "Internal Error: Database error");

        let error = ApiError::Unauthorized("Invalid token".to_string());
        assert_eq!(error.to_string(), "Unauthorized: Invalid token");
    }

    #[test]
    fn test_status_codes() {
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
