use actix_web::{web, HttpResponse};

/// Configure all API routes
///
/// # Database Schema Dependency
/// This module depends on Task 1's database schema (`crate::schema`).
/// The schema defines tables for users, products, carts, and `cart_items`
/// that will be used by the authentication, catalog, and cart endpoints
/// implemented in subsequent tasks.
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

/// User routes configuration (placeholder for Task 3)
fn user_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder - Task 3 will implement user authentication routes
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

/// Product routes configuration (placeholder for Task 4)
fn product_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder - Task 4 will implement product catalog routes
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}
