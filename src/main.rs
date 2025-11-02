//! E-Commerce Application Server
//!
//! This is the main entry point for the HTTP server.
//! It configures and starts the Actix-web server with all routes and middleware.

use actix_web::{middleware::Logger, App, HttpServer};
use cto_parallel_test::configure_routes;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger with INFO level by default
    // Can be overridden with RUST_LOG environment variable
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = "127.0.0.1";
    let port = 8080;

    log::info!("🚀 Starting API server on http://{host}:{port}");
    log::info!("📝 Health check available at http://{host}:{port}/api/health");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
