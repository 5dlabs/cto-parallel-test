//! E-Commerce API Server
//!
//! This binary starts the Actix-web HTTP server with all API routes configured.

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use cto_parallel_test::cart::CartService;
use cto_parallel_test::catalog::ProductService;
use std::io;

/// Health check endpoint.
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "e-commerce-api"
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment variables
    dotenv::dotenv().ok();

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create shared service instances
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    // Seed some products for testing
    let _ = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Laptop Pro".to_string(),
        description: "High-performance laptop for professionals".to_string(),
        price: rust_decimal::Decimal::new(129_999, 2), // 1299.99
        inventory_count: 10,
    });

    let _ = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Wireless Mouse".to_string(),
        description: "Ergonomic wireless mouse with 6 buttons".to_string(),
        price: rust_decimal::Decimal::new(2_999, 2), // 29.99
        inventory_count: 50,
    });

    let _ = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Mechanical Keyboard".to_string(),
        description: "RGB mechanical keyboard with blue switches".to_string(),
        price: rust_decimal::Decimal::new(14_999, 2), // 149.99
        inventory_count: 20,
    });

    log::info!("Starting E-Commerce API server on http://0.0.0.0:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Add services as app data
            .app_data(cart_service.clone())
            .app_data(product_service.clone())
            // Configure routes
            .route("/health", web::get().to(health_check))
            .configure(cto_parallel_test::api::configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
