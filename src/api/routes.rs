use actix_web::{web, HttpResponse, Responder};

/// Configures all API routes for the application.
///
/// This function sets up the complete route structure:
/// - `/api/health` - Health check endpoint
/// - `/api/auth/*` - Authentication endpoints (Task 3)
/// - `/api/users/*` - User management endpoints (Task 3)
/// - `/api/products/*` - Product catalog endpoints (Task 4)
/// - `/api/cart/*` - Shopping cart endpoints (Task 5)
///
/// Placeholder routes return 501 Not Implemented to clearly indicate future work.
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

/// Health check endpoint that returns server status and version information.
///
/// # Returns
/// JSON response with:
/// - `status`: "ok" if server is running
/// - `version`: Current application version from Cargo.toml
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Configures authentication-related routes (Task 3).
///
/// Routes:
/// - `POST /api/auth/register` - User registration
/// - `POST /api/auth/login` - User login
fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(not_implemented))
        .route("/login", web::post().to(not_implemented));
}

/// Configures user management routes (Task 3).
///
/// Routes:
/// - `GET /api/users` - Get user information
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented));
}

/// Configures product catalog routes (Task 4).
///
/// Routes:
/// - `GET /api/products` - List all products
/// - `GET /api/products/{id}` - Get specific product by ID
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/{id}", web::get().to(not_implemented));
}

/// Configures shopping cart routes (Task 5).
///
/// Routes:
/// - `GET /api/cart` - Get current cart
/// - `POST /api/cart/add` - Add item to cart
/// - `DELETE /api/cart/remove/{product_id}` - Remove item from cart
/// - `POST /api/cart/clear` - Clear all items from cart
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(not_implemented))
        .route("/add", web::post().to(not_implemented))
        .route("/remove/{product_id}", web::delete().to(not_implemented))
        .route("/clear", web::post().to(not_implemented));
}

/// Placeholder handler for endpoints not yet implemented.
///
/// Returns HTTP 501 Not Implemented with a JSON error message.
async fn not_implemented() -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "not_implemented",
        "message": "This endpoint will be implemented in a later task"
    }))
}
