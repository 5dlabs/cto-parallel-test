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
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// User routes placeholder - Task 3 will implement
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(user_placeholder)));
}

async fn user_placeholder() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

/// Product routes placeholder - Task 4 will implement
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(product_placeholder)));
}

async fn product_placeholder() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
