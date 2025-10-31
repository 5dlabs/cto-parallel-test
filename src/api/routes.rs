use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(
                web::scope("/products")
                    .route("", web::get().to(get_products))
                    .route("/{id}", web::get().to(get_product)),
            )
            .service(
                web::scope("/cart")
                    .route("", web::get().to(get_cart))
                    .route("/add", web::post().to(add_to_cart))
                    .route("/remove/{product_id}", web::delete().to(remove_from_cart))
                    .route("/clear", web::post().to(clear_cart)),
            ),
    );
}

/// Health check endpoint
#[actix_web::get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// Get all products
async fn get_products(product_service: web::Data<ProductService>) -> impl Responder {
    let products = product_service.get_all();
    HttpResponse::Ok().json(products)
}

/// Get a specific product by ID
async fn get_product(
    product_service: web::Data<ProductService>,
    path: web::Path<i32>,
) -> impl Responder {
    let product_id = path.into_inner();
    match product_service.get_by_id(product_id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Product not found"})),
    }
}

#[derive(Deserialize)]
struct AddToCartRequest {
    product_id: i32,
    quantity: i32,
}

/// Extract user ID from JWT token in Authorization header
fn extract_user_id(req: &HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(serde_json::json!({"error": "Missing authorization"}))
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

/// Get the current user's cart
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    match extract_user_id(&req) {
        Ok(user_id) => {
            let cart = cart_service.get_or_create_cart(&user_id);
            HttpResponse::Ok().json(cart)
        }
        Err(err) => err,
    }
}

/// Add an item to the cart
async fn add_to_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    body: web::Json<AddToCartRequest>,
) -> impl Responder {
    match extract_user_id(&req) {
        Ok(user_id) => {
            cart_service.add_item(&user_id, body.product_id, body.quantity);
            let cart = cart_service.get_cart(&user_id).expect("Cart should exist");
            HttpResponse::Ok().json(cart)
        }
        Err(err) => err,
    }
}

/// Remove an item from the cart
async fn remove_from_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    path: web::Path<i32>,
) -> impl Responder {
    match extract_user_id(&req) {
        Ok(user_id) => {
            let product_id = path.into_inner();
            if cart_service.remove_item(&user_id, product_id) {
                let cart = cart_service.get_cart(&user_id).expect("Cart should exist");
                HttpResponse::Ok().json(cart)
            } else {
                HttpResponse::NotFound().json(serde_json::json!({"error": "Item not in cart"}))
            }
        }
        Err(err) => err,
    }
}

/// Clear the cart
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    match extract_user_id(&req) {
        Ok(user_id) => {
            cart_service.clear_cart(&user_id);
            let cart = cart_service.get_cart(&user_id).expect("Cart should exist");
            HttpResponse::Ok().json(cart)
        }
        Err(err) => err,
    }
}
