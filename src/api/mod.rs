use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

/// Configure API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/products", web::get().to(super::catalog::get_all_products))
            .route(
                "/products/{id}",
                web::get().to(super::catalog::get_product_by_id),
            )
            .route("/cart", web::get().to(super::cart::get_cart))
            .route("/cart/add", web::post().to(super::cart::add_to_cart)),
    );
}
