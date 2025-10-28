use actix_web::{App, HttpServer};

mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: Server startup logged by actix-web runtime
    // For production: implement proper tracing initialization

    HttpServer::new(|| App::new().configure(api::routes::configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
