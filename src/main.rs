use actix_web::{App, HttpServer};
use std::env;
use tracing::info;

mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Load server configuration from environment variables
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{host}:{port}");

    info!("Starting server on {}", bind_address);

    HttpServer::new(|| App::new().configure(api::routes::configure_routes))
        .bind(&bind_address)?
        .run()
        .await
}
