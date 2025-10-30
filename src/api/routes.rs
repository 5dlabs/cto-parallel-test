use actix_web::{web, HttpResponse};

// Import schema to validate Task 1 dependency
#[allow(unused_imports)]
use crate::schema;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes)),
    );
}

/// Health check endpoint
/// Returns JSON: {"status": "ok"}
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Placeholder configuration for user routes
/// Task 3 will implement actual user authentication endpoints
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

/// Placeholder configuration for product routes
/// Task 4 will implement actual product catalog endpoints
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}
