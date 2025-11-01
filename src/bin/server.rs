//! Shopping Cart API Server
//!
//! This binary provides an HTTP API server for the shopping cart service.
//! All endpoints require JWT authentication.
//!
//! ## Configuration
//!
//! - `HOST`: Server host address (default: "127.0.0.1")
//! - `PORT`: Server port (default: "8080")
//! - `JWT_SECRET`: Secret key for JWT token signing (default: test key for development)
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin server
//! ```
//!
//! ## API Endpoints
//!
//! - `GET /api/cart` - Get user's cart
//! - `POST /api/cart/add` - Add item to cart
//! - `PUT /api/cart/update/{product_id}` - Update item quantity
//! - `DELETE /api/cart/remove/{product_id}` - Remove item from cart
//! - `POST /api/cart/clear` - Clear cart
//!
//! All endpoints require `Authorization: Bearer <token>` header.

use actix_web::{middleware, web, App, HttpServer};
use ecommerce_catalog::{api::configure_cart_routes, cart::CartService, catalog::ProductService};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{host}:{port}");

    // Initialize services
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    println!("Starting Shopping Cart API server at http://{bind_address}");
    println!("API endpoints:");
    println!("  GET    /api/cart                  - Get user's cart");
    println!("  POST   /api/cart/add              - Add item to cart");
    println!("  PUT    /api/cart/update/{{id}}     - Update item quantity");
    println!("  DELETE /api/cart/remove/{{id}}     - Remove item from cart");
    println!("  POST   /api/cart/clear            - Clear cart");
    println!();
    println!("All endpoints require JWT authentication:");
    println!("  Authorization: Bearer <token>");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Register shared services
            .app_data(cart_service.clone())
            .app_data(product_service.clone())
            // Configure API routes
            .service(web::scope("/api/cart").configure(configure_cart_routes))
    })
    .bind(&bind_address)?
    .run()
    .await
}
