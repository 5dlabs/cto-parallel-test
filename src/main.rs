use actix_web::{web, App, HttpServer};

mod api;
mod auth;
mod cart;
mod catalog;
mod schema;

use cart::CartService;
use catalog::ProductService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize services
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    HttpServer::new(move || {
        App::new()
            .app_data(cart_service.clone())
            .app_data(product_service.clone())
            .configure(api::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
