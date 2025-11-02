use actix_web::{test, App};

/// Test the health check endpoint returns 200 OK with correct JSON structure
#[actix_web::test]
async fn test_health_check() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json.get("version").is_some());
}

/// Test that placeholder endpoints return 501 Not Implemented
#[actix_web::test]
async fn test_not_implemented_endpoints() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    // Test product listing endpoint
    let req = test::TestRequest::get().uri("/api/products").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501); // Not Implemented

    // Test auth register endpoint
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test cart endpoint
    let req = test::TestRequest::get().uri("/api/cart").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}

/// Test that non-existent routes return 404 Not Found
#[actix_web::test]
async fn test_404_handling() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    let req = test::TestRequest::get()
        .uri("/api/nonexistent")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test all authentication routes are configured
#[actix_web::test]
async fn test_auth_routes_configured() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    // Test register endpoint
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test login endpoint
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}

/// Test all product routes are configured
#[actix_web::test]
async fn test_product_routes_configured() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    // Test list products endpoint
    let req = test::TestRequest::get().uri("/api/products").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test get product by ID endpoint
    let req = test::TestRequest::get().uri("/api/products/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}

/// Test all cart routes are configured
#[actix_web::test]
async fn test_cart_routes_configured() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    // Test get cart endpoint
    let req = test::TestRequest::get().uri("/api/cart").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test add to cart endpoint
    let req = test::TestRequest::post().uri("/api/cart/add").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test remove from cart endpoint
    let req = test::TestRequest::delete()
        .uri("/api/cart/remove/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);

    // Test clear cart endpoint
    let req = test::TestRequest::post()
        .uri("/api/cart/clear")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}

/// Test user routes are configured
#[actix_web::test]
async fn test_user_routes_configured() {
    let app =
        test::init_service(App::new().configure(ecommerce_catalog::api::configure_routes)).await;

    // Test get user endpoint
    let req = test::TestRequest::get().uri("/api/users").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501);
}
