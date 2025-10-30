use actix_web::{test, web, App};
use cto_parallel_test::api::routes::configure_routes;
use cto_parallel_test::catalog::{models::NewProduct, ProductService};
use rust_decimal::Decimal;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[actix_web::test]
async fn test_product_routes_placeholder() {
    // Create ProductService with test products
    let product_service = web::Data::new(ProductService::new());

    // Add test products
    let _ = product_service.create(NewProduct {
        name: "Test Product 1".to_string(),
        description: "First test product".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    let _ = product_service.create(NewProduct {
        name: "Test Product 2".to_string(),
        description: "Second test product".to_string(),
        price: Decimal::new(2999, 2), // $29.99
        inventory_count: 5,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test that the route exists (even if it returns NotImplemented)
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;
    // The route exists but returns NotImplemented (501)
    assert!(
        resp.status() == actix_web::http::StatusCode::NOT_IMPLEMENTED
            || resp.status() == actix_web::http::StatusCode::OK
    );
}
