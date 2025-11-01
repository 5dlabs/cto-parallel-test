//! API Routes Integration Tests
//!
//! This module tests the HTTP API endpoints to ensure they respond correctly.
//! Tests include:
//! - Health check endpoint
//! - Placeholder endpoints (not yet implemented)
//! - 404 handling for non-existent routes

use actix_web::{test, App};

#[actix_web::test]
async fn test_health_check() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // When: We request the health check endpoint
    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;

    // Then: We should get a 200 OK response
    assert!(resp.status().is_success());

    // And: The response body should contain the expected JSON
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[actix_web::test]
async fn test_not_implemented_product_list() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // When: We request an unimplemented endpoint
    let req = test::TestRequest::get().uri("/api/products").to_request();

    let resp = test::call_service(&app, req).await;

    // Then: We should get a 501 Not Implemented response
    assert_eq!(resp.status(), 501);

    // And: The response should contain an error message
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "not_implemented");
    assert!(json["message"].is_string());
}

#[actix_web::test]
async fn test_not_implemented_auth_register() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // When: We request the auth register endpoint
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: We should get a 501 Not Implemented response
    assert_eq!(resp.status(), 501);
}

#[actix_web::test]
async fn test_not_implemented_auth_login() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // When: We request the auth login endpoint
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: We should get a 501 Not Implemented response
    assert_eq!(resp.status(), 501);
}

#[actix_web::test]
async fn test_not_implemented_cart_endpoints() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // Test GET /api/cart
    let req = test::TestRequest::get().uri("/api/cart").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test POST /api/cart/add
    let req = test::TestRequest::post().uri("/api/cart/add").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test DELETE /api/cart/remove/1
    let req = test::TestRequest::delete()
        .uri("/api/cart/remove/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test POST /api/cart/clear
    let req = test::TestRequest::post()
        .uri("/api/cart/clear")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}

#[actix_web::test]
async fn test_not_found_route() {
    // Given: A test app with configured routes
    let app =
        test::init_service(App::new().configure(cto_parallel_test::api::configure_routes)).await;

    // When: We request a non-existent route
    let req = test::TestRequest::get()
        .uri("/api/nonexistent")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: We should get a 404 Not Found response
    assert_eq!(resp.status(), 404);
}
