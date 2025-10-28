# Acceptance Criteria: Integration Tests

## Required Files

### ✅ `tests/integration_tests.rs`
- [ ] File exists at `tests/integration_tests.rs`
- [ ] Contains #[cfg(test)] module declaration
- [ ] Imports actix-web test utilities
- [ ] Imports necessary modules from crate (api::routes, auth::jwt, catalog, cart)
- [ ] Contains `test_health_check()` function with #[actix_web::test] attribute
- [ ] Health check test creates App with configure_routes
- [ ] Health check test sends GET request to /api/health
- [ ] Health check test asserts StatusCode::OK
- [ ] Health check test parses JSON response
- [ ] Health check test asserts json["status"] == "ok"
- [ ] Contains `test_full_user_flow()` function with #[actix_web::test] attribute
- [ ] User flow test initializes ProductService and CartService
- [ ] User flow test creates test product with create()
- [ ] User flow test generates JWT token with create_token()
- [ ] User flow test POSTs to /api/cart/add with Authorization header
- [ ] User flow test includes product_id and quantity in request
- [ ] User flow test asserts successful response
- [ ] User flow test GETs /api/cart with Authorization header
- [ ] User flow test parses Cart from response
- [ ] User flow test asserts cart has 1 item
- [ ] User flow test asserts item product_id and quantity are correct

### ✅ `tests/api_tests.rs`
- [ ] File exists at `tests/api_tests.rs`
- [ ] Contains #[cfg(test)] module declaration
- [ ] Imports actix-web test utilities
- [ ] Imports catalog::ProductService and models
- [ ] Contains `test_product_routes()` function with #[actix_web::test] attribute
- [ ] Creates ProductService with test data
- [ ] Adds at least 2 test products
- [ ] Tests GET /api/products endpoint
- [ ] Asserts StatusCode::OK for products list
- [ ] Parses Vec<Product> from response
- [ ] Asserts correct number of products returned
- [ ] Tests GET /api/products/1 endpoint
- [ ] Asserts StatusCode::OK for product detail
- [ ] Parses Product from response
- [ ] Asserts product.id and product.name are correct

### ✅ `tests/auth_tests.rs`
- [ ] File exists at `tests/auth_tests.rs`
- [ ] Contains #[cfg(test)] module declaration
- [ ] Imports auth::jwt functions (create_token, validate_token)
- [ ] Imports auth::models::User
- [ ] Contains `test_jwt_creation_and_validation()` function with #[test] attribute
- [ ] Creates token for test user_id
- [ ] Validates token successfully
- [ ] Asserts claims.sub matches user_id
- [ ] Contains `test_password_hashing_and_verification()` function with #[test] attribute
- [ ] Hashes test password
- [ ] Creates User with hashed password
- [ ] Verifies correct password returns true
- [ ] Verifies incorrect password returns false

### ✅ `src/main.rs`
- [ ] File imports all necessary modules (api, schema, auth, catalog, cart)
- [ ] Contains #[actix_web::main] attribute on main function
- [ ] Main function is async
- [ ] Initializes ProductService as web::Data
- [ ] Initializes CartService as web::Data
- [ ] HttpServer::new closure clones services
- [ ] App includes product_service via app_data()
- [ ] App includes cart_service via app_data()
- [ ] App configures routes with configure_routes
- [ ] Binds to 127.0.0.1:8080
- [ ] Returns std::io::Result<()>

## Functional Requirements

### Test Execution
- [ ] cargo test compiles all tests without errors
- [ ] cargo test runs all tests
- [ ] All integration tests pass
- [ ] All API tests pass
- [ ] All auth tests pass
- [ ] No test panics or failures
- [ ] Test output is clear and informative

### Health Check Test
- [ ] Verifies /api/health endpoint exists
- [ ] Confirms 200 OK status code
- [ ] Validates JSON response format
- [ ] Checks "status" field equals "ok"

### Full User Flow Test
- [ ] Creates test product successfully
- [ ] Generates valid JWT token
- [ ] Adds product to cart with authentication
- [ ] Retrieves cart successfully
- [ ] Verifies cart contents match expectations
- [ ] Tests complete e-commerce flow

### Product API Tests
- [ ] Tests listing all products
- [ ] Tests getting product by ID
- [ ] Verifies correct status codes
- [ ] Validates response data structure
- [ ] Confirms product data accuracy

### Authentication Tests
- [ ] JWT token can be created
- [ ] JWT token can be validated
- [ ] Token claims are correct
- [ ] Password hashing works
- [ ] Password verification works correctly
- [ ] Wrong password is rejected

### Main Application
- [ ] Services are initialized correctly
- [ ] Services are shared across handlers
- [ ] Routes are configured properly
- [ ] Application can be tested
- [ ] Application can run normally

## Validation Tests

### Compilation Check
```bash
cargo test --no-run
```
- [ ] Compiles without errors
- [ ] No warnings about unused imports
- [ ] All test modules compile
- [ ] Integration with actix-web works

