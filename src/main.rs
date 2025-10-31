use actix_web::{App, HttpServer};

mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Server starts on http://127.0.0.1:8080
    // Note: Logging will be added when tracing is configured in future tasks

    HttpServer::new(|| App::new().configure(api::routes::configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
