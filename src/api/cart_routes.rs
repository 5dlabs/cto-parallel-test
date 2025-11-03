//! Cart API routes
//!
//! Provides HTTP endpoints for shopping cart operations with JWT authentication.
//! All routes require a valid JWT token in the Authorization header.

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Request payload for adding an item to cart
#[derive(Debug, Deserialize, Serialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Standard error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Custom error type for cart operations
#[derive(Debug)]
pub enum CartError {
    Unauthorized(String),
    BadRequest(String),
    NotFound(String),
}

impl fmt::Display for CartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            Self::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            Self::NotFound(msg) => write!(f, "Not Found: {msg}"),
        }
    }
}

impl error::ResponseError for CartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(ErrorResponse { error: msg.clone() })
            }
            Self::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse { error: msg.clone() })
            }
            Self::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse { error: msg.clone() })
            }
        }
    }
}

/// Configures cart routes for the application
///
/// Mounts all cart endpoints under the configured scope:
/// - `GET /` - Get user's cart
/// - `POST /add` - Add item to cart
/// - `DELETE /remove/{product_id}` - Remove item from cart
/// - `POST /clear` - Clear cart
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}

/// Extracts user ID from JWT token in Authorization header
///
/// # Arguments
///
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
///
/// * `Ok(user_id)` if token is valid and contains a numeric user ID
/// * `Err(CartError)` if authentication fails
fn extract_user_id(req: &HttpRequest) -> Result<i32, CartError> {
    // Get Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| CartError::Unauthorized("Missing Authorization header".to_string()))?;

    // Parse header value
    let auth_str = auth_header
        .to_str()
        .map_err(|_| CartError::Unauthorized("Invalid Authorization header format".to_string()))?;

    // Check for Bearer token
    if !auth_str.starts_with("Bearer ") {
        return Err(CartError::Unauthorized(
            "Authorization header must start with 'Bearer '".to_string(),
        ));
    }

    // Extract token
    let token = &auth_str[7..];

    // Validate token and extract claims
    let claims = validate_token(token)
        .map_err(|_| CartError::Unauthorized("Invalid or expired token".to_string()))?;

    // Parse user ID from claims
    claims
        .sub
        .parse::<i32>()
        .map_err(|_| CartError::Unauthorized("Invalid user ID in token".to_string()))
}

/// GET /cart - Retrieves the user's shopping cart
///
/// Requires valid JWT authentication. Returns the cart or creates an empty one
/// if the user doesn't have a cart yet.
///
/// # Responses
///
/// * `200 OK` - Returns the user's cart
/// * `401 Unauthorized` - Invalid or missing authentication token
async fn get_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
) -> Result<HttpResponse, CartError> {
    let user_id = extract_user_id(&req)?;
    let cart = cart_service.get_or_create_cart(user_id);
    Ok(HttpResponse::Ok().json(cart))
}

/// POST /cart/add - Adds an item to the user's cart
///
/// Validates inventory before adding the item. If the item already exists in the cart,
/// increments the quantity.
///
/// # Request Body
///
/// ```json
/// {
///   "product_id": 1,
///   "quantity": 2
/// }
/// ```
///
/// # Responses
///
/// * `200 OK` - Item added successfully, returns updated cart
/// * `400 Bad Request` - Insufficient inventory or invalid quantity
/// * `401 Unauthorized` - Invalid or missing authentication token
/// * `404 Not Found` - Product not found
async fn add_item(
    req: HttpRequest,
    payload: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> Result<HttpResponse, CartError> {
    let user_id = extract_user_id(&req)?;

    // Validate quantity
    if payload.quantity <= 0 {
        return Err(CartError::BadRequest(
            "Quantity must be positive".to_string(),
        ));
    }

    // Get product
    let product = product_service
        .get_by_id(payload.product_id)
        .ok_or_else(|| {
            CartError::NotFound(format!("Product with ID {} not found", payload.product_id))
        })?;

    // Check inventory
    if product.inventory_count < payload.quantity {
        return Err(CartError::BadRequest(format!(
            "Insufficient inventory. Available: {}, Requested: {}",
            product.inventory_count, payload.quantity
        )));
    }

    // Add item to cart
    let cart = cart_service.add_item(user_id, &product, payload.quantity);
    Ok(HttpResponse::Ok().json(cart))
}

/// DELETE `/cart/remove/{product_id}` - Removes an item from the cart
///
/// Removes the specified product from the user's cart.
///
/// # Path Parameters
///
/// * `product_id` - The ID of the product to remove
///
/// # Responses
///
/// * `200 OK` - Item removed successfully, returns updated cart
/// * `401 Unauthorized` - Invalid or missing authentication token
/// * `404 Not Found` - Cart not found
async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> Result<HttpResponse, CartError> {
    let user_id = extract_user_id(&req)?;

    let cart = cart_service
        .remove_item(user_id, *product_id)
        .ok_or_else(|| CartError::NotFound("Cart not found".to_string()))?;

    Ok(HttpResponse::Ok().json(cart))
}

/// POST /cart/clear - Clears all items from the cart
///
/// Removes all items from the user's cart, leaving an empty cart.
///
/// # Responses
///
/// * `200 OK` - Cart cleared successfully, returns empty cart
/// * `401 Unauthorized` - Invalid or missing authentication token
/// * `404 Not Found` - Cart not found
async fn clear_cart(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
) -> Result<HttpResponse, CartError> {
    let user_id = extract_user_id(&req)?;

    let cart = cart_service
        .clear_cart(user_id)
        .ok_or_else(|| CartError::NotFound("Cart not found".to_string()))?;

    Ok(HttpResponse::Ok().json(cart))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, App};
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;

    fn create_test_product_service() -> ProductService {
        let service = ProductService::new();
        // Add some test products
        let _ = service.create(NewProduct {
            name: "Test Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_f64(19.99).unwrap(),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Test Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::from_f64(29.99).unwrap(),
            inventory_count: 5,
        });
        service
    }

    #[actix_web::test]
    async fn test_get_cart_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::get()
            .uri("/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/cart/add")
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_add_item_with_valid_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 999, // More than inventory
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_invalid_product() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 999,
                quantity: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_remove_item_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/cart/remove/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_clear_cart_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post().uri("/cart/clear").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_cart_isolation() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        // User 1 adds item
        let token1 = create_token("1").unwrap();
        let req1 = test::TestRequest::post()
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token1}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();
        let _ = test::call_service(&app, req1).await;

        // User 2 checks their cart (should be empty)
        let token2 = create_token("2").unwrap();
        let req2 = test::TestRequest::get()
            .uri("/cart")
            .insert_header(("Authorization", format!("Bearer {token2}")))
            .to_request();

        let resp2 = test::call_service(&app, req2).await;
        assert_eq!(resp2.status(), 200);

        let body: crate::cart::Cart = test::read_body_json(resp2).await;
        assert!(body.items.is_empty()); // User 2's cart should be empty
    }
}
