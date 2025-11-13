//! Authentication module: JWT handling, user model, and DTOs.

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};

#[cfg(test)]
mod tests;
