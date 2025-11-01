//! Shopping cart API routes with JWT authentication
//!
//! This module provides HTTP endpoints for cart operations:
//! - GET /api/cart - Get user's cart
//! - POST /api/cart/add - Add item to cart
//! - DELETE /api/cart/remove/{id} - Remove item from cart
//! - POST /api/cart/clear - Clear cart
//!
//! All endpoints require JWT authentication via the Authorization header.

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::auth::validate_token;
use crate::cart::{Cart, CartService};
use crate::catalog::ProductService;

/// Request body for adding items to cart
#[derive(Debug, Serialize, Deserialize)]
pub struct AddItemRequest {
    /// The product ID to add
    pub product_id: i32,
    /// The quantity to add (must be positive)
    pub quantity: i32,
}

/// Request body for updating item quantity
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuantityRequest {
    /// The new quantity (must be positive)
    pub quantity: i32,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Success response with cart data
#[derive(Debug, Serialize)]
pub struct CartResponse {
    /// The user's cart
    pub cart: Cart,
}

/// Extracts user ID from JWT token in Authorization header
///
/// # Errors
///
/// Returns 401 Unauthorized if:
/// - Authorization header is missing
/// - Authorization header format is invalid (must be "Bearer <token>")
/// - Token is invalid or expired
/// - User ID in token cannot be parsed as i32
fn extract_user_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    // Get Authorization header
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Missing Authorization header".to_string(),
        })
    })?;

    // Convert header to string
    let auth_str = auth_header.to_str().map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid Authorization header format".to_string(),
        })
    })?;

    // Check for Bearer prefix
    if !auth_str.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Authorization header must start with 'Bearer '".to_string(),
        }));
    }

    // Extract token
    let token = &auth_str[7..];

    // Validate token
    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or expired token".to_string(),
        })
    })?;

    // Parse user ID from subject claim
    claims.sub.parse::<i32>().map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid user ID in token".to_string(),
        })
    })
}

/// GET /api/cart - Get user's cart
///
/// Returns the authenticated user's shopping cart, or creates an empty one if none exists.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header: `Authorization: Bearer <token>`
///
/// # Responses
///
/// - 200 OK: Returns cart data
/// - 401 Unauthorized: Missing or invalid JWT token
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(CartResponse { cart })
}

/// POST /api/cart/add - Add item to cart
///
/// Adds a product to the authenticated user's cart with the specified quantity.
/// If the item already exists in the cart, the quantity is incremented.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header: `Authorization: Bearer <token>`
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
/// - 200 OK: Item added successfully, returns updated cart
/// - 400 Bad Request: Invalid quantity, product not found, or insufficient inventory
/// - 401 Unauthorized: Missing or invalid JWT token
async fn add_item(
    req: HttpRequest,
    body: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.add_item(user_id, &product_service, body.product_id, body.quantity) {
        Ok(cart) => HttpResponse::Ok().json(CartResponse { cart }),
        Err(err) => {
            let mut status = match err {
                crate::cart::service::CartError::InvalidQuantity => HttpResponse::BadRequest(),
                crate::cart::service::CartError::ProductNotFound
                | crate::cart::service::CartError::CartNotFound => HttpResponse::NotFound(),
                crate::cart::service::CartError::InsufficientInventory { .. } => {
                    HttpResponse::BadRequest()
                }
            };
            status.json(ErrorResponse {
                error: err.to_string(),
            })
        }
    }
}

