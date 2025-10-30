use actix_web::{test, web, App};
use cto_parallel_test::api::routes::configure_routes;
use cto_parallel_test::{create_token, Cart, CartService, NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::json;

#[actix_web::test]
async fn test_full_user_flow() {
    // Setup services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // 1. Create a test product
    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A test product".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // 2. Create a test user token
    let token = create_token("1").unwrap(); // User ID 1

    // 3. Add product to cart
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(json!({
            "product_id": test_product.id,
            "quantity": 2
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 4. Get cart and verify product was added
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let cart: Cart = serde_json::from_slice(&body).unwrap();

    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].product_id, test_product.id);
    assert_eq!(cart.items[0].quantity, 2);
}

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

    // Try to access cart without authentication
    let req = test::TestRequest::get().uri("/api/cart").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

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

    // Try to access cart with invalid token
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", "Bearer invalid.token.here"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_multiple_users_have_separate_carts() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Create test products
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

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Create tokens for two users
    let token1 = create_token("user1").unwrap();
    let token2 = create_token("user2").unwrap();

    // User 1 adds product 1
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token1}")))
        .set_json(json!({
            "product_id": product1.id,
            "quantity": 1
        }))
        .to_request();
    test::call_service(&app, req).await;

    // User 2 adds product 2
    let req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token2}")))
        .set_json(json!({
            "product_id": product2.id,
            "quantity": 2
        }))
        .to_request();
    test::call_service(&app, req).await;

    // Verify user 1's cart
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token1}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let cart1: Cart = serde_json::from_slice(&body).unwrap();
    assert_eq!(cart1.items.len(), 1);
    assert_eq!(cart1.items[0].product_id, product1.id);

    // Verify user 2's cart
    let req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token2}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let cart2: Cart = serde_json::from_slice(&body).unwrap();
    assert_eq!(cart2.items.len(), 1);
    assert_eq!(cart2.items[0].product_id, product2.id);
}
