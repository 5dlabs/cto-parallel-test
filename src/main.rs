use actix_web::{web, App, HttpServer};
use cto_parallel_test::api::routes::configure_routes;
use cto_parallel_test::cart::CartService;
use cto_parallel_test::catalog::ProductService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize services
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    // Allow println in main for server startup message
    #[allow(clippy::disallowed_macros)]
    {
        println!("Starting server at http://127.0.0.1:8080");
    }

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(cart_service.clone())
            .app_data(product_service.clone())
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
