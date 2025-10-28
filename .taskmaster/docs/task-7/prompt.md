# Autonomous Agent Prompt: Integration Tests

## Mission
You are tasked with creating comprehensive integration tests for the Rust API project. These tests verify that all system components work together correctly, covering the complete user flow from authentication through cart operations, API endpoints, and authentication mechanisms.

## Prerequisites

**IMPORTANT**: This is a Level 2 task with dependencies on:
- **Task 2 (API Endpoints)** - Must be complete for route configuration
- **Task 5 (Shopping Cart API)** - Must be complete for cart functionality
- **Task 6 (Frontend Components)** - Should exist (file check)

Before starting, verify these modules exist:
- `src/api/routes.rs` with `configure_routes()` function
- `src/cart/service.rs` with `CartService`
- `src/catalog/service.rs` with `ProductService`
- `src/auth/jwt.rs` with `create_token()` and `validate_token()`

If these are not available, WAIT for dependency tasks to complete.

## What You Need to Do

### 1. Create Integration Tests (`tests/integration_tests.rs`)

Create the file with health check and full user flow tests - use complete code from task.txt section 1.

**Key test functions:**
- `test_health_check()`: Verify /api/health returns 200 OK with {"status": "ok"}
- `test_full_user_flow()`: Test complete flow:
  1. Create ProductService and CartService
  2. Create test product
  3. Generate JWT token for user ID 1
  4. POST to /api/cart/add with product and quantity
  5. GET /api/cart to retrieve cart
  6. Assert cart contains correct product and quantity

### 2. Create API Tests (`tests/api_tests.rs`)

Create the file with product endpoint tests - use complete code from task.txt section 2.

**Key test function:**
- `test_product_routes()`:
  - Create ProductService with 2 test products
  - Test GET /api/products returns all products
  - Test GET /api/products/1 returns correct product by ID
  - Assert response status codes and data correctness

### 3. Create Authentication Tests (`tests/auth_tests.rs`)

Create the file with JWT and password tests - use complete code from task.txt section 3.

**Key test functions:**
- `test_jwt_creation_and_validation()`:
  - Create token for user ID "123"
  - Validate token
  - Assert claims.sub matches user ID

- `test_password_hashing_and_verification()`:
  - Hash password "secure_password"
  - Create User with hashed password
  - Verify correct password returns true
  - Verify wrong password returns false

### 4. Update Main Application (`src/main.rs`)

Modify src/main.rs to properly initialize services - use code from task.txt section 4.

**Key changes:**
- Import all modules: api, schema, auth, catalog, cart
- Initialize ProductService and CartService as web::Data
- Clone services in HttpServer closure
- Pass services to App via app_data()
- Configure routes

## Expected Behavior

After implementation:
- Running `cargo test` executes all tests
- All tests pass without errors
- Health check test verifies API is responding
- User flow test proves end-to-end integration works
- API tests validate product endpoints
- Auth tests confirm JWT and password security
- Application is properly testable

## Validation Steps

Before marking complete, verify:

1. **File Structure**
   ```bash
   ls -la tests/integration_tests.rs
   ls -la tests/api_tests.rs
   ls -la tests/auth_tests.rs
   ```

2. **Compilation Check**
   ```bash
   cargo test --no-run
   ```
   Should compile all tests without errors.

3. **Run Tests**
   ```bash
   cargo test
   ```
   Should show all tests passing:
   - test integration_tests::test_health_check ... ok
   - test integration_tests::test_full_user_flow ... ok
   - test api_tests::test_product_routes ... ok
   - test auth_tests::test_jwt_creation_and_validation ... ok
   - test auth_tests::test_password_hashing_and_verification ... ok

4. **Check Test Output**
   Verify no compilation errors, no test failures, no panics.

## Constraints

- This is a test project - tests verify placeholder functionality
- Wait for Tasks 2, 5 to complete before starting
- Use actix-web test utilities for API tests
- Use standard #[test] for unit tests
- Import from crate using `use crate::module::...`
- Each test should be independent (no shared state)
- Create services fresh for each test

## Common Issues and Solutions

**Issue**: Cannot find `configure_routes` or other imports
- **Solution**: Ensure Task 2 is complete and modules are properly declared

**Issue**: Cannot create JWT tokens
- **Solution**: Ensure Task 3 is complete with auth::jwt module

**Issue**: CartService or ProductService not found
- **Solution**: Ensure Tasks 4 and 5 are complete

**Issue**: Tests compile but fail
- **Solution**: Check that services are initialized correctly in tests

**Issue**: #[actix_web::test] macro not found
- **Solution**: Ensure actix-web is in dev-dependencies with macros feature

## Success Definition

Task is complete when:
- ✅ 3 test files created: integration_tests.rs, api_tests.rs, auth_tests.rs
- ✅ src/main.rs updated with service initialization
- ✅ All test files use correct imports (crate::)
- ✅ Health check test implemented
- ✅ Full user flow test implemented
- ✅ Product API tests implemented
- ✅ JWT and password tests implemented
- ✅ cargo test --no-run compiles successfully
- ✅ cargo test runs all tests
- ✅ All tests pass

## Integration Notes

### With Task 2 (API Endpoints)
- Import: `use crate::api::routes::configure_routes;`
- Use: Pass to App with `.configure(configure_routes)`
- Health check must be at /api/health

### With Task 3 (Authentication)
- Import: `use crate::auth::jwt::{create_token, validate_token};`
- Import: `use crate::auth::models::User;`
- Use: `create_token("user_id")` for test tokens
- Use: `validate_token(token)` to verify tokens

### With Task 4 (Product Catalog)
- Import: `use crate::catalog::ProductService;`
- Import: `use crate::catalog::models::{Product, NewProduct};`
- Use: `ProductService::new()` to create service
- Use: `product_service.create(...)` to add test products

### With Task 5 (Shopping Cart)
- Import: `use crate::cart::CartService;`
- Use: `CartService::new()` to create service
- Test: Cart endpoints with JWT authentication
- Verify: Product integration with cart

## Next Steps

After completing this task:
1. All 7 tasks are complete
2. Full system integration is verified
3. Project demonstrates parallel task execution
4. Tests serve as documentation of system behavior
5. Platform can measure actual vs theoretical speedup
