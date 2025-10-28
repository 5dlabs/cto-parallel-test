# Task 7: Integration Tests

## Overview
Create comprehensive integration tests for the application covering the full user flow, API endpoints, authentication, and system integration. This is a Level 2 task (final integration) that depends on Tasks 2 (API Endpoints), 5 (Shopping Cart API), and 6 (Frontend Components), demonstrating the complete dependency chain in the parallel task execution system.

## Context
This task is the final integration step in the parallel task execution test project. It validates that all components work together correctly by testing the complete system end-to-end. The implementation uses actix-web test utilities for API testing and verifies the integration of authentication, product catalog, and shopping cart functionality.

## Objectives
1. Create `tests/integration_tests.rs` for general integration tests including health check and full user flow
2. Create `tests/api_tests.rs` for API-specific endpoint testing
3. Create `tests/auth_tests.rs` for authentication and JWT testing
4. Update `src/main.rs` to be testable with proper service initialization
5. Verify all major system components integrate correctly

## Dependencies

**Depends On:**
- **Task 2 (API Endpoints)** - Level 1 - Provides API route configuration and health check
- **Task 5 (Shopping Cart API)** - Level 1 - Provides cart functionality to test complete user flow
- **Task 6 (Frontend Components)** - Level 0 - Frontend structure (verified present but not directly tested in Rust tests)

**Depended Upon By:**
- None - This is the final task

## Files to Create/Modify

### 1. `tests/integration_tests.rs`
General integration tests for system-wide functionality:

```rust
#[cfg(test)]
mod integration_tests {
    use actix_web::{test, web, App};
    use actix_web::http::StatusCode;
    use serde_json::json;

    use crate::api::routes::configure_routes;
    use crate::auth::jwt::create_token;
    use crate::catalog::ProductService;
    use crate::cart::CartService;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .configure(configure_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/health")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
    }

    #[actix_web::test]
    async fn test_full_user_flow() {
        // Setup services
        let product_service = web::Data::new(ProductService::new());
        let cart_service = web::Data::new(CartService::new());

        let app = test::init_service(
            App::new()
                .app_data(product_service.clone())
                .app_data(cart_service.clone())
                .configure(configure_routes)
        ).await;

        // 1. Create a test product
        let test_product = product_service.create(crate::catalog::models::NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: rust_decimal::Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        // 2. Create a test user token
        let token = create_token("1").unwrap(); // User ID 1

        // 3. Add product to cart
        let req = test::TestRequest::post()
            .uri("/api/cart/add")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "product_id": test_product.id,
                "quantity": 2
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // 4. Get cart and verify product was added
        let req = test::TestRequest::get()
            .uri("/api/cart")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let cart: crate::cart::service::Cart = serde_json::from_slice(&body).unwrap();

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, test_product.id);
        assert_eq!(cart.items[0].quantity, 2);
    }
}
```

**Test Coverage:**
- Health check endpoint responds correctly
- Complete user flow: create product → generate JWT → add to cart → verify cart

### 2. `tests/api_tests.rs`
API endpoint-specific tests:

```rust
#[cfg(test)]
mod api_tests {
    use actix_web::{test, web, App};
    use actix_web::http::StatusCode;

    use crate::api::routes::configure_routes;
    use crate::catalog::ProductService;

    #[actix_web::test]
    async fn test_product_routes() {
        // Setup product service with test data
        let product_service = web::Data::new(ProductService::new());

        // Add test products
        product_service.create(crate::catalog::models::NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: rust_decimal::Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        product_service.create(crate::catalog::models::NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: rust_decimal::Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        let app = test::init_service(
            App::new()
                .app_data(product_service.clone())
                .configure(configure_routes)
        ).await;

        // Test get all products
        let req = test::TestRequest::get()
            .uri("/api/products")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let products: Vec<crate::catalog::models::Product> = serde_json::from_slice(&body).unwrap();

        assert_eq!(products.len(), 2);

        // Test get product by ID
        let req = test::TestRequest::get()
            .uri("/api/products/1")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let product: crate::catalog::models::Product = serde_json::from_slice(&body).unwrap();

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Product 1");
    }
}
```

**Test Coverage:**
- Product API endpoints (GET all, GET by ID)
- Response status codes and data structure
- Product data correctness

### 3. `tests/auth_tests.rs`
Authentication and JWT tests:

```rust
#[cfg(test)]
mod auth_tests {
    use crate::auth::jwt::{create_token, validate_token};
    use crate::auth::models::User;

    #[test]
    fn test_jwt_creation_and_validation() {
        // Create a token
        let user_id = "123";
        let token = create_token(user_id).unwrap();

        // Validate the token
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_password_hashing_and_verification() {
        // Create a test user with hashed password
        let password = "secure_password";
        let hashed = User::hash_password(password);

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hashed,
        };

        // Verify password
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));
    }
}
```

**Test Coverage:**
- JWT token creation and validation
- Password hashing and verification
- User authentication mechanisms

