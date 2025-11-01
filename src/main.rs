//! E-commerce API server
//!
//! Main entry point for the shopping cart API with JWT authentication.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cto_parallel_test::{api::configure_cart_routes, cart::CartService, catalog::ProductService};

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "cto-parallel-test"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize services
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    println!("🚀 Starting e-commerce API server...");
    println!("📍 Server running at http://127.0.0.1:8080");
    println!("🏥 Health check: http://127.0.0.1:8080/api/health");
    println!("🛒 Cart API: http://127.0.0.1:8080/api/cart");

    HttpServer::new(move || {
        App::new()
            .app_data(cart_service.clone())
            .app_data(product_service.clone())
            .route("/api/health", web::get().to(health_check))
            .configure(configure_cart_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
