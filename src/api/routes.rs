use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Health check response
#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
}

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

/// Get all products
async fn get_products(product_service: web::Data<ProductService>) -> impl Responder {
    let products = product_service.get_all();
    HttpResponse::Ok().json(products)
}

/// Get product by ID
async fn get_product_by_id(
    product_service: web::Data<ProductService>,
    product_id: web::Path<i32>,
) -> impl Responder {
    match product_service.get_by_id(*product_id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found"
        })),
    }
}

/// Request body for adding to cart
#[derive(Deserialize)]
struct AddToCartRequest {
    product_id: i32,
    quantity: i32,
}

/// Extract user ID from Authorization header
fn extract_user_id(req: &HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            }))
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid authorization header format"
        }))
    })?;

    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid token"
        }))
    })?;

    Ok(claims.sub)
}

/// Add item to cart (requires authentication)
async fn add_to_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    body: web::Json<AddToCartRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    cart_service.add_to_cart(&user_id, body.product_id, body.quantity);
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Item added to cart"
    }))
}

/// Get cart contents (requires authentication)
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_cart(&user_id);
    HttpResponse::Ok().json(cart)
}

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/products", web::get().to(get_products))
            .route("/products/{id}", web::get().to(get_product_by_id))
            .route("/cart/add", web::post().to(add_to_cart))
            .route("/cart", web::get().to(get_cart)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check_endpoint() {
        let app = test::init_service(App::new().configure(configure_routes)).await;

        let req = test::TestRequest::get().uri("/api/health").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "ok");
    }
}
