//! Integration tests for API routes
//!
//! Tests health check endpoint and placeholder routes

use actix_web::{test, App};
use cto_parallel_test::api::configure_routes;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    // Initialize test service
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Create request
    let req = test::TestRequest::get().uri("/api/health").to_request();

    // Call service
    let resp = test::call_service(&app, req).await;

    // Assert success status
    assert!(resp.status().is_success());
    assert_eq!(resp.status().as_u16(), 200);

    // Parse and verify response body
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON response");

    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[actix_web::test]
async fn test_not_implemented_products() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 501 Not Implemented
    assert_eq!(resp.status().as_u16(), 501);

    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON response");

    assert_eq!(json["error"], "not_implemented");
    assert_eq!(
        json["message"],
        "This endpoint will be implemented in a later task"
    );
}

#[actix_web::test]
async fn test_not_implemented_auth_register() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 501);

    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON response");

    assert_eq!(json["error"], "not_implemented");
}

#[actix_web::test]
async fn test_not_implemented_cart() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/cart").to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 501);
}

#[actix_web::test]
async fn test_not_found_invalid_route() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri("/api/invalid-route")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 404 Not Found
    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
async fn test_cart_routes_structure() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test /api/cart/add
    let req = test::TestRequest::post().uri("/api/cart/add").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);

    // Test /api/cart/remove/{product_id}
    let req = test::TestRequest::delete()
        .uri("/api/cart/remove/123")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);

    // Test /api/cart/clear
    let req = test::TestRequest::post()
        .uri("/api/cart/clear")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);
}

#[actix_web::test]
async fn test_product_routes_structure() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test /api/products (list)
    let req = test::TestRequest::get().uri("/api/products").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);

    // Test /api/products/{id}
    let req = test::TestRequest::get()
        .uri("/api/products/456")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);
}

#[actix_web::test]
async fn test_auth_routes_structure() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test /api/auth/register
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);

    // Test /api/auth/login
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 501);
}
