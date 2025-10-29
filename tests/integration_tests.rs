use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::auth::create_token;
use cto_parallel_test::cart::{Cart, CartService};
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::json;

/// Test the complete user flow: create product → add to cart → get cart
/// This integration test validates that all modules work together correctly
#[actix_web::test]
async fn test_full_user_flow() {
    // Setup services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Initialize test app with both services
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // 1. Create a test product
    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A test product for integration testing".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    // 2. Create a JWT token for test user (ID: "1")
    let token = create_token("1").expect("Failed to create token");

    // 3. Add product to cart with authentication
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": test_product.id,
            "quantity": 2
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "Add to cart should return 200 OK"
    );

    // 4. Get cart and verify product was added
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Get cart should return 200 OK");

    let body = test::read_body(resp).await;
    let cart: Cart = serde_json::from_slice(&body).expect("Failed to parse cart response");

    // Verify cart contains the correct item with correct quantity
    assert_eq!(cart.items.len(), 1, "Cart should contain 1 item");
    assert_eq!(
        cart.items[0].product_id, test_product.id,
        "Cart item should have correct product_id"
    );
    assert_eq!(
        cart.items[0].quantity, 2,
        "Cart item should have correct quantity"
    );
}

/// Test that cart operations require authentication
#[actix_web::test]
async fn test_cart_requires_authentication() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Try to get cart without authentication
    let req = test::TestRequest::get().uri("/api/cart").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401, "Request without auth should return 401");
}

/// Test that cart operations reject invalid tokens
#[actix_web::test]
async fn test_cart_rejects_invalid_token() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Try to get cart with invalid token
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", "Bearer invalid.token.here"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        401,
        "Request with invalid token should return 401"
    );
}

/// Test adding multiple items to cart
#[actix_web::test]
async fn test_add_multiple_items_to_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Create two test products
    let product1 = product_service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "First product".to_string(),
        price: Decimal::new(1999, 2),
        inventory_count: 10,
    });

    let product2 = product_service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Second product".to_string(),
        price: Decimal::new(2999, 2),
        inventory_count: 5,
    });

    let token = create_token("1").expect("Failed to create token");

    // Add first product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product1.id,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Add second product
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product2.id,
            "quantity": 3
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get cart and verify both products are present
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let cart: Cart = serde_json::from_slice(&body).expect("Failed to parse cart");

    assert_eq!(cart.items.len(), 2, "Cart should contain 2 items");
}

/// Test that adding same product twice increases quantity
#[actix_web::test]
async fn test_adding_same_product_increases_quantity() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    let product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "Product for quantity test".to_string(),
        price: Decimal::new(1999, 2),
        inventory_count: 20,
    });

    let token = create_token("1").expect("Failed to create token");

    // Add product first time with quantity 2
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product.id,
            "quantity": 2
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Add same product again with quantity 3
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product.id,
            "quantity": 3
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get cart and verify quantity is 5 (2 + 3)
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let cart: Cart = serde_json::from_slice(&body).expect("Failed to parse cart");

    assert_eq!(cart.items.len(), 1, "Cart should contain 1 unique item");
    assert_eq!(
        cart.items[0].quantity, 5,
        "Quantity should be sum of both additions"
    );
}