### Test Execution Check
```bash
cargo test
```
- [ ] All tests execute
- [ ] Output shows test names
- [ ] All tests pass
- [ ] Clear pass/fail indication

### Test Output Verification
```
test integration_tests::test_health_check ... ok
test integration_tests::test_full_user_flow ... ok
test api_tests::test_product_routes ... ok
test auth_tests::test_jwt_creation_and_validation ... ok
test auth_tests::test_password_hashing_and_verification ... ok
```
- [ ] All tests show "ok" status
- [ ] No "FAILED" tests
- [ ] Test summary shows all passed

### Individual Test Runs
```bash
cargo test test_health_check
cargo test test_full_user_flow
cargo test test_product_routes
cargo test test_jwt_creation_and_validation
cargo test test_password_hashing_and_verification
```
- [ ] Each test can run independently
- [ ] Each test passes on its own

## Non-Functional Requirements

### Code Quality
- [ ] Tests follow Rust testing conventions
- [ ] Clear test names describe what is tested
- [ ] Tests are independent (no shared state)
- [ ] Test data is clear and descriptive
- [ ] Assertions are meaningful

### Test Organization
- [ ] Tests in tests/ directory (not src/)
- [ ] Separate files for different concerns
- [ ] Each file has focused responsibility
- [ ] Module structure is clear

### Test Coverage
- [ ] Health check endpoint covered
- [ ] Product CRUD operations covered
- [ ] Cart operations covered
- [ ] JWT lifecycle covered
- [ ] Password security covered
- [ ] Major integration points tested

### Maintainability
- [ ] Tests are easy to understand
- [ ] Test failures would be informative
- [ ] Tests can be extended easily
- [ ] No hard-coded magic values (or well-documented)

## Integration Readiness

### With Task 2 (API Endpoints)
- [ ] Can import configure_routes
- [ ] Health check endpoint works in tests
- [ ] Routes configuration works in test context

### With Task 3 (Authentication)
- [ ] Can import JWT functions
- [ ] Can create test tokens
- [ ] Can validate tokens
- [ ] Password functions work correctly

### With Task 4 (Product Catalog)
- [ ] Can import ProductService
- [ ] Can create test products
- [ ] Product methods work in tests
- [ ] Product serialization works

### With Task 5 (Shopping Cart)
- [ ] Can import CartService
- [ ] Cart operations work with JWT
- [ ] Cart integrates with products
- [ ] Full flow test proves integration

### With Task 6 (Frontend)
- [ ] Frontend files exist (can verify with file check)
- [ ] Backend API is ready for frontend integration

## Edge Cases and Error Handling

### Test Isolation
- [ ] Each test creates its own services
- [ ] No test affects another test
- [ ] Tests can run in any order
- [ ] Tests can run in parallel

### Error Scenarios
- [ ] Invalid JWT tokens would fail (tested implicitly)
- [ ] Missing products would return 404 (can add test)
- [ ] Unauthorized cart access would fail (tested in user flow)

### Data Management
- [ ] Test data is created fresh each test
- [ ] No database state to clean up
- [ ] In-memory storage is isolated per test

## Performance Considerations

- [ ] Tests complete in reasonable time
- [ ] No unnecessary delays or timeouts
- [ ] Async tests run efficiently
- [ ] Test suite can run frequently during development

## Success Metrics

- **Completion**: All test files created and main.rs updated
- **Compilation**: cargo test --no-run succeeds
- **Execution**: cargo test runs successfully
- **Pass Rate**: 100% of tests pass
- **Coverage**: Major integration points tested
- **Quality**: Tests are clear and maintainable

## Manual Verification Checklist

1. **File Existence**
   - [ ] tests/integration_tests.rs exists
   - [ ] tests/api_tests.rs exists
   - [ ] tests/auth_tests.rs exists
   - [ ] src/main.rs has been updated

2. **Test Implementation**
   - [ ] Health check test present
   - [ ] Full user flow test present
   - [ ] Product routes test present
   - [ ] JWT test present
   - [ ] Password test present

3. **Service Integration**
   - [ ] main.rs initializes ProductService
   - [ ] main.rs initializes CartService
   - [ ] Services passed to App correctly

4. **Compilation**
   - [ ] Run cargo test --no-run
   - [ ] Verify success

5. **Execution**
   - [ ] Run cargo test
   - [ ] Verify all tests pass
   - [ ] Check test count matches expectations

6. **Integration Verification**
   - [ ] Verify imports work from all dependency tasks
   - [ ] Check that services can be created in tests
   - [ ] Confirm JWT functions accessible

## Definition of Done

This task is complete when:
1. All 3 test files exist with correct implementations
2. src/main.rs updated with service initialization
3. All tests use correct imports with crate:: prefix
4. Health check test validates /api/health endpoint
5. Full user flow test covers product creation through cart operations
6. Product API tests verify endpoint functionality
7. Authentication tests validate JWT and password security
8. cargo test --no-run compiles successfully
9. cargo test executes all tests
10. All tests pass without failures
11. Test coverage includes major integration points
12. System integration is fully verified
