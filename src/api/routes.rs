use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

/// Get all products
async fn get_products(product_service: web::Data<ProductService>) -> impl Responder {
    let products = product_service.get_all();
    HttpResponse::Ok().json(products)
}

/// Get a specific product by ID
async fn get_product_by_id(
    product_service: web::Data<ProductService>,
    product_id: web::Path<i32>,
) -> impl Responder {
    match product_service.get_by_id(*product_id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().json(json!({"error": "Product not found"})),
    }
}

/// Add item to cart (requires authentication)
async fn add_to_cart(
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
    req: actix_web::HttpRequest,
    body: web::Json<AddToCartRequest>,
) -> impl Responder {
    // Extract and validate JWT token
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(json!({"error": e})),
    };

    // Verify product exists
    if product_service.get_by_id(body.product_id).is_none() {
        return HttpResponse::NotFound().json(json!({"error": "Product not found"}));
    }

    // Add to cart
    match cart_service.add_item(&user_id, body.product_id, body.quantity) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

/// Get user's cart (requires authentication)
async fn get_cart(
    cart_service: web::Data<CartService>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // Extract and validate JWT token
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(json!({"error": e})),
    };

    let cart = cart_service.get_cart(&user_id);
    HttpResponse::Ok().json(cart)
}

/// Helper function to extract user ID from JWT token
fn extract_user_id(req: &actix_web::HttpRequest) -> Result<String, String> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or("Missing Authorization header")?
        .to_str()
        .map_err(|_| "Invalid Authorization header")?;

    if !auth_header.starts_with("Bearer ") {
        return Err("Invalid Authorization format".to_string());
    }

    let token = &auth_header[7..]; // Skip "Bearer "
    let claims = validate_token(token).map_err(|_| "Invalid or expired token")?;

    Ok(claims.sub)
}

/// Request body for adding to cart
#[derive(serde::Deserialize)]
struct AddToCartRequest {
    product_id: i32,
    quantity: i32,
}

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/products", web::get().to(get_products))
            .route("/products/{id}", web::get().to(get_product_by_id))
            .route("/cart", web::get().to(get_cart))
            .route("/cart/add", web::post().to(add_to_cart)),
    );
}
