//! API route configuration
//!
//! This module defines all HTTP routes and their handlers.
//! Routes are organized into scopes by functionality:
//! - `/api/health` - Health check endpoint
//! - `/api/auth/*` - Authentication endpoints (Task 3)
//! - `/api/users/*` - User management endpoints (Task 3)
//! - `/api/products/*` - Product catalog endpoints (Task 4)
//! - `/api/cart/*` - Shopping cart endpoints (Task 5)

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// Configure all API routes
///
/// This function is called during application setup to register
/// all route handlers and middleware.
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
/// Returns server status and version information.
/// This endpoint is used for monitoring and load balancer health checks.
///
/// # Example Response
/// ```json
/// {
///   "status": "ok",
///   "version": "0.1.0"
/// }
/// ```
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Configure authentication routes (Task 3)
///
/// Placeholder routes for user authentication functionality:
/// - POST /auth/register - User registration
/// - POST /auth/login - User login with JWT token
fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(not_implemented))
        .route("/login", web::post().to(not_implemented));
}

/// Configure user management routes (Task 3)
///
/// Placeholder routes for user management:
/// - GET /users - Get user information (requires authentication)
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented));
}

/// Configure product catalog routes (Task 4)
///
/// Placeholder routes for product catalog:
/// - GET /products - List all products
/// - GET /products/{id} - Get product details
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/{id}", web::get().to(not_implemented));
}

/// Configure shopping cart routes (Task 5)
///
/// Placeholder routes for shopping cart:
/// - GET /cart - Get user's cart
/// - POST /cart/add - Add item to cart
/// - DELETE `/cart/remove/{product_id}` - Remove item from cart
/// - POST /cart/clear - Clear cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/add", web::post().to(not_implemented))
        .route("/remove/{product_id}", web::delete().to(not_implemented))
        .route("/clear", web::post().to(not_implemented));
}

/// Placeholder handler for endpoints not yet implemented
///
/// Returns 501 Not Implemented status with a JSON error message.
/// This handler is used during development to indicate that an endpoint
/// is planned but not yet functional.
async fn not_implemented() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({
        "error": "not_implemented",
        "message": "This endpoint will be implemented in a later task"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        let req = test::TestRequest::get().uri("/api/health").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
        assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));
    }

    #[actix_web::test]
    async fn test_not_implemented_endpoints() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        // Test product endpoint
        let req = test::TestRequest::get().uri("/api/products").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 501);

        // Test cart endpoint
        let req = test::TestRequest::get().uri("/api/cart").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 501);

        // Test auth register endpoint
        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 501);
    }

    #[actix_web::test]
    async fn test_404_for_invalid_routes() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        let req = test::TestRequest::get()
            .uri("/api/nonexistent")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }
}
