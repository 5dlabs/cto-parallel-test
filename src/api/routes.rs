//! API route configuration
//!
//! This module configures all HTTP routes for the application, including:
//! - Health check endpoint
//! - Authentication routes (Task 3)
//! - User management routes (Task 3)
//! - Product catalog routes (Task 4)
//! - Shopping cart routes (Task 5)

use actix_web::{web, HttpResponse, Responder};

/// Configure all application routes
///
/// This function sets up the main `/api` scope and all sub-scopes for different
/// resource types. Placeholder routes return 501 Not Implemented until their
/// respective tasks are completed.
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
/// Returns the API status and version information.
/// This endpoint is always available and requires no authentication.
///
/// # Returns
/// - 200 OK with JSON: `{"status": "ok", "version": "x.y.z"}`
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Authentication route configuration
///
/// Placeholder for Task 3: User Authentication
/// Routes:
/// - POST /api/auth/register - User registration
/// - POST /api/auth/login - User login
fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(not_implemented))
        .route("/login", web::post().to(not_implemented));
}

/// User management route configuration
///
/// Placeholder for Task 3: User Authentication
/// Routes:
/// - GET /api/users - Get user profile
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented));
}

/// Product catalog route configuration
///
/// Placeholder for Task 4: Product Catalog
/// Routes:
/// - GET /api/products - List all products
/// - GET /api/products/{id} - Get product by ID
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/{id}", web::get().to(not_implemented));
}

/// Shopping cart route configuration
///
/// Placeholder for Task 5: Shopping Cart
/// Routes:
/// - GET /api/cart - Get user's cart
/// - POST /api/cart/add - Add item to cart
/// - DELETE `/api/cart/remove/{product_id}` - Remove item from cart
/// - POST /api/cart/clear - Clear cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/add", web::post().to(not_implemented))
        .route("/remove/{product_id}", web::delete().to(not_implemented))
        .route("/clear", web::post().to(not_implemented));
}

/// Placeholder handler for unimplemented endpoints
///
/// Returns 501 Not Implemented with a JSON error message indicating
/// the endpoint will be implemented in a future task.
async fn not_implemented() -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "not_implemented",
        "message": "This endpoint will be implemented in a later task"
    }))
}
