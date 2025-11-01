use crate::auth::validate_token;
use crate::cart::CartService;
use crate::catalog::ProductService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request body for adding an item to cart
#[derive(Debug, Deserialize, Serialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Extracts user ID from JWT token in Authorization header
///
/// # Arguments
/// * `req` - The HTTP request containing Authorization header
///
/// # Returns
/// `Ok(i32)` with user ID if valid token, `Err(())` if invalid or missing
fn extract_user_id(req: &HttpRequest) -> Result<i32, ()> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = validate_token(token) {
                    return Ok(claims.sub.parse::<i32>().unwrap_or(0));
                }
            }
        }
    }
    Err(())
}

/// GET /api/cart - Get user's cart
pub async fn get_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing authorization token".to_string(),
        });
    };

    let cart = cart_service.get_or_create_cart(user_id);
    HttpResponse::Ok().json(cart)
}

/// POST /api/cart/add - Add item to cart
pub async fn add_item(
    req: HttpRequest,
    body: web::Json<AddItemRequest>,
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing authorization token".to_string(),
        });
    };

    // Validate product exists
    let Some(product) = product_service.get_by_id(body.product_id) else {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Product with ID {} not found", body.product_id),
        });
    };

    // Validate inventory
    if product.inventory_count < body.quantity {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!(
                "Insufficient inventory. Available: {}, requested: {}",
                product.inventory_count, body.quantity
            ),
        });
    }

    // Add to cart
    let cart = cart_service.add_item(user_id, &product, body.quantity);
    HttpResponse::Ok().json(cart)
}

/// DELETE `/api/cart/remove/{product_id}` - Remove item from cart
pub async fn remove_item(
    req: HttpRequest,
    product_id: web::Path<i32>,
    cart_service: web::Data<CartService>,
) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing authorization token".to_string(),
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
pub async fn clear_cart(req: HttpRequest, cart_service: web::Data<CartService>) -> impl Responder {
    let Ok(user_id) = extract_user_id(&req) else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid or missing authorization token".to_string(),
        });
    };

    match cart_service.clear_cart(user_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Cart not found".to_string(),
        }),
    }
}

/// Configure cart routes
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_cart))
        .route("/add", web::post().to(add_item))
        .route("/remove/{product_id}", web::delete().to(remove_item))
        .route("/clear", web::post().to(clear_cart));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::models::NewProduct;
    use actix_web::{test, web::Data, App};
    use rust_decimal_macros::dec;

    #[actix_web::test]
    async fn test_get_cart_no_auth() {
        let cart_service = Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/cart").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_get_cart_with_auth() {
        let cart_service = Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", "Bearer user_1"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_add_item_no_auth() {
        let cart_service = Data::new(CartService::new());
        let product_service = Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let add_req = AddItemRequest {
            product_id: 1,
            quantity: 2,
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .set_json(&add_req)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_add_item_product_not_found() {
        let cart_service = Data::new(CartService::new());
        let product_service = Data::new(ProductService::new());

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let add_req = AddItemRequest {
            product_id: 999,
            quantity: 2,
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", "Bearer user_1"))
            .set_json(&add_req)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_add_item_insufficient_inventory() {
        let cart_service = Data::new(CartService::new());
        let product_service = Data::new(ProductService::new());

        // Create product with limited inventory
        let product = product_service.create(NewProduct {
            name: "Limited Stock".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let add_req = AddItemRequest {
            product_id: product.id,
            quantity: 10, // More than available
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", "Bearer user_1"))
            .set_json(&add_req)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_add_item_success() {
        let cart_service = Data::new(CartService::new());
        let product_service = Data::new(ProductService::new());

        // Create product
        let product = product_service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let add_req = AddItemRequest {
            product_id: product.id,
            quantity: 2,
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", "Bearer user_1"))
            .set_json(&add_req)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_remove_item_no_auth() {
        let cart_service = Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/api/cart/remove/1")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_clear_cart_no_auth() {
        let cart_service = Data::new(CartService::new());
        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/cart/clear")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_cart_isolation_between_users() {
        let cart_service = Data::new(CartService::new());
        let product_service = Data::new(ProductService::new());

        // Create products
        let product1 = product_service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let product2 = product_service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test".to_string(),
            price: dec!(20.00),
            inventory_count: 10,
        });

        let app = test::init_service(
            App::new()
                .app_data(cart_service.clone())
                .app_data(product_service.clone())
                .service(web::scope("/api/cart").configure(configure_cart_routes)),
        )
        .await;

        // User 1 adds product 1
        let add_req = AddItemRequest {
            product_id: product1.id,
            quantity: 1,
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", "Bearer user_1"))
            .set_json(&add_req)
            .to_request();
        let _ = test::call_service(&app, req).await;

        // User 2 adds product 2
        let add_req = AddItemRequest {
            product_id: product2.id,
            quantity: 1,
        };

        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .insert_header(("Authorization", "Bearer user_2"))
            .set_json(&add_req)
            .to_request();
        let _ = test::call_service(&app, req).await;

        // Verify User 1's cart only has product 1
        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", "Bearer user_1"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // Verify User 2's cart only has product 2
        let req = test::TestRequest::get()
            .uri("/api/cart")
            .insert_header(("Authorization", "Bearer user_2"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
