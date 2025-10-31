//! E-Commerce API Server
//!
//! REST API server for e-commerce application using Actix-web framework.
//! Provides endpoints for user authentication, product catalog, and shopping cart.

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use ecommerce_catalog::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logging with environment-based configuration
    // Default to INFO level, can be overridden with RUST_LOG env var
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Display startup information
    log::info!("🚀 Starting E-Commerce API Server");
    log::info!("📡 Server will listen on http://127.0.0.1:8080");
    log::info!("🏥 Health check available at http://127.0.0.1:8080/api/health");

    // Create and configure HTTP server
    HttpServer::new(|| {
        App::new()
            // Add request logging middleware
            .wrap(middleware::Logger::default())
            // Configure API routes
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
