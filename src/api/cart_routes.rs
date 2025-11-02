//! Shopping cart API routes with JWT authentication.

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request payload for adding items to cart.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddItemRequest {
    /// Product ID to add
    pub product_id: i32,
    /// Quantity to add
    pub quantity: i32,
}

/// Response for successful operations.
#[derive(Debug, Serialize)]
pub struct CartResponse {
    /// Success message
    pub message: String,
    /// Cart data
    pub cart: crate::cart::Cart,
}

/// Error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Extracts user ID from JWT token in Authorization header.
///
/// # Arguments
///
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
///
/// `Ok(user_id)` if authentication succeeds, `Err(HttpResponse)` with 401 otherwise
fn extract_user_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = validate_token(token) {
                    if let Ok(user_id) = claims.sub.parse::<i32>() {
                        return Ok(user_id);
                    }
                }
            }
        }
    }

    Err(HttpResponse::Unauthorized().json(ErrorResponse {
        error: "Invalid or missing authentication token".to_string(),
    }))
}

/// `GET /api/cart` - Get user's shopping cart.
///
/// Requires JWT authentication via Authorization header.
///
/// # Returns
///
/// - `200 OK` with cart data if successful
/// - `401 Unauthorized` if authentication fails
#[get("")]
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_or_create_cart(user_id);

    HttpResponse::Ok().json(cart)
}

/// `POST /api/cart/add` - Add item to cart.
///
/// Requires JWT authentication via Authorization header.
/// Validates product exists and has sufficient inventory.
///
/// # Returns
///
/// - `200 OK` with updated cart if successful
/// - `400 Bad Request` if insufficient inventory
/// - `401 Unauthorized` if authentication fails
/// - `404 Not Found` if product doesn't exist
#[post("/add")]
async fn add_item(
    req: HttpRequest,
    payload: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Validate product exists
    let Some(product) = product_service.get_by_id(payload.product_id) else {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Product with ID {} not found", payload.product_id),
        });
    };

    // Validate inventory
    if product.inventory_count < payload.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!(
                "Insufficient inventory. Available: {}, Requested: {}",
                product.inventory_count, payload.quantity
            ),
        });
    }

    // Add to cart
    let cart = cart_service.add_item(user_id, &product, payload.quantity);

    HttpResponse::Ok().json(CartResponse {
        message: "Item added to cart successfully".to_string(),
        cart,
    })
}

/// `DELETE /api/cart/remove/{product_id}` - Remove item from cart.
///
/// Requires JWT authentication via Authorization header.
///
/// # Returns
///
/// - `200 OK` with updated cart if successful
/// - `401 Unauthorized` if authentication fails
/// - `404 Not Found` if cart doesn't exist
#[delete("/remove/{product_id}")]
async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.remove_item(user_id, *product_id) {
        Some(cart) => HttpResponse::Ok().json(CartResponse {
            message: "Item removed from cart successfully".to_string(),
            cart,
        }),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Cart not found".to_string(),
        }),
    }
}

/// `POST /api/cart/clear` - Clear all items from cart.
///
/// Requires JWT authentication via Authorization header.
///
/// # Returns
///
/// - `200 OK` with empty cart if successful
/// - `401 Unauthorized` if authentication fails
/// - `404 Not Found` if cart doesn't exist
#[post("/clear")]
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(CartResponse {
            message: "Cart cleared successfully".to_string(),
            cart,
        }),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Cart not found".to_string(),
        }),
    }
}

/// Configures cart routes under `/api/cart` scope.
///
/// # Routes
///
/// - `GET /api/cart` - Get cart
/// - `POST /api/cart/add` - Add item
/// - `DELETE /api/cart/remove/{product_id}` - Remove item
/// - `POST /api/cart/clear` - Clear cart
///
/// All routes require JWT authentication.
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/cart")
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
    use crate::catalog::models::{NewProduct, Product};
    use actix_web::{test, web, App};
    use rust_decimal_macros::dec;

    fn create_test_product(
        service: &ProductService,
        _id: i32,
        name: &str,
        price: rust_decimal::Decimal,
        inventory: i32,
    ) -> Product {
        service.create(NewProduct {
            name: name.to_string(),
            description: format!("Description for {name}"),
            price,
            inventory_count: inventory,
        })
    }

    #[actix_web::test]
    async fn test_get_cart_without_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
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
    async fn test_add_item_without_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
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
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_add_item_with_valid_token_and_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product
        let _ = create_test_product(&product_service, 1, "Test Product", dec!(19.99), 10);

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
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
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product with limited inventory
        let _ = create_test_product(&product_service, 1, "Test Product", dec!(19.99), 1);

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 10,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_nonexistent_product() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
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
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_remove_item() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product and add it to cart
        let product = create_test_product(&product_service, 1, "Test Product", dec!(19.99), 10);
        let _ = cart_service.add_item(1, &product, 2);

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/api/cart/remove/1")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_clear_cart() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product and add it to cart
        let product = create_test_product(&product_service, 1, "Test Product", dec!(19.99), 10);
        let _ = cart_service.add_item(1, &product, 2);

        let token = create_token("1").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
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

    #[actix_web::test]
    async fn test_cart_isolation() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product
        let product = create_test_product(&product_service, 1, "Test Product", dec!(19.99), 10);

        // Add item to user 1's cart
        let _ = cart_service.add_item(1, &product, 2);

        // Try to access with user 2's token
        let token = create_token("2").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // User 2 should have an empty cart
        let body: crate::cart::Cart = test::read_body_json(resp).await;
        assert_eq!(body.user_id, 2);
        assert!(body.items.is_empty());
    }
}
