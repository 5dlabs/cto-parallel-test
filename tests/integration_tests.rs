use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::auth::create_token;
use cto_parallel_test::cart::{Cart, CartService};
use cto_parallel_test::catalog::{models::NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::{json, Value};

#[actix_web::test]
async fn test_full_user_flow() {
    // Step 1: Create ProductService and CartService
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Step 2: Initialize test app with both services
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Step 3: Create test product in ProductService
    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A test product for integration testing".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    // Step 4: Create JWT token for test user (ID: "1")
    let token = create_token("1").expect("Failed to create token");

    // Step 5: POST /api/cart/add with token and product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": test_product.id,
            "quantity": 2
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Step 6: Verify 200 OK response
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["message"], "Item added to cart");

    // Step 7: GET /api/cart with token
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Step 8: Verify cart contains correct item with quantity
    let cart: Cart = test::read_body_json(resp).await;

    assert_eq!(cart.user_id, "1");
    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].product_id, test_product.id);
    assert_eq!(cart.items[0].quantity, 2);
}

#[actix_web::test]
async fn test_cart_without_authentication() {
    // Create services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Try to add to cart without authentication
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .set_json(json!({
            "product_id": 1,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 401 Unauthorized
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].as_str().is_some());
}

#[actix_web::test]
async fn test_cart_with_invalid_token() {
    // Create services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Try to add to cart with invalid token
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", "Bearer invalid.token.here"))
        .set_json(json!({
            "product_id": 1,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 401 Unauthorized
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_multiple_products_in_cart() {
    // Create services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Create multiple products
    let product1 = product_service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "First product".to_string(),
        price: Decimal::new(1000, 2),
        inventory_count: 10,
    });

    let product2 = product_service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Second product".to_string(),
        price: Decimal::new(2000, 2),
        inventory_count: 5,
    });

    // Create token
    let token = create_token("2").expect("Failed to create token");

    // Add first product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product1.id,
            "quantity": 3
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Add second product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product2.id,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Get cart
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let cart: Cart = test::read_body_json(resp).await;

    // Verify both products in cart
    assert_eq!(cart.items.len(), 2);
    assert_eq!(cart.items[0].product_id, product1.id);
    assert_eq!(cart.items[0].quantity, 3);
    assert_eq!(cart.items[1].product_id, product2.id);
    assert_eq!(cart.items[1].quantity, 1);
}

#[actix_web::test]
async fn test_separate_user_carts() {
    // Create services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Create product
    let product = product_service.create(NewProduct {
        name: "Shared Product".to_string(),
        description: "Product for multiple users".to_string(),
        price: Decimal::new(1500, 2),
        inventory_count: 20,
    });

    // Create tokens for two different users
    let token_user1 = create_token("user1").expect("Failed to create token for user1");
    let token_user2 = create_token("user2").expect("Failed to create token for user2");

    // User 1 adds product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token_user1}")))
        .set_json(json!({
            "product_id": product.id,
            "quantity": 2
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // User 2 adds same product with different quantity
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token_user2}")))
        .set_json(json!({
            "product_id": product.id,
            "quantity": 5
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Get User 1 cart
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token_user1}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let cart1: Cart = test::read_body_json(resp).await;

    // Get User 2 cart
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token_user2}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let cart2: Cart = test::read_body_json(resp).await;

    // Verify separate carts
    assert_eq!(cart1.user_id, "user1");
    assert_eq!(cart1.items[0].quantity, 2);

    assert_eq!(cart2.user_id, "user2");
    assert_eq!(cart2.items[0].quantity, 5);
}
