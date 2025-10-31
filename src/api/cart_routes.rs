//! Cart API routes with JWT authentication
//!
//! This module provides HTTP endpoints for shopping cart operations:
//! - GET /api/cart - Get user's cart
//! - POST /api/cart/add - Add item to cart
//! - DELETE `/api/cart/remove/{product_id}` - Remove item from cart
//! - POST /api/cart/clear - Clear cart
//!
//! All endpoints require JWT authentication via Authorization header.

use crate::auth::jwt::{extract_user_id, validate_token};
use crate::cart::service::{AddItemRequest, CartError, CartService};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

/// Extracts user ID from JWT token in Authorization header
///
/// # Arguments
///
/// * `req` - HTTP request with Authorization header
///
/// # Returns
///
/// User ID if token is valid
///
/// # Errors
///
/// Returns 401 Unauthorized if:
/// - Authorization header is missing
/// - Token format is invalid (must be "Bearer <token>")
/// - Token validation fails
fn extract_user_from_request(req: &HttpRequest) -> Result<i32, HttpResponse> {
    // Get Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(json!({
                "error": "Missing Authorization header"
            }))
        })?;

    // Extract token from "Bearer <token>" format
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        HttpResponse::Unauthorized().json(json!({
            "error": "Invalid Authorization header format. Expected: Bearer <token>"
        }))
    })?;

    // Validate token and extract claims
    let claims = validate_token(token).map_err(|e| {
        HttpResponse::Unauthorized().json(json!({
            "error": format!("Invalid token: {e}")
        }))
    })?;

    // Extract user ID from claims
    extract_user_id(&claims).map_err(|e| {
        HttpResponse::Unauthorized().json(json!({
            "error": format!("Invalid token claims: {e}")
        }))
    })
}

/// Converts `CartError` to HTTP response
fn cart_error_to_response(error: CartError) -> HttpResponse {
    match error {
        CartError::ProductNotFound(id) => HttpResponse::NotFound().json(json!({
            "error": format!("Product not found: {id}")
        })),
        CartError::InsufficientInventory {
            product_id,
            requested,
            available,
        } => HttpResponse::BadRequest().json(json!({
            "error": format!("Insufficient inventory for product {product_id}"),
            "requested": requested,
            "available": available
        })),
        CartError::CartItemNotFound(id) => HttpResponse::NotFound().json(json!({
            "error": format!("Cart item not found: {id}")
        })),
        CartError::InvalidQuantity(qty) => HttpResponse::BadRequest().json(json!({
            "error": format!("Invalid quantity: {qty}. Quantity must be positive.")
        })),
        CartError::DatabaseError(msg) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {msg}")
        })),
    }
}

/// GET /api/cart - Get user's shopping cart
///
/// Returns the user's cart with all items and product details.
///
/// # Authentication
///
/// Requires JWT token in Authorization header.
///
/// # Returns
///
/// - 200 OK: Cart with items
/// - 401 Unauthorized: Missing or invalid token
/// - 500 Internal Server Error: Database error
#[get("/cart")]
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_from_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.get_cart(user_id) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => cart_error_to_response(e),
    }
}

/// POST /api/cart/add - Add item to cart
///
/// Adds an item to the user's cart with inventory validation.
///
/// # Authentication
///
/// Requires JWT token in Authorization header.
///
/// # Request Body
///
/// ```json
/// {
///   "product_id": 123,
///   "quantity": 2
/// }
/// ```
///
/// # Returns
///
/// - 200 OK: Cart with updated items
/// - 400 Bad Request: Invalid quantity or insufficient inventory
/// - 401 Unauthorized: Missing or invalid token
/// - 404 Not Found: Product not found
/// - 500 Internal Server Error: Database error
#[post("/cart/add")]
async fn add_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    request: web::Json<AddItemRequest>,
) -> impl Responder {
    let user_id = match extract_user_from_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.add_item(user_id, &request.into_inner()) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => cart_error_to_response(e),
    }
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
///
/// Removes an item from the user's cart.
///
/// # Authentication
///
/// Requires JWT token in Authorization header.
///
/// # Path Parameters
///
/// - `product_id`: ID of the product to remove
///
/// # Returns
///
/// - 200 OK: Cart with remaining items
/// - 401 Unauthorized: Missing or invalid token
/// - 404 Not Found: Cart item not found
/// - 500 Internal Server Error: Database error
#[delete("/cart/remove/{product_id}")]
async fn remove_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let user_id = match extract_user_from_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.remove_item(user_id, *product_id) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => cart_error_to_response(e),
    }
}

