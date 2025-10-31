//! API route configuration and handlers
//!
//! This module defines all HTTP routes for the e-commerce API, including:
//! - Health check endpoint
//! - User authentication routes (placeholder for Task 3)
//! - Product catalog routes (placeholder for Task 4)
//! - Shopping cart routes (placeholder for Task 5)

use actix_web::{web, HttpResponse};
use serde_json::json;

/// Main route configuration function
///
/// Registers all API endpoints under the `/api` scope. This function is called
/// during application initialization to configure the HTTP routing table.
///
/// # Route Structure
/// - `/api/health` - Health check endpoint (implemented)
/// - `/api/users` - User authentication endpoints (placeholder for Task 3)
/// - `/api/products` - Product catalog endpoints (placeholder for Task 4)
/// - `/api/cart` - Shopping cart endpoints (placeholder for Task 5)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .service(web::scope("/cart").configure(cart_routes)),
    );
}

/// Health check endpoint for monitoring
///
/// Returns basic service information and status. This endpoint is used by
/// load balancers and monitoring systems to verify the service is running.
///
/// # Endpoint
/// `GET /api/health`
///
/// # Response
/// ```json
/// {
///   "status": "ok",
///   "service": "e-commerce-api",
///   "version": "0.1.0"
/// }
/// ```
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "e-commerce-api",
        "version": "0.1.0"
    }))
}

/// User routes configuration (Task 3)
///
/// Placeholder for authentication and user management endpoints.
/// These routes will be fully implemented in Task 3 (User Authentication).
///
/// # Planned Endpoints
/// - `POST /api/users/register` - User registration
/// - `POST /api/users/login` - User login
/// - `GET /api/users/profile` - Get user profile
fn user_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 3 (User Authentication)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented)),
    );
}

/// Product routes configuration (Task 4)
///
/// Placeholder for product catalog endpoints.
/// These routes will be fully implemented in Task 4 (Product Catalog).
///
/// # Planned Endpoints
/// - `GET /api/products` - List all products
/// - `GET /api/products/{id}` - Get product details
/// - `POST /api/products` - Create new product (admin)
/// - `PUT /api/products/{id}` - Update product (admin)
/// - `DELETE /api/products/{id}` - Delete product (admin)
fn product_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 4 (Product Catalog)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented)),
    )
    .service(web::resource("/{id}").route(web::get().to(not_implemented)));
}

/// Cart routes configuration (Task 5)
///
/// Placeholder for shopping cart endpoints.
/// These routes will be fully implemented in Task 5 (Shopping Cart).
///
/// # Planned Endpoints
/// - `GET /api/cart` - Get user's cart
/// - `POST /api/cart/add` - Add item to cart
/// - `DELETE /api/cart/remove/{id}` - Remove item from cart
/// - `POST /api/cart/clear` - Clear cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 5 (Shopping Cart)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented)),
    );
}

/// Placeholder handler for not-yet-implemented endpoints
///
/// Returns HTTP 501 Not Implemented status with a JSON error message.
/// This handler is used for all routes that will be implemented in future tasks.
///
/// # Response
/// ```json
/// {
///   "error": "This endpoint is not yet implemented",
///   "message": "This functionality will be added in a future task"
/// }
/// ```
async fn not_implemented() -> HttpResponse {
    HttpResponse::NotImplemented().json(json!({
        "error": "This endpoint is not yet implemented",
        "message": "This functionality will be added in a future task"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_not_implemented_handler() {
        let resp = not_implemented().await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);
    }

    #[actix_web::test]
    async fn test_configure_routes() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        // Test health check
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test placeholder routes
        let req = test::TestRequest::get().uri("/api/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);

        let req = test::TestRequest::get().uri("/api/products").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);

        let req = test::TestRequest::get().uri("/api/cart").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);
    }
}
