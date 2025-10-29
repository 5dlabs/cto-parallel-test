//! CTO Parallel Test - Authentication Service Binary
//!
//! This is a minimal binary stub for the authentication library.
//! The actual authentication logic is in the library crate.

// Allow println! in main binary for demonstration/CLI purposes
#![allow(clippy::disallowed_macros)]

use cto_parallel_test::auth::{create_token, validate_token, User};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        println!("cto-parallel-test v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    println!("CTO Parallel Test - Authentication Service");
    println!("===========================================");
    println!();
    println!("This is a library crate with authentication utilities.");
    println!("Available modules:");
    println!("  - JWT token creation and validation");
    println!("  - User password hashing and verification");
    println!();
    println!("Example usage:");
    println!("  use cto_parallel_test::auth::{{create_token, validate_token}};");
    println!();

    // Demonstrate functionality
    println!("Demo: Creating and validating a token...");
    let token = create_token("demo_user_123")?;
    println!("✓ Token created: {}...", &token[..50]);

    let claims = validate_token(&token)?;
    println!("✓ Token validated for user: {}", claims.sub);

    println!();
    println!("Demo: Hashing and verifying a password...");
    let password = "demo_password_123";
    let hash = User::hash_password(password);
    println!("✓ Password hashed: {}...", &hash[..30]);

    let user = User {
        id: 1,
        username: "demo_user".to_string(),
        email: "demo@example.com".to_string(),
        password_hash: hash,
    };

    if user.verify_password(password) {
        println!("✓ Password verified successfully");
    }

    println!();
    println!("All authentication components working correctly!");

    Ok(())
}
