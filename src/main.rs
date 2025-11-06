use ecommerce_api::auth::{create_token, validate_token, User};

fn main() {
    println!("E-Commerce API - Authentication Module");
    println!("======================================\n");

    // Example: Hash a password
    let password = "example_password_123";
    let password_hash = User::hash_password(password);
    println!("Password hashed successfully");

    // Example: Create a user
    let user = User {
        id: 1,
        username: "demo_user".to_string(),
        email: "demo@example.com".to_string(),
        password_hash: password_hash.clone(),
    };

    // Example: Verify password
    if user.verify_password(password) {
        println!("✓ Password verification successful");
    } else {
        println!("✗ Password verification failed");
    }

    // Example: Create JWT token
    match create_token(&user.id.to_string()) {
        Ok(token) => {
            println!("✓ JWT token created successfully");
            println!("  Token: {}...", &token[..50]);

            // Example: Validate JWT token
            match validate_token(&token) {
                Ok(claims) => {
                    println!("✓ JWT token validated successfully");
                    println!("  User ID: {}", claims.sub);
                    println!("  Issued at: {}", claims.iat);
                    println!("  Expires at: {}", claims.exp);
                }
                Err(e) => println!("✗ Token validation failed: {e}"),
            }
        }
        Err(e) => println!("✗ Token creation failed: {e}"),
    }

    println!("\nAuthentication module is working correctly!");
}
