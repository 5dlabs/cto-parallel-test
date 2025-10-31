//! CTO Parallel Test - User Authentication Module
//!
//! This library provides JWT-based authentication with secure password hashing using Argon2.
//!
//! # Features
//! - JWT token creation and validation with 24-hour expiration
//! - Secure password hashing using Argon2 with random salts
//! - User model with authentication methods
//!
//! # Examples
//!
//! ## Creating and validating JWT tokens
//! ```
//! use cto_parallel_test::auth::{create_token, validate_token};
//!
//! // Create a token for a user
//! let token = create_token("user123").expect("Failed to create token");
//!
//! // Validate the token
//! let claims = validate_token(&token).expect("Failed to validate token");
//! assert_eq!(claims.sub, "user123");
//! ```
//!
//! ## Password hashing and verification
//! ```
//! use cto_parallel_test::auth::User;
//!
//! // Hash a password
//! let password = "secure_password";
//! let hash = User::hash_password(password).expect("Failed to hash password");
//!
//! // Create a user with the hashed password
//! let user = User::new(1, "john".to_string(), "john@example.com".to_string(), hash);
//!
//! // Verify the password
//! assert!(user.verify_password(password));
//! assert!(!user.verify_password("wrong_password"));
//! ```

pub mod auth;

// Re-export commonly used types for convenience
pub use auth::{create_token, validate_token, Claims, User};
