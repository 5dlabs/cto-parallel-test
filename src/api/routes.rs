use actix_web::{web, HttpResponse};

// Import schema to validate Task 1 dependency
// The schema is intentionally imported here to ensure Task 1 is complete.
// Future tasks will use this import for database operations.
#[allow(unused_imports)]
use crate::schema;

/// Configure all API routes under the /api scope
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes)),
    );
}

/// Health check endpoint that returns a simple status
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// User routes placeholder - will be implemented by Task 3
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(user_placeholder)));
}

/// Placeholder handler for user routes
async fn user_placeholder() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

/// Product routes placeholder - will be implemented by Task 4
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(product_placeholder)));
}

/// Placeholder handler for product routes
async fn product_placeholder() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
