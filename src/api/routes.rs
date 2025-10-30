use actix_web::{web, HttpResponse};

// Import schema to validate Task 1 dependency
// This ensures that Task 1 (Database Schema Setup) has completed successfully
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
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// User routes placeholder (will be implemented by Task 3)
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

/// Product routes placeholder (will be implemented by Task 4)
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}