/// PUT `/api/cart/update/{product_id}` - Update item quantity
///
/// Updates the quantity of an existing item in the cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header: `Authorization: Bearer <token>`
///
/// # Path Parameters
///
/// - `product_id`: The ID of the product to update
///
/// # Request Body
///
/// ```json
/// {
///   "quantity": 5
/// }
/// ```
///
/// # Responses
///
/// - 200 OK: Quantity updated successfully, returns updated cart
/// - 400 Bad Request: Invalid quantity or insufficient inventory
/// - 401 Unauthorized: Missing or invalid JWT token
/// - 404 Not Found: Cart or product not found
async fn update_quantity(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<UpdateQuantityRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let product_id = path.into_inner();

    match cart_service.update_quantity(user_id, &product_service, product_id, body.quantity) {
        Ok(cart) => HttpResponse::Ok().json(CartResponse { cart }),
        Err(err) => {
            let mut status = match err {
                crate::cart::service::CartError::InvalidQuantity => HttpResponse::BadRequest(),
                crate::cart::service::CartError::ProductNotFound
                | crate::cart::service::CartError::CartNotFound => HttpResponse::NotFound(),
                crate::cart::service::CartError::InsufficientInventory { .. } => {
                    HttpResponse::BadRequest()
                }
            };
            status.json(ErrorResponse {
                error: err.to_string(),
            })
        }
    }
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
///
/// Removes an item from the authenticated user's cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header: `Authorization: Bearer <token>`
///
/// # Path Parameters
///
/// - `product_id`: The ID of the product to remove
///
/// # Responses
///
/// - 200 OK: Item removed successfully, returns updated cart
/// - 401 Unauthorized: Missing or invalid JWT token
/// - 404 Not Found: Cart not found
async fn remove_item(
    req: HttpRequest,
    path: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let product_id = path.into_inner();

    match cart_service.remove_item(user_id, product_id) {
        Ok(cart) => HttpResponse::Ok().json(CartResponse { cart }),
        Err(err) => HttpResponse::NotFound().json(ErrorResponse {
            error: err.to_string(),
        }),
    }
}

/// POST /api/cart/clear - Clear cart
///
/// Removes all items from the authenticated user's cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header: `Authorization: Bearer <token>`
///
/// # Responses
///
/// - 200 OK: Cart cleared successfully, returns empty cart
/// - 401 Unauthorized: Missing or invalid JWT token
/// - 404 Not Found: Cart not found
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.clear_cart(user_id) {
        Ok(cart) => HttpResponse::Ok().json(CartResponse { cart }),
        Err(err) => HttpResponse::NotFound().json(ErrorResponse {
            error: err.to_string(),
        }),
    }
}

/// Configures cart API routes
///
/// Mounts all cart-related endpoints under the configured scope.
/// Typically called with scope "/api/cart".
///
/// # Routes
///
/// - `GET ""` → `get_cart`
/// - `POST "/add"` → `add_item`
/// - `PUT "/update/{product_id}"` → `update_quantity`
/// - `DELETE "/remove/{product_id}"` → `remove_item`
/// - `POST "/clear"` → `clear_cart`
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/update/{product_id}", web::put().to(update_quantity))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::create_token;
    use crate::catalog::{NewProduct, ProductService};
    use actix_web::{http, test, App};
    use rust_decimal_macros::dec;

    fn create_test_product_service() -> ProductService {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Test Laptop".to_string(),
            description: "A test laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Test Mouse".to_string(),
            description: "A test mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });
        service
    }

    #[actix_web::test]
    async fn test_get_cart_without_auth() {
        let cart_service = web::Data::new(CartService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_auth() {
        let cart_service = web::Data::new(CartService::new());
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_add_item_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_add_item_with_valid_auth() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 100,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_add_item_product_not_found() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 999,
                quantity: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_remove_item() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());
        let token = create_token("1").expect("Failed to create token");

        // First add an item
        cart_service
            .add_item(1, &product_service, 1, 2)
            .expect("Failed to add item");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/api/cart/remove/1")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_clear_cart() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());
        let token = create_token("1").expect("Failed to create token");

        // Add items
        cart_service
            .add_item(1, &product_service, 1, 2)
            .expect("Failed to add item");
        cart_service
            .add_item(1, &product_service, 2, 3)
            .expect("Failed to add item");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/clear")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_isolation() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let token1 = create_token("1").expect("Failed to create token");
        let token2 = create_token("2").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        // User 1 adds item
        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token1}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();
        let _ = test::call_service(&app, req).await;

        // User 2 gets cart - should be empty or different
        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token2}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
