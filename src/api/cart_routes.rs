//! Shopping cart API routes with JWT authentication.

use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request payload for adding an item to the cart.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddItemRequest {
    /// Product ID to add
    pub product_id: i32,
    /// Quantity to add
    pub quantity: i32,
}

/// Error response structure.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

impl ErrorResponse {
    /// Creates a new error response.
    fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

/// Extracts user ID from JWT token in the Authorization header.
///
/// # Arguments
///
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
///
/// `Ok(user_id)` if authentication succeeds, `Err(HttpResponse)` otherwise
fn extract_user_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(ErrorResponse::new("Missing Authorization header"))
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(
            HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid Authorization format"))
        );
    }

    let token = &auth_header[7..];
    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid or expired token"))
    })?;

    claims.sub.parse::<i32>().map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid user ID in token"))
    })
}

/// GET /api/cart - Get the current user's cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header.
///
/// # Returns
///
/// - 200 OK with cart data if successful
/// - 401 Unauthorized if authentication fails
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add an item to the cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header.
///
/// # Request Body
///
/// JSON object with `product_id` and `quantity` fields.
///
/// # Returns
///
/// - 200 OK with updated cart if successful
/// - 400 Bad Request if product not found or insufficient inventory
/// - 401 Unauthorized if authentication fails
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

    // Validate quantity is positive
    if payload.quantity <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Quantity must be positive"));
    }

    // Get product from catalog
    let Some(product) = product_service.get_by_id(payload.product_id) else {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Product not found"));
    };

    // Check inventory
    if product.inventory_count < payload.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
            "Insufficient inventory. Available: {}, Requested: {}",
            product.inventory_count, payload.quantity
        )));
    }

    // Add to cart
    let cart = cart_service.add_item(user_id, &product, payload.quantity);
    HttpResponse::Ok().json(cart)
}

/// DELETE `/api/cart/remove/{product_id}` - Remove an item from the cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header.
///
/// # Path Parameters
///
/// - `product_id`: The ID of the product to remove
///
/// # Returns
///
/// - 200 OK with updated cart if successful
/// - 401 Unauthorized if authentication fails
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
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::Ok().json(cart_service.get_or_create_cart(user_id)),
    }
}

/// POST /api/cart/clear - Clear all items from the cart.
///
/// # Authentication
///
/// Requires valid JWT token in Authorization header.
///
/// # Returns
///
/// - 200 OK with empty cart if successful
/// - 401 Unauthorized if authentication fails
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::Ok().json(cart_service.get_or_create_cart(user_id)),
    }
}

/// Configure cart API routes.
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_cart))
        .route("/add", web::post().to(add_item))
        .route("/remove/{product_id}", web::delete().to(remove_item))
        .route("/clear", web::post().to(clear_cart));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::create_token;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, App};
    use rust_decimal_macros::dec;

    fn create_test_product_service() -> ProductService {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Test Laptop".to_string(),
            description: "A test laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });
        service
    }

    #[actix_web::test]
    async fn test_get_cart_without_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_valid_token() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
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
    async fn test_add_item_success() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(&AddItemRequest {
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

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(&AddItemRequest {
                product_id: 1,
                quantity: 100, // More than available
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_invalid_product() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(&AddItemRequest {
                product_id: 999, // Non-existent product
                quantity: 1,
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_remove_item() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(create_test_product_service());

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
        )
        .await;

        // First add an item
        let _ = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(&AddItemRequest {
                product_id: 1,
                quantity: 2,
            })
            .to_request();

        // Then remove it
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
        let product_service = web::Data::new(create_test_product_service());

        let token = create_token(1).expect("Failed to create token");

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .configure(|cfg| {
                    cfg.service(web::scope("/api/cart").configure(configure_cart_routes));
                }),
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
