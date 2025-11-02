//! Integration tests for API routes
//!
//! These tests verify that all API endpoints are properly configured
//! and return expected responses.

use actix_web::{test, App};
use cto_parallel_test::configure_routes;

/// Test health check endpoint returns 200 OK with correct JSON structure
#[actix_web::test]
async fn test_health_check_returns_ok() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "Health check should return 200 OK"
    );

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok", "Status should be 'ok'");
    assert!(
        json["version"].is_string(),
        "Version should be present and be a string"
    );
}

/// Test that all placeholder endpoints return 501 Not Implemented
#[actix_web::test]
async fn test_placeholder_endpoints_return_not_implemented() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test auth endpoints
    let endpoints = vec![
        ("POST", "/api/auth/register"),
        ("POST", "/api/auth/login"),
        ("GET", "/api/users"),
        ("GET", "/api/products"),
        ("GET", "/api/products/1"),
        ("GET", "/api/cart"),
        ("POST", "/api/cart/add"),
        ("DELETE", "/api/cart/remove/1"),
        ("POST", "/api/cart/clear"),
    ];

    for (method, uri) in endpoints {
        let req = match method {
            "POST" => test::TestRequest::post().uri(uri).to_request(),
            "DELETE" => test::TestRequest::delete().uri(uri).to_request(),
            "GET" => test::TestRequest::get().uri(uri).to_request(),
            _ => panic!("Unsupported method: {method}"),
        };

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            501,
            "{method} {uri} should return 501 Not Implemented"
        );

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            json["error"], "not_implemented",
            "{method} {uri} should have 'not_implemented' error type"
        );
        assert!(
            json["message"].is_string(),
            "{method} {uri} should have a message field"
        );
    }
}

/// Test that invalid routes return 404 Not Found
#[actix_web::test]
async fn test_invalid_routes_return_404() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let invalid_routes = vec![
        "/api/nonexistent",
        "/api/invalid/path",
        "/api/products/invalid/action",
    ];

    for uri in invalid_routes {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404, "{uri} should return 404 Not Found");
    }
}

/// Test that product routes with IDs are properly configured
#[actix_web::test]
async fn test_product_routes_with_ids() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test with numeric ID
    let req = test::TestRequest::get()
        .uri("/api/products/123")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501, "Product by ID should return 501");

    // Test cart remove with ID
    let req = test::TestRequest::delete()
        .uri("/api/cart/remove/456")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501, "Cart remove by ID should return 501");
}

/// Test that all route scopes are properly mounted under /api
#[actix_web::test]
async fn test_routes_are_under_api_scope() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Routes without /api prefix should return 404
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        404,
        "Routes without /api prefix should return 404"
    );

    let req = test::TestRequest::get().uri("/products").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        404,
        "Routes without /api prefix should return 404"
    );
}

/// Test error response format for not implemented endpoints
#[actix_web::test]
async fn test_error_response_format() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/products").to_request();
    let resp = test::call_service(&app, req).await;

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify error response structure
    assert!(json.is_object(), "Error response should be a JSON object");
    assert!(
        json.get("error").is_some(),
        "Error response should have 'error' field"
    );
    assert!(
        json.get("message").is_some(),
        "Error response should have 'message' field"
    );
    assert_eq!(json["error"], "not_implemented");
}
