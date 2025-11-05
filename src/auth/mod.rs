/// JWT token creation and validation module
pub mod jwt;
/// User models and authentication data structures
pub mod models;

// Re-export commonly used types for convenience
pub use self::jwt::{create_token, validate_token, Claims};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
