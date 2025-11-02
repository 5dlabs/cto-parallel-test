//! API route configuration
//!
//! Defines all HTTP routes for the e-commerce API including:
//! - Health check endpoint
//! - Authentication routes (placeholder)
//! - User management routes (placeholder)
//! - Product catalog routes (placeholder)
//! - Shopping cart routes (placeholder)

use actix_web::{web, HttpResponse, Responder};

/// Configure all API routes
///
/// Sets up the main `/api` scope with all sub-routes:
/// - `/api/health` - Health check endpoint
/// - `/api/auth/*` - Authentication endpoints (Task 3)
/// - `/api/users/*` - User management endpoints (Task 3)
/// - `/api/products/*` - Product catalog endpoints (Task 4)
/// - `/api/cart/*` - Shopping cart endpoints (Task 5)
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
/// Returns HTTP 200 OK with service status and version information.
///
/// # Example Response
/// ```json
/// {
///   "status": "ok",
///   "version": "0.1.0"
/// }
/// ```
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Authentication route configuration (placeholder for Task 3)
///
/// Routes:
/// - `POST /auth/register` - User registration
/// - `POST /auth/login` - User login
fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(not_implemented))
        .route("/login", web::post().to(not_implemented));
}

/// User management route configuration (placeholder for Task 3)
///
/// Routes:
/// - `GET /users` - Get user information
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented));
}

/// Product catalog route configuration (placeholder for Task 4)
///
/// Routes:
/// - `GET /products` - List all products
/// - `GET /products/{id}` - Get product details
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/{id}", web::get().to(not_implemented));
}

/// Shopping cart route configuration (placeholder for Task 5)
///
/// Routes:
/// - `GET /cart` - Get user's cart
/// - `POST /cart/add` - Add item to cart
/// - `DELETE /cart/remove/{product_id}` - Remove item from cart
/// - `POST /cart/clear` - Clear cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/add", web::post().to(not_implemented))
        .route("/remove/{product_id}", web::delete().to(not_implemented))
        .route("/clear", web::post().to(not_implemented));
}

/// Placeholder handler for unimplemented endpoints
///
/// Returns HTTP 501 Not Implemented with a JSON error response.
///
/// # Example Response
/// ```json
/// {
///   "error": "not_implemented",
///   "message": "This endpoint will be implemented in a later task"
/// }
/// ```
async fn not_implemented() -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
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

        // Test GET endpoints
        let get_endpoints = vec!["/api/products", "/api/users", "/api/cart"];

        for endpoint in get_endpoints {
            let req = test::TestRequest::get().uri(endpoint).to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(
                resp.status(),
                actix_web::http::StatusCode::NOT_IMPLEMENTED,
                "GET {endpoint} should return 501"
            );
        }

        // Test POST endpoints
        let post_endpoints = vec!["/api/auth/register", "/api/auth/login"];

        for endpoint in post_endpoints {
            let req = test::TestRequest::post().uri(endpoint).to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(
                resp.status(),
                actix_web::http::StatusCode::NOT_IMPLEMENTED,
                "POST {endpoint} should return 501"
            );
        }
    }

    #[actix_web::test]
    async fn test_not_found() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        let req = test::TestRequest::get()
            .uri("/api/nonexistent")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    }
}
