use actix_web::{test, web, App};
use cto_parallel_test::api::configure_routes;
use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");
    assert_eq!(json["status"], "ok");
}

#[actix_web::test]
async fn test_get_all_products() {
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

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let products: Vec<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");
    assert_eq!(products.len(), 2);
}

#[actix_web::test]
async fn test_get_product_by_id() {
    let product_service = web::Data::new(ProductService::new());

    // Create a test product
    let product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "Test Description".to_string(),
        price: Decimal::new(1999, 2),
        inventory_count: 10,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test getting existing product
    let req = test::TestRequest::get()
        .uri(&format!("/api/products/{}", product.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let returned_product: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");
    assert_eq!(returned_product["name"], "Test Product");
    assert_eq!(returned_product["id"], product.id);
}

#[actix_web::test]
async fn test_get_nonexistent_product() {
    let product_service = web::Data::new(ProductService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service)
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products/999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
