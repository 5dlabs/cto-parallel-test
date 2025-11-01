/// E-commerce backend library for Task 3: User Authentication Module
///
/// This library provides secure authentication functionality including:
/// - JWT token creation and validation
/// - Argon2 password hashing with random salt
/// - User models with password verification
/// - Authentication DTOs for request/response handling
pub mod auth;

pub use auth::{create_token, validate_token, Claims, User};
