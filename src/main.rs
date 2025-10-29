use actix_web::{App, HttpServer};

mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Startup message for server initialization (simple test project)
    #[allow(clippy::disallowed_macros)]
    {
        println!("Starting API server on http://127.0.0.1:8080");
    }

    HttpServer::new(|| App::new().configure(api::routes::configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
