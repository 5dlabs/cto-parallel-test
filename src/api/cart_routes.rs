use crate::auth::{validate_token, Claims};
use crate::cart::CartService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request payload for adding items to cart
#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Error response for API errors
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

/// Extract user ID from JWT token in Authorization header
///
/// # Errors
/// Returns error response if:
/// - Authorization header is missing
/// - Token format is invalid (not "Bearer <token>")
/// - Token validation fails
fn extract_user_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    // Get Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(ErrorResponse::new("Missing Authorization header"))
        })?
        .to_str()
        .map_err(|_| {
            HttpResponse::Unauthorized()
                .json(ErrorResponse::new("Invalid Authorization header format"))
        })?;

    // Check Bearer token format
    if !auth_header.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().json(ErrorResponse::new(
            "Authorization header must use Bearer scheme",
        )));
    }

    let token = &auth_header[7..];

    // Validate token and extract claims
    let claims: Claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid or expired token"))
    })?;

    // Parse user ID from subject claim
    claims.sub.parse::<i32>().map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid user ID in token"))
    })
}

/// GET /api/cart - Get user's cart
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Returns
/// - 200 OK with cart data
/// - 401 Unauthorized if token is missing/invalid
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add item to cart
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Request Body
/// ```json
/// {
///   "product_id": 1,
///   "quantity": 2
/// }
/// ```
///
/// # Returns
/// - 200 OK with updated cart
/// - 400 Bad Request if product not found or insufficient inventory
/// - 401 Unauthorized if token is missing/invalid
async fn add_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    payload: web::Json<AddItemRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.add_item(user_id, payload.product_id, payload.quantity) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(crate::cart::service::CartError::ProductNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse::new("Product not found"))
        }
        Err(crate::cart::service::CartError::InsufficientInventory {
            available,
            requested,
        }) => HttpResponse::BadRequest().json(ErrorResponse::new(format!(
            "Insufficient inventory. Available: {available}, Requested: {requested}"
        ))),
        Err(crate::cart::service::CartError::InvalidQuantity) => {
            HttpResponse::BadRequest().json(ErrorResponse::new("Quantity must be greater than 0"))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new("Internal server error"))
        }
    }
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Path Parameters
/// - `product_id`: ID of the product to remove
///
/// # Returns
/// - 200 OK with updated cart
/// - 404 Not Found if cart or item doesn't exist
/// - 401 Unauthorized if token is missing/invalid
async fn remove_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.remove_item(user_id, *product_id) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(crate::cart::service::CartError::CartNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse::new("Cart not found"))
        }
        Err(crate::cart::service::CartError::ItemNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse::new("Item not found in cart"))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new("Internal server error"))
        }
    }
}

/// POST /api/cart/clear - Clear all items from cart
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Returns
/// - 200 OK with empty cart
/// - 404 Not Found if cart doesn't exist
/// - 401 Unauthorized if token is missing/invalid
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.clear_cart(user_id) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(crate::cart::service::CartError::CartNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse::new("Cart not found"))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new("Internal server error"))
        }
    }
}

/// Configure cart routes under /api/cart
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}
