use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    // Initialize test app with routes
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Make request to health check endpoint
    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;

    // Verify 200 OK response
    assert!(
        resp.status().is_success(),
        "Health check should return 200 OK"
    );

    // Parse response body
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response structure
    assert_eq!(
        json["status"], "ok",
        "Health check should return status: ok"
    );
}

#[actix_web::test]
async fn test_get_all_products() {
    // Create product service with test data
    let product_service = web::Data::new(ProductService::new());

    // Add test products
    let _ = product_service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "Description 1".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    let _ = product_service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Description 2".to_string(),
        price: Decimal::new(2999, 2), // $29.99
        inventory_count: 5,
    });

    // Initialize test app with product service
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Make request to get all products
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;

    // Verify response
    assert!(
        resp.status().is_success(),
        "Get products should return 200 OK"
    );

    let body = test::read_body(resp).await;
    let products: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify we got an array of products
    assert!(products.is_array(), "Response should be an array");
    let products_array = products.as_array().unwrap();
    assert_eq!(products_array.len(), 2, "Should return 2 products");
}

#[actix_web::test]
async fn test_get_product_by_id() {
    // Create product service with test data
    let product_service = web::Data::new(ProductService::new());

    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "Test Description".to_string(),
        price: Decimal::new(4999, 2), // $49.99
        inventory_count: 15,
    });

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Make request to get specific product
    let req = test::TestRequest::get()
        .uri(&format!("/api/products/{}", test_product.id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Verify response
    assert!(
        resp.status().is_success(),
        "Get product by ID should return 200 OK"
    );

    let body = test::read_body(resp).await;
    let product: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify product details
    assert_eq!(product["id"], test_product.id, "Product ID should match");
    assert_eq!(product["name"], "Test Product", "Product name should match");
}

#[actix_web::test]
async fn test_get_nonexistent_product_returns_404() {
    let product_service = web::Data::new(ProductService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    // Request a product that doesn't exist
    let req = test::TestRequest::get()
        .uri("/api/products/9999")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 404
    assert_eq!(
        resp.status().as_u16(),
        404,
        "Nonexistent product should return 404"
    );
}

#[actix_web::test]
async fn test_product_routes_with_empty_catalog() {
    let product_service = web::Data::new(ProductService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    // Get all products when catalog is empty
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "Empty catalog should still return 200 OK"
    );

    let body = test::read_body(resp).await;
    let products: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(products.is_array(), "Response should be an array");
    assert_eq!(
        products.as_array().unwrap().len(),
        0,
        "Empty catalog should return empty array"
    );
}
