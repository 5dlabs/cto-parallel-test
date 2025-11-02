use actix_web::{web, App, HttpServer};
use cto_parallel_test::{api::configure_cart_routes, cart::CartService, catalog::ProductService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize services
    let product_service = ProductService::new();
    let cart_service = CartService::new(product_service.clone());

    // Wrap in Arc for sharing across threads
    let cart_service_data = web::Data::new(cart_service);
    let product_service_data = web::Data::new(product_service);

    // Create and run HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(cart_service_data.clone())
            .app_data(product_service_data.clone())
            .configure(configure_cart_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
