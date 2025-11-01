//! E-Commerce API Server
//!
//! This is the main entry point for the HTTP API server.
//! It initializes the Actix-web server with logging middleware and route configuration.

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger with INFO level by default
    // Can be overridden with RUST_LOG environment variable
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("🚀 Starting E-Commerce API server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(cto_parallel_test::api::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
