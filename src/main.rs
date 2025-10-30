use ecommerce_api::auth;

#[allow(clippy::disallowed_macros)] // Binary main uses println for demonstration
fn main() {
    println!("E-commerce API - Authentication Module Ready");

    // Example usage
    let token = auth::create_token("user123").expect("Failed to create token");
    println!("Created token: {}", &token[..50]); // Print first 50 chars

    let claims = auth::validate_token(&token).expect("Failed to validate token");
    println!("Token valid for user: {}", claims.sub);
}
