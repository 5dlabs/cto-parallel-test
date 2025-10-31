use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;

/// Request body for adding an item to the cart
#[derive(Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Configure cart-related routes under /cart scope
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}

/// Extract and validate JWT token from Authorization header
///
/// # Errors
///
/// Returns `HttpResponse` with 401 status if:
/// - Authorization header is missing
/// - Authorization header is malformed
/// - Token validation fails
/// - User ID cannot be parsed
fn authenticate_user(req: &HttpRequest) -> Result<i32, HttpResponse> {
    // Extract Authorization header
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "Missing authorization header"}))
    })?;

    // Convert header to string
    let auth_str = auth_header.to_str().map_err(|_| {
        HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "Invalid authorization header"}))
    })?;

    // Check Bearer prefix
    if !auth_str.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "Invalid authorization format"})));
    }

    // Extract token
    let token = &auth_str[7..];

    // Validate token
    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid or expired token"}))
    })?;

    // Parse user_id from claims.sub
    let user_id = claims.sub.parse::<i32>().map_err(|_| {
        HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid user ID in token"}))
    })?;

    Ok(user_id)
}

/// GET /api/cart - Get user's shopping cart
///
/// Returns the authenticated user's cart or creates an empty one if it doesn't exist.
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_user(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Get or create cart
    let cart = cart_service.get_or_create_cart(user_id);

    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add item to cart
///
/// Validates the product exists and has sufficient inventory before adding to cart.
async fn add_item(
    req: HttpRequest,
    request: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_user(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Validate product exists
    let Some(product) = product_service.get_by_id(request.product_id) else {
        return HttpResponse::NotFound().json(serde_json::json!({"error": "Product not found"}));
    };

    // Check inventory
    if product.inventory_count < request.quantity {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Not enough inventory",
            "available": product.inventory_count,
            "requested": request.quantity
        }));
    }

    // Add item to cart
    let cart = cart_service.add_item(user_id, &product, request.quantity);

    HttpResponse::Ok().json(cart)
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
///
/// Removes the specified product from the user's cart.
async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_user(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Remove item
    match cart_service.remove_item(user_id, *product_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Cart or item not found"}))
        }
    }
}

/// POST /api/cart/clear - Clear all items from cart
///
/// Removes all items from the user's cart.
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_user(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Clear cart
    if let Some(cart) = cart_service.clear_cart(user_id) {
        HttpResponse::Ok().json(cart)
    } else {
        // If cart doesn't exist, create empty one
        let cart = cart_service.get_or_create_cart(user_id);
        HttpResponse::Ok().json(cart)
    }
}
