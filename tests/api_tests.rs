use actix_web::{test, web, App};
use cto_parallel_test::api::routes::configure_routes;
use cto_parallel_test::{CartService, NewProduct, Product, ProductService};
use rust_decimal::Decimal;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[actix_web::test]
async fn test_product_routes() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Add test products
    let _ = product_service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "First product".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    let _ = product_service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Second product".to_string(),
        price: Decimal::new(2999, 2), // $29.99
        inventory_count: 5,
    });

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    // Test get all products
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let products: Vec<Product> = serde_json::from_slice(&body).unwrap();
    assert_eq!(products.len(), 2);

    // Test get specific product
    let req = test::TestRequest::get().uri("/api/products/1").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let product: Product = serde_json::from_slice(&body).unwrap();
    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Product 1");
}

#[actix_web::test]
async fn test_product_not_found() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let app = test::init_service(
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products/9999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
