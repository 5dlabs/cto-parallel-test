//! API route configuration module
//!
//! Defines all HTTP routes and their handlers, organized into scopes
//! for different resources (auth, users, products, cart).

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// Configure all application routes
///
/// This function sets up the complete routing structure with:
/// - Health check endpoint
/// - Authentication routes (to be implemented in Task 3)
/// - User management routes (to be implemented in Task 3)
/// - Product catalog routes (to be implemented in Task 4)
/// - Shopping cart routes (to be implemented in Task 5)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .service(web::scope("/auth").configure(auth_routes))
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .service(web::scope("/cart").configure(cart_routes)),
    );
}

/// Health check endpoint
///
/// Returns API status and version information.
/// Always returns 200 OK when the service is running.
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Configure authentication routes (Task 3)
///
/// Routes:
/// - `POST /api/auth/register` - User registration
/// - `POST /api/auth/login` - User login
fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(not_implemented))
        .route("/login", web::post().to(not_implemented));
}

/// Configure user management routes (Task 3)
///
/// Routes:
/// - `GET /api/users` - Get user information
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented));
}

/// Configure product catalog routes (Task 4)
///
/// Routes:
/// - `GET /api/products` - List all products
/// - `GET /api/products/{id}` - Get product by ID
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/{id}", web::get().to(not_implemented));
}

/// Configure shopping cart routes (Task 5)
///
/// Routes:
/// - `GET /api/cart` - Get user's cart
/// - `POST /api/cart/add` - Add item to cart
/// - `DELETE /api/cart/remove/{product_id}` - Remove item from cart
/// - `POST /api/cart/clear` - Clear cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/add", web::post().to(not_implemented))
        .route("/remove/{product_id}", web::delete().to(not_implemented))
        .route("/clear", web::post().to(not_implemented));
}

/// Placeholder handler for not-yet-implemented endpoints
///
/// Returns HTTP 501 Not Implemented with a JSON message
/// indicating the endpoint will be implemented in a future task.
async fn not_implemented() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({
        "error": "not_implemented",
        "message": "This endpoint will be implemented in a later task"
    }))
}
