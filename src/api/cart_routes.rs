//! Shopping Cart API Routes
//!
//! This module provides HTTP API endpoints for cart operations.
//! All endpoints require JWT authentication via Bearer token.

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request payload for adding items to cart
#[derive(Debug, Serialize, Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Request payload for updating item quantity
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub quantity: i32,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    #[must_use]
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

/// Extracts user ID from JWT token in Authorization header
///
/// # Arguments
///
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
///
/// `Ok(user_id)` if token is valid, `Err(())` otherwise
fn extract_user_id(req: &HttpRequest) -> Result<i32, ()> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = validate_token(token) {
                    return claims.sub.parse::<i32>().map_err(|_| ());
                }
            }
        }
    }
    Err(())
}

/// GET /api/cart - Get user's shopping cart
///
/// Requires JWT authentication. Returns the cart with all items.
///
/// # Responses
///
/// * 200 OK - Cart retrieved successfully
/// * 401 Unauthorized - Missing or invalid JWT token
pub async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized()
            .json(ErrorResponse::new("Unauthorized: Invalid or missing token"));
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/items - Add item to cart
///
/// Requires JWT authentication. Validates product exists and has sufficient inventory
/// before adding to cart.
///
/// # Request Body
///
/// ```json
/// {
///     "product_id": 1,
///     "quantity": 2
/// }
/// ```
///
/// # Responses
///
/// * 200 OK - Item added successfully
/// * 400 Bad Request - Invalid request or insufficient inventory
/// * 401 Unauthorized - Missing or invalid JWT token
/// * 404 Not Found - Product not found
pub async fn add_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
    add_request: web::Json<AddItemRequest>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized()
            .json(ErrorResponse::new("Unauthorized: Invalid or missing token"));
    };

    // Validate quantity
    if add_request.quantity <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Quantity must be positive"));
    }

    // Get product and validate it exists
    let Some(product) = product_service.get_by_id(add_request.product_id) else {
        return HttpResponse::NotFound().json(ErrorResponse::new("Product not found"));
    };

    // Validate inventory
    if product.inventory_count < add_request.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
            "Insufficient inventory. Available: {}, Requested: {}",
            product.inventory_count, add_request.quantity
        )));
    }

    // Add item to cart
    let cart = cart_service.add_item(user_id, &product, add_request.quantity);
    HttpResponse::Ok().json(cart)
}

/// PUT `/api/cart/items/{product_id}` - Update item quantity in cart
///
/// Requires JWT authentication. Updates the quantity of an existing item in the cart.
///
/// # Path Parameters
///
/// * `product_id` - The ID of the product to update
///
/// # Request Body
///
/// ```json
/// {
///     "quantity": 5
/// }
/// ```
///
/// # Responses
///
/// * 200 OK - Item quantity updated successfully
/// * 400 Bad Request - Invalid quantity
/// * 401 Unauthorized - Missing or invalid JWT token
/// * 404 Not Found - Product not in cart
pub async fn update_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
    product_id: web::Path<i32>,
    update_request: web::Json<UpdateItemRequest>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized()
            .json(ErrorResponse::new("Unauthorized: Invalid or missing token"));
    };

    // Validate quantity
    if update_request.quantity <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Quantity must be positive"));
    }

    // Verify product exists
    let Some(product) = product_service.get_by_id(*product_id) else {
        return HttpResponse::NotFound().json(ErrorResponse::new("Product not found"));
    };

    // Validate inventory
    if product.inventory_count < update_request.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
            "Insufficient inventory. Available: {}, Requested: {}",
            product.inventory_count, update_request.quantity
        )));
    }

    // Remove existing item and add with new quantity
    let _ = cart_service.remove_item(user_id, *product_id);
    let cart = cart_service.add_item(user_id, &product, update_request.quantity);

    HttpResponse::Ok().json(cart)
}

