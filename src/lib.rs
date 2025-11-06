/// User authentication module with JWT and Argon2 password hashing
pub mod auth;

// Re-export commonly used types for convenience
pub use auth::{create_token, validate_token, Claims, User};