/// POST /api/cart/clear - Clear all items from cart
///
/// Removes all items from the user's cart.
///
/// # Authentication
///
/// Requires JWT token in Authorization header.
///
/// # Returns
///
/// - 200 OK: Empty cart
/// - 401 Unauthorized: Missing or invalid token
/// - 500 Internal Server Error: Database error
#[post("/cart/clear")]
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_from_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.clear_cart(user_id) {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => cart_error_to_response(e),
    }
}

/// Configures cart routes for the application
///
/// # Arguments
///
/// * `cfg` - Service configuration
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_cart)
            .service(add_item)
            .service(remove_item)
            .service(clear_cart),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;
    use crate::config::establish_connection_pool;
    use crate::models::{NewProduct, NewUser};
    use crate::schema::{products, users};
    use actix_web::{test, App};
    use diesel::prelude::*;
    use rust_decimal_macros::dec;
    use std::sync::Arc;

    fn setup_test_db() -> Arc<crate::config::Pool> {
        dotenv::dotenv().ok();
        Arc::new(establish_connection_pool())
    }

    fn create_test_user(pool: &crate::config::Pool, username: &str) -> i32 {
        let mut conn = pool.get().expect("Failed to get connection");
        let new_user = NewUser {
            username: username.to_string(),
            email: format!("{username}@test.com"),
            password_hash: "test_hash".to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(users::id)
            .get_result(&mut conn)
            .expect("Failed to create test user")
    }

    fn create_test_product(pool: &crate::config::Pool, name: &str, inventory: i32) -> i32 {
        let mut conn = pool.get().expect("Failed to get connection");
        let new_product = NewProduct {
            name: name.to_string(),
            description: Some(format!("Test product: {name}")),
            price: dec!(99.99),
            inventory_count: inventory,
        };

        diesel::insert_into(products::table)
            .values(&new_product)
            .returning(products::id)
            .get_result(&mut conn)
            .expect("Failed to create test product")
    }

    #[actix_web::test]
    async fn test_get_cart_success() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "api_test_user_1");
        let token = create_token(user_id).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_get_cart_missing_token() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_invalid_token() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", "Bearer invalid.token.here"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_add_item_success() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "api_test_user_2");
        let product_id = create_test_product(&pool, "API Test Product", 10);
        let token = create_token(user_id).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(json!({ "product_id": product_id, "quantity": 2 }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "api_test_user_3");
        let product_id = create_test_product(&pool, "Limited Stock", 5);
        let token = create_token(user_id).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(json!({ "product_id": product_id, "quantity": 10 }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_remove_item_success() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "api_test_user_4");
        let product_id = create_test_product(&pool, "Remove Test", 10);
        let token = create_token(user_id).expect("Failed to create token");

        // Add item first
        cart_service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id,
                    quantity: 2,
                },
            )
            .expect("Failed to add item");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri(&format!("/api/cart/remove/{product_id}"))
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_clear_cart_success() {
        let pool = setup_test_db();
        let cart_service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "api_test_user_5");
        let product_id = create_test_product(&pool, "Clear Test", 10);
        let token = create_token(user_id).expect("Failed to create token");

        // Add item first
        cart_service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id,
                    quantity: 2,
                },
            )
            .expect("Failed to add item");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/clear")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
