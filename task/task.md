# Task 7: Integration Tests

## Overview
Create comprehensive integration tests for the entire application, validating API endpoints, authentication flow, product catalog, shopping cart, and end-to-end user workflows. This is a Level 2 task that depends on Tasks 2, 5, and 6, serving as the final validation of the complete system.

## Context
This task validates that all previous tasks integrate correctly and the application works as a cohesive system. It tests not just individual components but the interactions between modules, ensuring the parallel development efforts have produced a working application.

## Objectives
1. Test API health check and basic connectivity
2. Test JWT authentication (token creation and validation)
3. Test password hashing and verification
4. Test product catalog operations
5. Test shopping cart with authentication
6. Test full user flow (auth → product → cart)
7. Validate error handling and edge cases

## Dependencies
- **Task 2: API Endpoints** - Provides HTTP server and routing
- **Task 5: Shopping Cart API** - Provides cart functionality to test
- **Task 6: Frontend** - Component structure (for future frontend tests)

Implicitly depends on Tasks 1, 3, and 4 through Task 5.

## Files to Create
- `tests/integration_tests.rs` - General integration tests and full user flow
- `tests/api_tests.rs` - API endpoint tests
- `tests/auth_tests.rs` - Authentication unit and integration tests

## Technical Specifications

### Testing Framework
- **Framework**: Built-in Rust testing with `#[test]` and `#[actix_web::test]`
- **HTTP Testing**: Actix-web test utilities (`test::init_service`, `test::TestRequest`)
- **Async Testing**: Tokio runtime via `#[actix_web::test]` macro
- **JSON Handling**: serde_json for request/response validation

### Test Categories

#### Unit Tests (in auth_tests.rs)
- JWT token creation and validation
- Password hashing and verification
- User model serialization

#### API Tests (in api_tests.rs)
- Health check endpoint
- Product catalog endpoints
- Route structure validation

#### Integration Tests (in integration_tests.rs)
- Full user flow (create token → add product → add to cart → get cart)
- Multi-module interactions
- End-to-end scenarios

## Implementation Plan

### Step 1: Create Authentication Tests (tests/auth_tests.rs)

Test JWT lifecycle:
```rust
#[test]
fn test_jwt_creation_and_validation() {
    let user_id = "123";
    let token = create_token(user_id).unwrap();
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
}
```

Test password security:
```rust
#[test]
fn test_password_hashing_and_verification() {
    let password = "secure_password";
    let hashed = User::hash_password(password);
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hashed,
    };
    assert!(user.verify_password(password));
    assert!(!user.verify_password("wrong_password"));
}
```

**Design Note**: These are unit tests that validate Task 3's authentication module in isolation.

### Step 2: Create API Tests (tests/api_tests.rs)

Test health check:
```rust
#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}
```

Test product routes:
```rust
#[actix_web::test]
async fn test_product_routes() {
    let product_service = web::Data::new(ProductService::new());

    // Add test products
    product_service.create(/* ... */);

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
    let products: Vec<Product> = serde_json::from_slice(&body).unwrap();
    assert_eq!(products.len(), 2);
}
```

**Design Note**: These tests validate API endpoints with mocked services.

### Step 3: Create Integration Tests (tests/integration_tests.rs)

Test full user flow:
```rust
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
    let test_product = product_service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A test product".to_string(),
        price: Decimal::new(1999, 2), // $19.99
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
    let cart: Cart = serde_json::from_slice(&body).unwrap();

    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].product_id, test_product.id);
    assert_eq!(cart.items[0].quantity, 2);
}
```

**Design Note**: This end-to-end test validates the full integration of authentication, catalog, and cart modules.

### Step 4: Update main.rs for Testability

Ensure modules are accessible to tests:
```rust
// Make modules public for testing
pub mod api;
pub mod schema;
pub mod auth;
pub mod catalog;
pub mod cart;
```

Add conditional compilation for test-only code if needed:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Test helpers
}
```

### Step 5: Test Configuration

Add to `Cargo.toml`:
```toml
[[test]]
name = "integration_tests"
path = "tests/integration_tests.rs"

[[test]]
name = "api_tests"
path = "tests/api_tests.rs"

[[test]]
name = "auth_tests"
path = "tests/auth_tests.rs"
```

This allows running tests individually:
```bash
cargo test integration_tests
cargo test api_tests
cargo test auth_tests
```

## Architectural Considerations

### Test Isolation
- Each test creates its own service instances
- No shared state between tests
- Tests can run in parallel safely

### Test Data
- Use in-memory services (no database cleanup needed)
- Create test data programmatically
- Predictable IDs and values

### Async Testing
- `#[actix_web::test]` provides async runtime
- Tests can await futures
- Mimics real async server behavior

### Service Mocking
- Real services used (not mocks)
- Services are lightweight in-memory implementations
- Tests validate actual business logic

## Test Coverage

### Module Coverage
- ✅ Authentication (Task 3): Token and password tests
- ✅ Product Catalog (Task 4): CRUD operation tests
- ✅ Shopping Cart (Task 5): Cart operation tests with auth
- ✅ API Routing (Task 2): Endpoint accessibility tests
- ⚠️ Frontend (Task 6): Not covered (would need Selenium/Playwright)
- ⚠️ Database (Task 1): Not covered (schema is unused in tests)

### Scenario Coverage
1. **Happy Path**: User creates account, browses products, adds to cart
2. **Authentication**: Valid/invalid tokens, missing auth header
3. **Validation**: Product doesn't exist, insufficient inventory
4. **Edge Cases**: Empty cart, remove non-existent item

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All test files created
- Tests compile without errors
- All tests pass (`cargo test`)
- Full user flow test passes
- Authentication tests validate JWT security
- API tests validate all endpoints
- Integration tests validate module interactions

## Related Tasks
- **Task 2**: API Endpoints (tests the routing)
- **Task 3**: Authentication (tests JWT and passwords)
- **Task 4**: Catalog (tests product operations)
- **Task 5**: Cart (tests cart with auth and products)
- **Task 6**: Frontend (could add E2E tests in future)

## Risks and Considerations

1. **No Database Tests**: Schema from Task 1 is not tested as services use in-memory storage.

2. **No Frontend Tests**: Task 6 components not tested (would require different tools).

3. **Limited Error Scenarios**: Focus on happy path and basic error cases, not exhaustive edge case testing.

4. **Test Data Management**: Tests create data programmatically. Large test suites might benefit from fixtures.

5. **Async Timing**: No deliberate delays or race condition testing.

## Production Testing Improvements (Not in Scope)
- Database integration tests with test database
- Frontend E2E tests with Cypress/Playwright
- Load testing and performance benchmarks
- Chaos engineering (network failures, timeouts)
- Security testing (SQL injection, XSS, CSRF)
- Contract testing for API versioning
- Mutation testing for test quality
- Code coverage metrics (e.g., tarpaulin)

## References
- [Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Actix-web Testing](https://actix.rs/docs/testing/)
- Project PRD: `.taskmaster/docs/prd.txt`
- All previous task documentation
