use actix_web::{App, HttpServer};

// Make modules public for testing
pub mod api;
pub mod auth;
pub mod cart;
pub mod catalog;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::routes::configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
