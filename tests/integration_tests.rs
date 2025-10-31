use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::auth::create_token;
use cto_parallel_test::cart::CartService;
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;

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

    // Initialize the app with both services
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // 2. Create a test user token
    let token = create_token("1").expect("Token creation failed"); // User ID 1

    // 3. Add product to cart
    let add_req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "product_id": test_product.id,
            "quantity": 2
        }))
        .to_request();

    let add_resp = test::call_service(&app, add_req).await;
    assert!(
        add_resp.status().is_success(),
        "Adding to cart failed with status: {}",
        add_resp.status()
    );

    // 4. Get cart and verify product was added
    let get_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    assert!(
        get_resp.status().is_success(),
        "Getting cart failed with status: {}",
        get_resp.status()
    );

    let body = test::read_body(get_resp).await;
    let cart: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify cart contains correct item and quantity
    assert_eq!(
        cart["items"]
            .as_array()
            .expect("items is not an array")
            .len(),
        1
    );
    assert_eq!(cart["items"][0]["product_id"], test_product.id);
    assert_eq!(cart["items"][0]["quantity"], 2);
}

#[actix_web::test]
async fn test_cart_requires_authentication() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    // Try to get cart without authentication
    let req = test::TestRequest::get().uri("/api/cart").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401, "Should require authentication");
}

#[actix_web::test]
async fn test_add_multiple_items_to_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Create multiple test products
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
            .app_data(product_service)
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    let token = create_token("1").expect("Token creation failed");

    // Add first product
    let req1 = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "product_id": product1.id,
            "quantity": 1
        }))
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());

    // Add second product
    let req2 = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "product_id": product2.id,
            "quantity": 3
        }))
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success());

    // Verify both items in cart
    let get_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    let body = test::read_body(get_resp).await;
    let cart: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert_eq!(
        cart["items"]
            .as_array()
            .expect("items is not an array")
            .len(),
        2
    );
}

#[actix_web::test]
async fn test_remove_item_from_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let product = product_service.create(NewProduct {
        name: "Product".to_string(),
        description: "Description".to_string(),
        price: Decimal::new(1000, 2),
        inventory_count: 10,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    let token = create_token("1").expect("Token creation failed");

    // Add product to cart
    let add_req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "product_id": product.id,
            "quantity": 2
        }))
        .to_request();

    test::call_service(&app, add_req).await;

    // Remove product from cart
    let remove_req = test::TestRequest::delete()
        .uri(&format!("/api/cart/remove/{}", product.id))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let remove_resp = test::call_service(&app, remove_req).await;
    assert!(remove_resp.status().is_success());

    // Verify cart is empty
    let get_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    let body = test::read_body(get_resp).await;
    let cart: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert_eq!(
        cart["items"]
            .as_array()
            .expect("items is not an array")
            .len(),
        0
    );
}

#[actix_web::test]
async fn test_clear_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let product = product_service.create(NewProduct {
        name: "Product".to_string(),
        description: "Description".to_string(),
        price: Decimal::new(1000, 2),
        inventory_count: 10,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .app_data(cart_service)
            .configure(configure_routes),
    )
    .await;

    let token = create_token("1").expect("Token creation failed");

    // Add product to cart
    let add_req = test::TestRequest::post()
        .uri("/api/cart/add")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "product_id": product.id,
            "quantity": 2
        }))
        .to_request();

    test::call_service(&app, add_req).await;

    // Clear the cart
    let clear_req = test::TestRequest::post()
        .uri("/api/cart/clear")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let clear_resp = test::call_service(&app, clear_req).await;
    assert!(clear_resp.status().is_success());

    // Verify cart is empty
    let get_req = test::TestRequest::get()
        .uri("/api/cart")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    let body = test::read_body(get_resp).await;
    let cart: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert_eq!(
        cart["items"]
            .as_array()
            .expect("items is not an array")
            .len(),
        0
    );
}
