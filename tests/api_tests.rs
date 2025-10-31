use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::catalog::{Product, ProductService};
use rust_decimal::Decimal;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    // Initialize test app with routes
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // GET /api/health
    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;

    // Verify 200 OK response
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Verify JSON body {"status":"ok"}
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "ok");
}

#[actix_web::test]
async fn test_product_routes_get_all() {
    // Create ProductService with test products
    let product_service = web::Data::new(ProductService::new());

    // Add test products
    let _ = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Test Product 1".to_string(),
        description: "Description 1".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    let _ = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Test Product 2".to_string(),
        description: "Description 2".to_string(),
        price: Decimal::new(2999, 2), // $29.99
        inventory_count: 5,
    });

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test GET /api/products returns all products
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let products: Vec<Product> = test::read_body_json(resp).await;
    assert_eq!(products.len(), 2);
    assert_eq!(products[0].name, "Test Product 1");
    assert_eq!(products[1].name, "Test Product 2");
}

#[actix_web::test]
async fn test_product_routes_get_by_id() {
    // Create ProductService with test product
    let product_service = web::Data::new(ProductService::new());

    let created_product = product_service.create(cto_parallel_test::catalog::models::NewProduct {
        name: "Specific Product".to_string(),
        description: "A specific product".to_string(),
        price: Decimal::new(4999, 2), // $49.99
        inventory_count: 3,
    });

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test GET /api/products/:id returns specific product
    let req = test::TestRequest::get()
        .uri(&format!("/api/products/{}", created_product.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let product: Product = test::read_body_json(resp).await;
    assert_eq!(product.id, created_product.id);
    assert_eq!(product.name, "Specific Product");
    assert_eq!(product.price, Decimal::new(4999, 2));
}

#[actix_web::test]
async fn test_product_not_found() {
    // Create empty ProductService
    let product_service = web::Data::new(ProductService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test GET /api/products/:id with non-existent ID
    let req = test::TestRequest::get()
        .uri("/api/products/9999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].as_str().is_some());
}

#[actix_web::test]
async fn test_products_empty_list() {
    // Create empty ProductService
    let product_service = web::Data::new(ProductService::new());

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test GET /api/products returns empty array
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let products: Vec<Product> = test::read_body_json(resp).await;
    assert_eq!(products.len(), 0);
}
