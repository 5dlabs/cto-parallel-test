use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::auth::create_token;
use cto_parallel_test::cart::CartService;
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::{json, Value};

#[actix_web::test]
async fn test_full_user_flow() {
    // Setup: Create services
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

    // Step 1: Create a test product
    let test_product = product_service.create(NewProduct {
        name: "Integration Test Product".to_string(),
        description: "A product for integration testing".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });


    // Step 2: Create JWT token for test user (ID: "1")
    let user_id = "1";
    let token = create_token(user_id).expect("Failed to create test token");


    // Step 3: Add product to cart
    let add_to_cart_req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": test_product.id,
            "quantity": 2
        }))
        .to_request();

    let add_resp = test::call_service(&app, add_to_cart_req).await;

    // Verify add to cart succeeded
    assert!(
        add_resp.status().is_success(),
        "Add to cart should return 200 OK, got: {}",
        add_resp.status()
    );

    let add_body = test::read_body(add_resp).await;
    let _cart_after_add: Value =
        serde_json::from_slice(&add_body).expect("Failed to parse cart response");


    // Step 4: Get cart and verify product was added
    let get_cart_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_cart_req).await;

    // Verify get cart succeeded
    assert!(
        get_resp.status().is_success(),
        "Get cart should return 200 OK, got: {}",
        get_resp.status()
    );

    let get_body = test::read_body(get_resp).await;
    let cart: Value = serde_json::from_slice(&get_body).expect("Failed to parse cart");


    // Verify cart contents
    assert_eq!(
        cart["user_id"], user_id,
        "Cart should belong to the correct user"
    );

    let items = cart["items"]
        .as_array()
        .expect("Cart items should be an array");
    assert_eq!(items.len(), 1, "Cart should contain 1 item");

    let item = &items[0];
    assert_eq!(
        item["product_id"], test_product.id,
        "Cart item should have correct product ID"
    );
    assert_eq!(
        item["quantity"], 2,
        "Cart item should have correct quantity"
    );
}

#[actix_web::test]
async fn test_cart_requires_authentication() {
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    let app = test::init_service(
        App::new()
            .app_data(cart_service)
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    // Try to get cart without authentication
    let req = test::TestRequest::get().uri("/api/cart").to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 401 Unauthorized
    assert_eq!(
        resp.status().as_u16(),
        401,
        "Cart access without auth should return 401"
    );
}

#[actix_web::test]
async fn test_add_to_cart_with_invalid_token() {
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    let app = test::init_service(
        App::new()
            .app_data(cart_service)
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    // Try to add to cart with invalid token
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", "Bearer invalid_token"))
        .set_json(json!({
            "product_id": 1,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 401 Unauthorized
    assert_eq!(
        resp.status().as_u16(),
        401,
        "Invalid token should return 401"
    );
}

#[actix_web::test]
async fn test_add_nonexistent_product_to_cart() {
    let cart_service = web::Data::new(CartService::new());
    let product_service = web::Data::new(ProductService::new());

    let token = create_token("1").expect("Failed to create token");

    let app = test::init_service(
        App::new()
            .app_data(cart_service)
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    // Try to add a product that doesn't exist
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": 9999,
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 404 Not Found
    assert_eq!(
        resp.status().as_u16(),
        404,
        "Adding nonexistent product should return 404"
    );
}

#[actix_web::test]
async fn test_multiple_products_in_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

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

    let token = create_token("1").expect("Failed to create token");

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    // Add first product
    let req1 = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product1.id,
            "quantity": 2
        }))
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());

    // Add second product
    let req2 = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product2.id,
            "quantity": 3
        }))
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success());

    // Get cart
    let get_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    assert!(get_resp.status().is_success());

    let body = test::read_body(get_resp).await;
    let cart: Value = serde_json::from_slice(&body).expect("Failed to parse cart");

    let items = cart["items"].as_array().unwrap();
    assert_eq!(items.len(), 2, "Cart should contain 2 different products");
}

#[actix_web::test]
async fn test_system_integration() {
    // This test verifies that all modules work together correctly
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    // 1. Check system health
    let health_req = test::TestRequest::get().uri("/api/health").to_request();
    let health_resp = test::call_service(&app, health_req).await;
    assert!(
        health_resp.status().is_success(),
        "System should be healthy"
    );

    // 2. Browse products
    let products_req = test::TestRequest::get().uri("/api/products").to_request();
    let products_resp = test::call_service(&app, products_req).await;
    assert!(
        products_resp.status().is_success(),
        "Should be able to browse products"
    );

    // 3. Create a product (simulating admin action)
    let product = product_service.create(NewProduct {
        name: "System Test Product".to_string(),
        description: "For system integration test".to_string(),
        price: Decimal::new(5999, 2),
        inventory_count: 100,
    });

    // 4. Authenticate user
    let token = create_token("system_test_user").expect("Failed to create token");

    // 5. Add product to cart
    let add_req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": product.id,
            "quantity": 1
        }))
        .to_request();

    let add_resp = test::call_service(&app, add_req).await;
    assert!(
        add_resp.status().is_success(),
        "Should be able to add to cart"
    );

    // 6. Verify cart
    let cart_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let cart_resp = test::call_service(&app, cart_req).await;
    assert!(
        cart_resp.status().is_success(),
        "Should be able to retrieve cart"
    );

    let body = test::read_body(cart_resp).await;
    let cart: Value = serde_json::from_slice(&body).expect("Failed to parse cart");

    assert_eq!(
        cart["items"].as_array().unwrap().len(),
        1,
        "Cart should contain the added item"
    );
}
