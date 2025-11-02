use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger with default level "info" (can be overridden with RUST_LOG env var)
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("🚀 Starting API server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(ecommerce_catalog::api::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
