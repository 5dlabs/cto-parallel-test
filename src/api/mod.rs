pub mod cart_routes;

use actix_web::web;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/cart").configure(cart_routes::configure_cart_routes)),
    );
}
