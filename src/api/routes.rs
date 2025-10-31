use actix_web::{web, HttpResponse};

use crate::api::cart_routes::configure_cart_routes;

/// Configure all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .configure(configure_cart_routes),
    );
}

/// Health check endpoint - returns API status
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Placeholder for user routes (Task 3 will implement)
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

/// Placeholder for product routes (Task 4 will implement)
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}
