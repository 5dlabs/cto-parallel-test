//! Shopping cart API routes with JWT authentication
//!
//! All endpoints require a valid JWT token in the Authorization header.
//! Returns 401 Unauthorized for missing or invalid tokens.

use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request payload for adding an item to cart
#[derive(Debug, Deserialize, Serialize)]
pub struct AddItemRequest {
    /// Product ID to add
    pub product_id: i32,
    /// Quantity to add (must be positive)
    pub quantity: i32,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Extracts user ID from JWT token in Authorization header
///
/// # Arguments
///
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
///
/// Returns `Ok(user_id)` if token is valid, or `Err(())` if token is missing or invalid
fn extract_user_id(req: &HttpRequest) -> Result<i32, ()> {
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
    Err(())
}

/// GET /api/cart - Get user's shopping cart
///
/// Requires JWT authentication. Returns the user's cart with all items.
///
/// # Responses
///
/// * `200 OK` - Returns the cart (may be empty)
/// * `401 Unauthorized` - Missing or invalid JWT token
pub async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add item to cart
///
/// Requires JWT authentication. Adds a product to the cart with the specified quantity.
/// If the product is already in the cart, increments the quantity.
/// Validates inventory before adding.
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
/// * `400 Bad Request` - Invalid quantity, insufficient inventory, or product not found
/// * `401 Unauthorized` - Missing or invalid JWT token
/// * `404 Not Found` - Product not found
pub async fn add_item(
    req: HttpRequest,
    request: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    // Validate quantity
    if request.quantity <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Quantity must be positive".to_string(),
        });
    }

    // Get product from catalog
    let Some(product) = product_service.get_by_id(request.product_id) else {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Product with id {} not found", request.product_id),
        });
    };

    // Check inventory
    if product.inventory_count < request.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!(
                "Insufficient inventory. Available: {}, Requested: {}",
                product.inventory_count, request.quantity
            ),
        });
    }

    // Add to cart
    match cart_service.add_item(user_id, &product, request.quantity) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to add item to cart. This may occur if adding would exceed available inventory.".to_string(),
        }),
    }
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
///
/// Requires JWT authentication. Removes a product completely from the cart.
///
/// # Path Parameters
///
/// * `product_id` - The product ID to remove
///
/// # Responses
///
/// * `200 OK` - Item removed successfully, returns updated cart
/// * `401 Unauthorized` - Missing or invalid JWT token
/// * `404 Not Found` - Cart not found for user
pub async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    match cart_service.remove_item(user_id, *product_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Cart not found".to_string(),
        }),
    }
}

/// POST /api/cart/clear - Clear all items from cart
///
/// Requires JWT authentication. Removes all items from the user's cart.
/// The cart itself remains but becomes empty.
///
/// # Responses
///
/// * `200 OK` - Cart cleared successfully, returns empty cart
/// * `401 Unauthorized` - Missing or invalid JWT token
/// * `404 Not Found` - Cart not found for user
pub async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Cart not found".to_string(),
        }),
    }
}

/// Configures cart routes for the Actix-Web application
///
/// # Routes
///
/// * `GET /api/cart` - Get cart
/// * `POST /api/cart/add` - Add item
/// * `DELETE /api/cart/remove/{product_id}` - Remove item
/// * `POST /api/cart/clear` - Clear cart
///
/// All routes require JWT authentication.
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::create_token;
    use crate::cart::Cart;
    use crate::catalog::{NewProduct, ProductService};
    use actix_web::{test, App};
    use rust_decimal_macros::dec;

    fn setup_services() -> (ProductService, CartService) {
        let product_service = ProductService::new();
        let cart_service = CartService::new();

        // Add test products
        let _ = product_service.create(NewProduct {
            name: "Test Product 1".to_string(),
            description: "Description 1".to_string(),
            price: dec!(10.99),
            inventory_count: 10,
        });

        let _ = product_service.create(NewProduct {
            name: "Test Product 2".to_string(),
            description: "Description 2".to_string(),
            price: dec!(20.99),
            inventory_count: 5,
        });

        (product_service, cart_service)
    }

    #[actix_web::test]
    async fn test_get_cart_without_auth() {
        let (product_service, cart_service) = setup_services();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_token() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
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
    async fn test_add_item_with_valid_token() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id":1,"quantity":2}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id":1,"quantity":100}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_invalid_product() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id":999,"quantity":1}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_remove_item_with_valid_token() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        // First add an item
        let product = product_service.get_by_id(1).unwrap();
        let _ = cart_service.add_item(1, &product, 2);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
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
    async fn test_clear_cart_with_valid_token() {
        let (product_service, cart_service) = setup_services();
        let token = create_token("1").expect("Failed to create token");

        // Add items first
        let product = product_service.get_by_id(1).unwrap();
        let _ = cart_service.add_item(1, &product, 2);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service))
                .app_data(web::Data::new(product_service))
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
    async fn test_user_isolation() {
        let (product_service, cart_service) = setup_services();
        let token1 = create_token("1").expect("Failed to create token");
        let token2 = create_token("2").expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(cart_service.clone()))
                .app_data(web::Data::new(product_service.clone()))
                .configure(configure_cart_routes),
        )
        .await;

        // User 1 adds item
        let req1 = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token1}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id":1,"quantity":2}"#)
            .to_request();

        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), 200);

        // User 2 gets their cart (should be empty)
        let req2 = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token2}")))
            .to_request();

        let resp2 = test::call_service(&app, req2).await;
        assert_eq!(resp2.status(), 200);

        let body: Cart = test::read_body_json(resp2).await;
        assert!(body.items.is_empty());
    }
}
