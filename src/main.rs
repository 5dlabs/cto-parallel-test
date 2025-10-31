//! E-Commerce API Server
//!
//! This is the main entry point for the e-commerce REST API server.
//! It initializes the Actix-web HTTP server with all configured routes,
//! middleware, and database connections.

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use ecommerce_catalog::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("🚀 Starting E-Commerce API Server");
    log::info!("📡 Server will listen on http://127.0.0.1:8080");
    log::info!("🏥 Health check available at http://127.0.0.1:8080/api/health");

    // Create and configure HTTP server
    HttpServer::new(|| {
        App::new()
            // Add logging middleware to log all requests
            .wrap(middleware::Logger::default())
            // Configure API routes
            .configure(api::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
