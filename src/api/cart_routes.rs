//! Shopping cart API routes with JWT authentication
//!
//! All endpoints require valid JWT authentication via Authorization header.

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::service::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request to add an item to the cart
#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    /// Product ID to add
    pub product_id: i32,
    /// Quantity to add (must be positive)
    pub quantity: i32,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Extract user ID from JWT token in Authorization header
///
/// # Arguments
///
/// * `req` - The HTTP request containing Authorization header
///
/// # Returns
///
/// `Ok(i32)` with user ID if token is valid, `Err(())` otherwise
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

/// Get the current user's cart
///
/// # Endpoint
/// `GET /api/cart`
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Responses
/// - 200: Cart retrieved successfully
/// - 401: Unauthorized (missing or invalid token)
/// - 404: Cart not found
pub async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    if let Some(cart) = cart_service.get_cart(user_id) {
        HttpResponse::Ok().json(cart)
    } else {
        // Return empty cart if not found
        let empty_cart = cart_service.get_or_create_cart(user_id);
        HttpResponse::Ok().json(empty_cart)
    }
}

/// Add an item to the cart
///
/// # Endpoint
/// `POST /api/cart/add`
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
/// # Responses
/// - 200: Item added successfully
/// - 400: Bad request (invalid product ID, insufficient inventory, or invalid quantity)
/// - 401: Unauthorized (missing or invalid token)
pub async fn add_item(
    req: HttpRequest,
    body: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    // Validate product exists
    let Some(product) = product_service.get_by_id(body.product_id) else {
        let product_id = body.product_id;
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Product with ID {product_id} not found"),
        });
    };

    // Validate quantity is positive
    if body.quantity <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Quantity must be positive".to_string(),
        });
    }

    // Check inventory before adding
    if product.inventory_count < body.quantity {
        let requested = body.quantity;
        let available = product.inventory_count;
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Insufficient inventory: requested {requested}, available {available}"),
        });
    }

    // Add item to cart
    if let Some(cart) = cart_service.add_item(user_id, &product, body.quantity) {
        HttpResponse::Ok().json(cart)
    } else {
        HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to add item to cart (insufficient inventory or invalid quantity)"
                .to_string(),
        })
    }
}

/// Remove an item from the cart
///
/// # Endpoint
/// `DELETE /api/cart/remove/{product_id}`
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Path Parameters
/// - `product_id`: ID of the product to remove
///
/// # Responses
/// - 200: Item removed successfully
/// - 401: Unauthorized (missing or invalid token)
/// - 404: Item not found in cart
pub async fn remove_item(
    req: HttpRequest,
    path: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    let product_id = path.into_inner();

    if let Some(cart) = cart_service.remove_item(user_id, product_id) {
        HttpResponse::Ok().json(cart)
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Item with product ID {product_id} not found in cart"),
        })
    }
}

/// Clear all items from the cart
///
/// # Endpoint
/// `POST /api/cart/clear`
///
/// # Authentication
/// Requires valid JWT token in Authorization header
///
/// # Responses
/// - 200: Cart cleared successfully
/// - 401: Unauthorized (missing or invalid token)
/// - 404: Cart not found
pub async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized: Invalid or missing token".to_string(),
        });
    };

    if let Some(cart) = cart_service.clear_cart(user_id) {
        HttpResponse::Ok().json(cart)
    } else {
        // If no cart exists, create an empty one
        let empty_cart = cart_service.get_or_create_cart(user_id);
        HttpResponse::Ok().json(empty_cart)
    }
}

/// Configure cart routes
///
/// # Arguments
///
/// * `cfg` - Service configuration to attach routes to
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
    use crate::auth::jwt::create_token;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, App};
    use rust_decimal_macros::dec;

    #[actix_web::test]
    async fn test_get_cart_unauthorized() {
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

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_with_valid_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a product
        let _ = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 100,
        });

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id": 1, "quantity": 2}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a product with limited inventory
        let _ = product_service.create(NewProduct {
            name: "Limited Product".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id": 1, "quantity": 10}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_invalid_product() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_payload(r#"{"product_id": 999, "quantity": 1}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_remove_item() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create and add product first
        let product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 100,
        });

        let _ = cart_service.add_item(1, &product, 2);

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
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

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let token = create_token("1").unwrap();
        let req = test::TestRequest::post()
            .uri("/api/cart/clear")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
