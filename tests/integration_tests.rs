use actix_web::{test, web, App};
use cto_parallel_test::{
    api::routes::configure_routes,
    auth::jwt,
    cart::CartService,
    catalog::{models::NewProduct, ProductService},
};
use rust_decimal::Decimal;

#[actix_web::test]
async fn test_full_user_flow() {
    // Setup services
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Create a test product
    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A test product".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });

    // Create JWT token for test user (ID: "1")
    let _token = jwt::create_token("1").unwrap();

    // Add product to cart using CartService directly
    let _ = cart_service.add_item("1", test_product.id, 2);

    // Verify cart contains correct item
    let cart = cart_service.get_cart("1").unwrap();

    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].product_id, test_product.id);
    assert_eq!(cart.items[0].quantity, 2);
    assert_eq!(cart.user_id, "1");
}

#[actix_web::test]
async fn test_multiple_products_in_cart() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    // Create multiple test products
    let product1 = product_service.create(NewProduct {
        name: "Product 1".to_string(),
        description: "First product".to_string(),
        price: Decimal::new(999, 2),
        inventory_count: 5,
    });

    let product2 = product_service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Second product".to_string(),
        price: Decimal::new(1499, 2),
        inventory_count: 3,
    });

    // Add both products to cart
    let _ = cart_service.add_item("1", product1.id, 1);
    let _ = cart_service.add_item("1", product2.id, 2);

    // Verify cart contains both items
    let cart = cart_service.get_cart("1").unwrap();

    assert_eq!(cart.items.len(), 2);
    assert!(cart
        .items
        .iter()
        .any(|i| i.product_id == product1.id && i.quantity == 1));
    assert!(cart
        .items
        .iter()
        .any(|i| i.product_id == product2.id && i.quantity == 2));
}

#[actix_web::test]
async fn test_cart_quantity_update() {
    let product_service = web::Data::new(ProductService::new());
    let cart_service = web::Data::new(CartService::new());

    let product = product_service.create(NewProduct {
        name: "Product".to_string(),
        description: "Test product".to_string(),
        price: Decimal::new(999, 2),
        inventory_count: 10,
    });

    // Add product to cart
    let _ = cart_service.add_item("1", product.id, 2);

    // Add same product again (should update quantity)
    let _ = cart_service.add_item("1", product.id, 3);

    let cart = cart_service.get_cart("1").unwrap();

    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].quantity, 5); // 2 + 3 = 5
}

#[actix_web::test]
async fn test_authentication_token_validation() {
    // Test JWT token creation and validation
    let user_id = "test_user_123";
    let token = jwt::create_token(user_id).unwrap();

    // Validate token
    let claims = jwt::validate_token(&token).unwrap();

    assert_eq!(claims.sub, user_id);
    assert!(claims.exp > claims.iat);
}

#[actix_web::test]
async fn test_health_check_integration() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}
