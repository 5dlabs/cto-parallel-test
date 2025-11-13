//! Authentication module: JWT handling, user model, and DTOs.

pub mod jwt;
pub mod models;

pub use self::jwt::{Claims, create_token, validate_token};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};

#[cfg(test)]
mod tests;
