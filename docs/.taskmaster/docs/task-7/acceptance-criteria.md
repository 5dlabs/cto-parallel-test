# Task 7: Integration Tests - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `tests/integration_tests.rs` exists
- [ ] `tests/api_tests.rs` exists
- [ ] `tests/auth_tests.rs` exists
- [ ] `src/main.rs` updated with public modules

## Code Quality Criteria

### ✅ Authentication Tests (tests/auth_tests.rs)
- [ ] Test JWT creation and validation
- [ ] Test password hashing produces different results
- [ ] Test correct password verification
- [ ] Test incorrect password verification fails
- [ ] Test User serialization skips password_hash
- [ ] All tests use `#[test]` attribute

### ✅ API Tests (tests/api_tests.rs)
- [ ] Test health check returns 200 OK with {"status":"ok"}
- [ ] Test product routes with test data
- [ ] Test GET /api/products returns all products
- [ ] Test GET /api/products/:id returns specific product
- [ ] All tests use `#[actix_web::test]` attribute
- [ ] Tests initialize app with test::init_service
- [ ] Tests use test::TestRequest for requests

### ✅ Integration Tests (tests/integration_tests.rs)
- [ ] Test full user flow (product creation → cart add → cart get)
- [ ] Creates ProductService and CartService
- [ ] Initializes test app with both services
- [ ] Creates test product programmatically
- [ ] Creates JWT token for test user
- [ ] POSTs to /api/cart/add with authentication
- [ ] Verifies 200 OK response
- [ ] GETs /api/cart with authentication
- [ ] Verifies cart contains correct item and quantity
- [ ] All tests use `#[actix_web::test]` attribute

### ✅ Main Module Updates
- [ ] src/main.rs declares modules as public (pub mod)
- [ ] Modules: api, schema, auth, catalog, cart

## Functional Criteria

### ✅ Test Execution
- [ ] `cargo test` completes without errors
- [ ] All tests in auth_tests pass
- [ ] All tests in api_tests pass
- [ ] All tests in integration_tests pass
- [ ] Tests can run individually: `cargo test auth_tests`
- [ ] Tests can run individually: `cargo test api_tests`
- [ ] Tests can run individually: `cargo test integration_tests`

### ✅ Test Coverage
- [ ] Authentication module tested (JWT, passwords)
- [ ] API endpoints tested (health, products)
- [ ] Product catalog operations tested
- [ ] Shopping cart operations tested
- [ ] Multi-module integration tested
- [ ] End-to-end user flow tested

### ✅ Assertions
- [ ] HTTP status codes verified
- [ ] JSON response structures validated
- [ ] Data correctness checked (IDs, quantities, etc.)
- [ ] Authentication headers tested
- [ ] Error cases handled (404, 401, etc.)

## Success Definition

**Task is COMPLETE when:**
1. All test files created
2. Tests compile without errors
3. `cargo test` passes all tests
4. Full user flow test validates end-to-end functionality
5. Module integration verified

**Task is INCOMPLETE if:**
- Any test file missing
- Compilation errors
- Any test fails
- Integration test doesn't cover full flow

## Estimated Completion Time
60 minutes (as specified in PRD)

## Dependencies
- Task 2: API Endpoints (required)
- Task 5: Shopping Cart API (required)
- Task 6: Frontend (optional, not tested)

## Test Commands

### Run All Tests
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test auth_tests
cargo test api_tests
cargo test integration_tests
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Run Single Test
```bash
cargo test test_full_user_flow -- --nocapture
```
