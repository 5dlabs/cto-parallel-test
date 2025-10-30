use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::auth::jwt::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;

/// Request DTO for adding an item to the cart
#[derive(Deserialize, Serialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Configures cart-related routes
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart)),
    );
}

/// Extracts and validates JWT token from the Authorization header
///
/// # Arguments
/// * `req` - The HTTP request containing the Authorization header
///
/// # Returns
/// * `Result<i32, HttpResponse>` - The user ID if authentication succeeds, or an error response
fn authenticate_request(req: &HttpRequest) -> Result<i32, HttpResponse> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

    // Convert header to string
    let auth_str = auth_header
        .to_str()
        .map_err(|_| HttpResponse::Unauthorized().finish())?;

    // Check for Bearer prefix
    if !auth_str.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().finish());
    }

    // Extract token
    let token = &auth_str[7..];

    // Validate token
    let claims = validate_token(token).map_err(|_| HttpResponse::Unauthorized().finish())?;

    // Extract user_id from claims.sub
    let user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| HttpResponse::Unauthorized().finish())?;

    Ok(user_id)
}

/// GET /api/cart - Get the user's cart
async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Get or create cart
    let cart = cart_service.get_or_create_cart(user_id);

    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add an item to the cart
async fn add_item(
    req: HttpRequest,
    request: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Validate product exists
    let Some(product) = product_service.get_by_id(request.product_id) else {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found"
        }))
    };

    // Check inventory
    if product.inventory_count < request.quantity {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Not enough inventory"
        }));
    }

    // Get or create cart first
    let _ = cart_service.get_or_create_cart(user_id);

    // Add item to cart
    let cart = cart_service.add_item(user_id, &product, request.quantity);

    HttpResponse::Ok().json(cart)
}

/// DELETE `/api/cart/remove/{product_id}` - Remove an item from the cart
async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Remove item
    match cart_service.remove_item(user_id, *product_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Cart not found"
        })),
    }
}

/// POST /api/cart/clear - Clear all items from the cart
async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    // Authenticate user
    let user_id = match authenticate_request(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Clear cart
    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Cart not found"
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, App};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[actix_web::test]
    async fn test_get_cart_without_auth() {
        let cart_service = web::Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .configure(configure_cart_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/cart").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_auth() {
        let cart_service = web::Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
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
    async fn test_add_item_to_cart() {
        let cart_service = web::Data::new(CartService::new());
        let product_service = web::Data::new(ProductService::new());

        // Create a test product
        let _ = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 10,
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
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
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

        // Create a product with limited inventory
        let _ = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
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
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_json(AddItemRequest {
                product_id: 1,
                quantity: 10,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_nonexistent_product() {
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
            .uri("/cart/add")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .insert_header(("Content-Type", "application/json"))
            .set_json(AddItemRequest {
                product_id: 999,
                quantity: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }
}
