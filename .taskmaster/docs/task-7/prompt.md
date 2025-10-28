# Task 7: Integration Tests - Agent Prompt

You are a Rust test engineer tasked with creating comprehensive integration tests for the e-commerce API.

## Your Mission
Write integration tests that validate the complete application: authentication, product catalog, shopping cart, and end-to-end user flows. Ensure all modules work together correctly.

## What You Must Create

### 1. Create `tests/auth_tests.rs`
Unit tests for authentication:

**Test JWT creation and validation**:
- Create token with user ID
- Validate token
- Verify user ID extracted correctly

**Test password hashing and verification**:
- Hash a password
- Create User with hashed password
- Verify correct password returns true
- Verify incorrect password returns false

### 2. Create `tests/api_tests.rs`
API endpoint tests:

**Test health check**:
- Initialize test app with configure_routes
- GET /api/health
- Verify 200 OK response
- Verify JSON body {"status":"ok"}

**Test product routes**:
- Create ProductService with test products
- Initialize test app
- Test GET /api/products returns all products
- Test GET /api/products/:id returns specific product

### 3. Create `tests/integration_tests.rs`
Full user flow integration test:

**Test complete workflow**:
1. Create ProductService and CartService
2. Initialize test app with both services
3. Create test product in ProductService
4. Create JWT token for test user (ID: "1")
5. POST /api/cart/add with token and product
6. Verify 200 OK response
7. GET /api/cart with token
8. Verify cart contains correct item with quantity

### 4. Update `src/main.rs`
Make modules public for testing:
```rust
pub mod api;
pub mod schema;
pub mod auth;
pub mod catalog;
pub mod cart;
```

## Key Requirements

✅ **Test Framework**:
- Use `#[test]` for unit tests
- Use `#[actix_web::test]` for API tests
- Use actix_web::test utilities (init_service, TestRequest)

✅ **Test Coverage**:
- Authentication (JWT, passwords)
- API endpoints (health, products)
- Full user flow (create product → add to cart → get cart)

✅ **Assertions**:
- Verify HTTP status codes
- Verify JSON response structures
- Verify data correctness

## Success Definition
Task is complete when:
- All 3 test files created
- `cargo test` passes all tests
- Tests cover auth, API, and integration scenarios
- Full user flow test validates end-to-end functionality

## Context
**Your dependencies**:
- Task 2: API routing (test endpoints)
- Task 3: Authentication (test JWT)
- Task 4: Catalog (test products)
- Task 5: Cart (test cart operations)

This is a **Level 2** task - final validation of the system.

---

**Start working now. Create comprehensive tests and verify the entire system works.**
