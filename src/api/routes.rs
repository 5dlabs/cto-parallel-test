use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

// Import schema to validate Task 1 dependency
// The schema module contains the database table definitions and must be available
#[allow(unused_imports)]
use crate::schema;

use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/products").configure(product_routes))
            .service(web::scope("/cart").configure(cart_routes)),
    );
}

/// Health check endpoint
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Product routes
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_products)))
        .service(web::resource("/{id}").route(web::get().to(get_product)));
}

async fn get_products(product_service: web::Data<ProductService>) -> HttpResponse {
    let products = product_service.get_all();
    HttpResponse::Ok().json(products)
}

#[derive(Deserialize)]
struct ProductPath {
    id: i32,
}

async fn get_product(
    product_service: web::Data<ProductService>,
    path: web::Path<ProductPath>,
) -> HttpResponse {
    match product_service.get_by_id(path.id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Product not found"})),
    }
}

/// Cart routes
fn cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_cart)))
        .service(web::resource("/add").route(web::post().to(add_to_cart)));
}

/// Extracts user ID from JWT token in Authorization header
fn extract_user_id(req: &HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "Missing authorization header"}))
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "Invalid authorization format"}))
    })?;

    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid token"}))
    })?;

    Ok(claims.sub)
}

async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_or_create_cart(&user_id);
    HttpResponse::Ok().json(cart)
}

#[derive(Deserialize)]
struct AddToCartRequest {
    product_id: i32,
    quantity: i32,
}

async fn add_to_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    payload: web::Json<AddToCartRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    cart_service.add_item(&user_id, payload.product_id, payload.quantity);
    let cart = cart_service.get_or_create_cart(&user_id);
    HttpResponse::Ok().json(cart)
}