/// DELETE `/api/cart/items/{product_id}` - Remove item from cart
///
/// Requires JWT authentication. Removes a specific product from the cart.
///
/// # Path Parameters
///
/// * `product_id` - The ID of the product to remove
///
/// # Responses
///
/// * 200 OK - Item removed successfully
/// * 401 Unauthorized - Missing or invalid JWT token
/// * 404 Not Found - Cart not found
pub async fn remove_item(
    req: HttpRequest,
    cart_service: web::Data<CartService>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized()
            .json(ErrorResponse::new("Unauthorized: Invalid or missing token"));
    };

    match cart_service.remove_item(user_id, *product_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Cart not found")),
    }
}

/// DELETE /api/cart - Clear all items from cart
///
/// Requires JWT authentication. Removes all items from the user's cart.
///
/// # Responses
///
/// * 200 OK - Cart cleared successfully
/// * 401 Unauthorized - Missing or invalid JWT token
/// * 404 Not Found - Cart not found
pub async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized()
            .json(ErrorResponse::new("Unauthorized: Invalid or missing token"));
    };

    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Cart not found")),
    }
}

/// Configures cart routes for the Actix-web application
///
/// # Example
///
/// ```no_run
/// use actix_web::{web, App};
/// use cto_parallel_test::api::configure_cart_routes;
/// use cto_parallel_test::cart::CartService;
/// use cto_parallel_test::catalog::ProductService;
///
/// let cart_service = web::Data::new(CartService::new());
/// let product_service = web::Data::new(ProductService::new());
///
/// App::new()
///     .app_data(cart_service)
///     .app_data(product_service)
///     .service(web::scope("/api/cart").configure(configure_cart_routes));
/// ```
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_cart))
        .route("/items", web::post().to(add_item))
        .route("/items/{product_id}", web::put().to(update_item))
        .route("/items/{product_id}", web::delete().to(remove_item))
        .route("", web::delete().to(clear_cart));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, App};
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;

    #[actix_web::test]
    async fn test_get_cart_unauthorized() {
        let cart_service = web::Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart", web::get().to(get_cart)),
        )
        .await;

        let req = test::TestRequest::get().uri("/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_token() {
        let cart_service = web::Data::new(CartService::new());
        let token = create_token(1).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart", web::get().to(get_cart)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_unauthorized() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .route("/cart/items", web::post().to(add_item)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/cart/items")
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_add_item_product_not_found() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());
        let token = create_token(1).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .route("/cart/items", web::post().to(add_item)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/cart/items")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(AddItemRequest {
                product_id: 999,
                quantity: 2,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create product with limited inventory
        let _product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_f64(10.0).unwrap(),
            inventory_count: 5,
        });

        let token = create_token(1).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .route("/cart/items", web::post().to(add_item)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/cart/items")
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
    async fn test_add_item_success() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create product
        let _product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_f64(10.0).unwrap(),
            inventory_count: 10,
        });

        let token = create_token(1).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .route("/cart/items", web::post().to(add_item)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/cart/items")
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
    async fn test_remove_item_unauthorized() {
        let cart_service = web::Data::new(CartService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart/items/{product_id}", web::delete().to(remove_item)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/cart/items/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_remove_item_success() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create product and add to cart
        let product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_f64(10.0).unwrap(),
            inventory_count: 10,
        });

        let user_id = 1;
        let _ = cart_service.add_item(user_id, &product, 2);
        let token = create_token(user_id).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart/items/{product_id}", web::delete().to(remove_item)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/cart/items/1")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_clear_cart_unauthorized() {
        let cart_service = web::Data::new(CartService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart", web::delete().to(clear_cart)),
        )
        .await;

        let req = test::TestRequest::delete().uri("/cart").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_clear_cart_success() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create product and add to cart
        let product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_f64(10.0).unwrap(),
            inventory_count: 10,
        });

        let user_id = 1;
        let _ = cart_service.add_item(user_id, &product, 2);
        let token = create_token(user_id).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .route("/cart", web::delete().to(clear_cart)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/cart")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
