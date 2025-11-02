//! Authentication module exposing JWT and user models.

pub mod jwt;
pub mod models;

pub use self::jwt::{create_token, validate_token, Claims, JwtError};
pub use self::models::User;

// Unit tests for the auth module
#[cfg(test)]
mod tests;
