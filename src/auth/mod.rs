//! User Authentication Module
//!
//! This module provides JWT-based authentication and secure password hashing
//! using Argon2. It includes token creation, validation, and user password
//! management.
//!
//! # Features
//! - JWT token generation with 24-hour expiration
//! - Argon2 password hashing with random salt
//! - Password verification with constant-time comparison
//! - Secure serialization (password hashes never exposed)
//!
//! # Example
//! ```
//! use cto_parallel_test::auth::{User, create_token, validate_token};
//!
//! // Hash a password
//! let password = "secure_password";
//! let hash = User::hash_password(password).expect("Failed to hash password");
//!
//! // Create user
//! let user = User {
//!     id: 1,
//!     username: "testuser".to_string(),
//!     email: "test@example.com".to_string(),
//!     password_hash: hash,
//! };
//!
//! // Verify password
//! assert!(user.verify_password(password));
//!
//! // Create JWT token
//! let token = create_token("1").expect("Failed to create token");
//!
//! // Validate token
//! let claims = validate_token(&token).expect("Failed to validate token");
//! assert_eq!(claims.sub, "1");
//! ```

pub mod jwt;
pub mod models;

// Re-export commonly used types and functions
pub use self::jwt::{create_token, validate_token, Claims, Clock, SystemClock};
pub use self::models::{AuthResponse, LoginRequest, RegisterRequest, User};