### 4. `src/main.rs`
Update main.rs to be testable:

```rust
use actix_web::{App, HttpServer, web};
mod api;
mod schema;
mod auth;
mod catalog;
mod cart;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API server");

    // Initialize services
    let product_service = web::Data::new(catalog::ProductService::new());
    let cart_service = web::Data::new(cart::CartService::new());

    HttpServer::new(move || {
        App::new()
            .app_data(product_service.clone())
            .app_data(cart_service.clone())
            .configure(api::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Changes:**
- Properly initialize services (ProductService, CartService)
- Share services across handlers using web::Data
- Configure routes with services available

## Implementation Steps

1. **Create Tests Directory Structure**
   - Create `tests/` directory in project root (sibling to src/)
   - This is Cargo's standard location for integration tests

2. **Implement Integration Tests**
   - Create `tests/integration_tests.rs`
   - Import necessary modules from crate
   - Implement health check test
   - Implement full user flow test covering:
     - Product creation
     - JWT token generation
     - Cart operations
     - Response validation

3. **Implement API Tests**
   - Create `tests/api_tests.rs`
   - Set up test product data
   - Test product list endpoint
   - Test product detail endpoint
   - Verify status codes and response structure

4. **Implement Authentication Tests**
   - Create `tests/auth_tests.rs`
   - Test JWT token lifecycle
   - Test password hashing
   - Test password verification
   - Unit tests for auth module functions

5. **Update Main Application**
   - Modify `src/main.rs` to initialize services properly
   - Ensure services are shared as web::Data
   - Configure routes with service dependencies
   - Make application testable by integration tests

6. **Validation**
   - Run `cargo test` to execute all tests
   - Verify all tests pass
   - Check test output for coverage
   - Ensure no compilation errors

## Technical Considerations

### Testing Framework
- **actix-web::test**: Provides test utilities for actix-web apps
- **#[actix_web::test]**: Async test macro for actix handlers
- **#[test]**: Standard Rust unit test macro
- **test::init_service**: Creates test instance of application
- **test::TestRequest**: Builder for test HTTP requests
- **test::call_service**: Executes test request
- **test::read_body**: Reads response body

### Test Organization
- Integration tests in `tests/` directory
- Unit tests would be in module files with `#[cfg(test)]`
- Separate files by concern: general, API, auth
- Each test file has its own module

### Service Initialization
- Tests need access to services (ProductService, CartService)
- Use web::Data::new() to create shared state
- Clone Data for each test app instance
- Services must implement Clone or use Arc

### Test Data Management
- Create test data at beginning of each test
- Use descriptive test data for clarity
- Clean slate for each test (no shared state)
- Hardcode test values for reproducibility

### Async Testing
- Use #[actix_web::test] for async tests
- await on async operations
- actix runtime handles test execution

## Integration Points

### With Task 2 (API Endpoints)
- Tests use configure_routes from api module
- Health check endpoint must exist
- Route configuration must work in test environment

### With Task 3 (User Authentication)
- Tests use create_token from auth::jwt
- Tests validate JWT functionality
- Password hashing tests verify auth module

### With Task 4 (Product Catalog)
- Tests create and retrieve products
- ProductService must be testable
- Product models must serialize/deserialize

### With Task 5 (Shopping Cart API)
- Full user flow tests cart operations
- Cart endpoints must accept JWT tokens
- CartService must integrate with ProductService

### With Task 6 (Frontend Components)
- Verify frontend structure exists (file check)
- Not directly tested in Rust tests
- Frontend would have separate Jest/React tests

## Risks and Mitigation

**Risk**: Tests depend on all previous tasks being complete
- **Mitigation**: This is a Level 2 task. Platform ensures dependencies complete first.

**Risk**: Service initialization might differ between main and tests
- **Mitigation**: Use consistent pattern for service creation in both.

**Risk**: Tests might be flaky due to timing or state
- **Mitigation**: Each test creates its own services. No shared state.

**Risk**: Import paths might not work in test context
- **Mitigation**: Use `crate::` prefix for imports in test files.

## Success Criteria

1. ✅ `tests/integration_tests.rs` exists with health check and full user flow tests
2. ✅ `tests/api_tests.rs` exists with product API tests
3. ✅ `tests/auth_tests.rs` exists with JWT and password tests
4. ✅ All test files use correct actix-web test utilities
5. ✅ Tests import modules correctly with `crate::` prefix
6. ✅ `src/main.rs` updated with proper service initialization
7. ✅ Services (ProductService, CartService) properly shared
8. ✅ All tests compile without errors
9. ✅ `cargo test` runs successfully
10. ✅ All tests pass (health check, user flow, products, auth)
11. ✅ Test coverage includes major integration points
12. ✅ No test flakiness or timing issues

## Estimated Effort
**60 minutes** - Integration test implementation, API testing, authentication testing, and main.rs updates to support testability
