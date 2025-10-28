# Acceptance Criteria: Shopping Cart API

## Required Files

### ✅ `src/cart/mod.rs`
- [ ] File exists at `src/cart/mod.rs`
- [ ] Contains `pub mod service;` declaration
- [ ] Contains `pub use self::service::CartService;` export
- [ ] File is properly formatted

### ✅ `src/cart/service.rs`
- [ ] File exists at `src/cart/service.rs`
- [ ] Contains `CartItem` struct with fields: product_id (i32), quantity (i32), product_name (String), unit_price (Decimal)
- [ ] `CartItem` derives Debug, Clone, Serialize, and Deserialize
- [ ] Contains `Cart` struct with fields: id (i32), user_id (i32), items (Vec<CartItem>)
- [ ] `Cart` derives Debug, Clone, Serialize, and Deserialize
- [ ] Contains `CartService` struct with fields: carts (Arc<Mutex<HashMap<i32, Cart>>>), next_id (Arc<Mutex<i32>>)
- [ ] Implements `new()` constructor that initializes empty storage with next_id = 1
- [ ] Implements `get_or_create_cart(&self, user_id: i32) -> Cart` method
- [ ] `get_or_create_cart()` returns existing cart or creates new one for user
- [ ] Implements `add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart` method
- [ ] `add_item()` accumulates quantity if product already in cart
- [ ] `add_item()` creates new CartItem with product snapshot if not in cart
- [ ] Implements `remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart>` method
- [ ] `remove_item()` uses retain() to filter out the product
- [ ] Implements `get_cart(&self, user_id: i32) -> Option<Cart>` method
- [ ] Implements `clear_cart(&self, user_id: i32) -> Option<Cart>` method
- [ ] All methods use proper mutex locking
- [ ] Returns cloned carts to avoid holding locks
- [ ] All necessary imports included (HashMap, Arc, Mutex, Product, rust_decimal)

### ✅ `src/api/cart_routes.rs`
- [ ] File exists at `src/api/cart_routes.rs`
- [ ] Contains `AddItemRequest` struct with fields: product_id (i32), quantity (i32)
- [ ] `AddItemRequest` derives Deserialize
- [ ] Contains `configure_cart_routes(cfg: &mut web::ServiceConfig)` function
- [ ] Configures four routes: GET "", POST "/add", DELETE "/remove/{product_id}", POST "/clear"
- [ ] Implements `get_cart` handler with JWT authentication
- [ ] Implements `add_item` handler with JWT authentication and product validation
- [ ] Implements `remove_item` handler with JWT authentication
- [ ] Implements `clear_cart` handler with JWT authentication
- [ ] All handlers extract user_id from JWT token
- [ ] All handlers check for "Bearer " prefix in Authorization header
- [ ] All handlers call `validate_token()` from auth::jwt
- [ ] All handlers return 401 Unauthorized for auth failures
- [ ] `add_item` validates product exists using ProductService
- [ ] `add_item` checks inventory before adding to cart
- [ ] `add_item` returns 404 Not Found if product doesn't exist
- [ ] `add_item` returns 400 Bad Request if insufficient inventory
- [ ] Proper error JSON responses with "error" field
- [ ] All necessary imports included (actix_web, CartService, ProductService, validate_token, serde_json)

### ✅ `src/api/mod.rs`
- [ ] File contains `pub mod cart_routes;` declaration
- [ ] cart_routes module is properly exported

### ✅ `src/api/routes.rs`
- [ ] File imports `configure_cart_routes` from cart_routes module
- [ ] `configure_routes` function calls `configure_cart_routes`
- [ ] Cart routes are configured under /api scope
- [ ] Integration with existing routes is correct

## Functional Requirements

### Authentication
- [ ] All cart endpoints require valid JWT token
- [ ] JWT token extracted from Authorization header
- [ ] Token must have "Bearer " prefix
- [ ] User ID extracted from JWT claims.sub field
- [ ] Invalid tokens return 401 Unauthorized
- [ ] Missing Authorization header returns 401 Unauthorized
- [ ] Malformed Authorization header returns 401 Unauthorized

### Cart Management
- [ ] Users can retrieve their cart
- [ ] First cart access creates cart automatically
- [ ] Each user has isolated cart (no cross-user access)
- [ ] Cart persists across requests (in-memory)
- [ ] Cart ID is auto-generated and unique

### Adding Items
- [ ] Users can add products to cart
- [ ] Product must exist (validated via ProductService)
- [ ] Inventory must be sufficient
- [ ] Duplicate products accumulate quantity
- [ ] CartItem stores product snapshot (name, price)
- [ ] Returns updated cart after adding

### Removing Items
- [ ] Users can remove products from cart by product_id
- [ ] Removes entire product (not just decrement quantity)
- [ ] Returns updated cart after removal
- [ ] Returns 404 if item not in cart

### Clearing Cart
- [ ] Users can clear all items from cart
- [ ] Cart still exists but items list is empty
- [ ] Returns cleared cart

## Validation Tests

### Dependency Check
```bash
grep -r "validate_token" src/auth/jwt.rs
grep -r "ProductService" src/catalog/
```
- [ ] Task 3 JWT validation is available
- [ ] Task 4 ProductService is available

### Compilation Check
```bash
cargo check
```
- [ ] Command executes without errors
- [ ] No compilation warnings
- [ ] All dependencies resolve correctly
- [ ] All imports work correctly

### File Structure Check
```bash
ls -la src/cart/
ls -la src/api/cart_routes.rs
cat src/api/mod.rs | grep cart_routes
cat src/api/routes.rs | grep configure_cart_routes
```
- [ ] All required files exist
- [ ] Files are in correct locations
- [ ] Module declarations are correct

### Module Import Check
- [ ] Cart module can be imported: `use crate::cart::CartService;`
- [ ] Cart routes can be imported: `use crate::api::cart_routes::configure_cart_routes;`
- [ ] Service methods are callable
- [ ] No visibility issues with pub/private

## Non-Functional Requirements

### Code Quality
- [ ] Code follows Rust naming conventions
- [ ] Proper use of Rust idioms (Option, Result, pattern matching)
- [ ] No unnecessary complexity
- [ ] Code is well-organized and readable
- [ ] Appropriate use of references vs owned values
- [ ] Consistent error handling pattern across handlers

### Thread Safety
- [ ] CartService can be safely shared across threads
- [ ] Arc<Mutex<HashMap>> pattern used correctly
- [ ] No data races possible
- [ ] Mutex locks are acquired and released properly
- [ ] No deadlock potential
- [ ] Service can be used as web::Data in actix-web

### Security
- [ ] JWT validation prevents unauthorized access
- [ ] Users can only access their own carts
- [ ] No user ID spoofing possible
- [ ] Proper authentication on all endpoints
- [ ] Sensitive operations require authentication

### API Design
- [ ] RESTful route structure
- [ ] Appropriate HTTP methods (GET, POST, DELETE)
- [ ] Consistent response format
- [ ] Proper HTTP status codes (200, 400, 401, 404)
- [ ] JSON responses for data and errors

## Integration Readiness

### With Task 3 (Authentication)
- [ ] Uses validate_token() from auth::jwt module
- [ ] Correctly parses JWT claims
- [ ] Handles authentication errors gracefully
- [ ] User ID extraction works correctly

### With Task 4 (Product Catalog)
- [ ] Uses ProductService::get_by_id() for validation
- [ ] Checks product.inventory_count correctly
- [ ] Stores product snapshot in CartItem
- [ ] Handles non-existent products gracefully

### For Task 7 (Integration Tests)
- [ ] CartService can be instantiated and shared
- [ ] Endpoints can be tested with actix_web::test
- [ ] Full user flow is testable
- [ ] No blocking issues for integration tests

## Edge Cases and Error Handling

### Authentication Errors
- [ ] Missing Authorization header handled
- [ ] Invalid token format handled
- [ ] Expired token handled (if validate_token checks)
- [ ] Malformed Bearer token handled

### Product Validation Errors
- [ ] Non-existent product_id handled
- [ ] Insufficient inventory handled
- [ ] Zero or negative quantity handled (if validated)

### Cart Operation Errors
- [ ] Removing non-existent item handled
- [ ] Empty cart operations handled
- [ ] Concurrent modifications handled safely

### Data Consistency
- [ ] Product snapshot prevents stale data issues
- [ ] Cart state is consistent after operations
- [ ] No orphaned carts or items

## Performance Considerations

- [ ] Mutex locks released quickly (carts cloned before returning)
- [ ] No long-held locks that could block other threads
- [ ] HashMap provides O(1) cart lookup
- [ ] Acceptable performance for in-memory test implementation
- [ ] User lookup is O(n) but acceptable for test scale

## Success Metrics

- **Completion**: All required files created with correct content
- **Quality**: Code passes `cargo check` without errors or warnings
- **Functionality**: All CRUD operations work correctly
- **Security**: JWT authentication protects all endpoints
- **Integration**: Successfully integrates with Tasks 3 and 4
- **Readiness**: Module is ready for Task 7 to test

## Manual Verification Checklist

1. **File Existence**
   - [ ] Check `src/cart/mod.rs` exists
   - [ ] Check `src/cart/service.rs` exists
   - [ ] Check `src/api/cart_routes.rs` exists
   - [ ] Check `src/api/mod.rs` declares cart_routes
   - [ ] Check `src/api/routes.rs` configures cart routes

2. **Service Implementation**
   - [ ] Verify CartItem and Cart structs
   - [ ] Verify CartService has correct fields
   - [ ] Verify all 5 methods exist: new, get_or_create_cart, add_item, remove_item, get_cart, clear_cart
   - [ ] Check Arc<Mutex<HashMap>> pattern

3. **API Routes Implementation**
   - [ ] Verify AddItemRequest struct
   - [ ] Verify configure_cart_routes function
   - [ ] Verify all 4 handler functions exist
   - [ ] Check JWT authentication in all handlers
   - [ ] Check product validation in add_item
   - [ ] Check inventory checking in add_item

4. **Integration Points**
   - [ ] Confirm use of validate_token from auth::jwt
   - [ ] Confirm use of ProductService from catalog
   - [ ] Verify proper error responses

5. **Compilation**
   - [ ] Run `cargo check` and verify success
   - [ ] Ensure no warnings or errors
   - [ ] Verify all imports resolve

## Automated Testing (Conceptual)

While this task doesn't require implementing tests, the following should be testable:

```rust
// Example test flow (not required to implement)

// Setup
let cart_service = CartService::new();
let product_service = ProductService::new();

// Create test product
let product = product_service.create(NewProduct {
    name: "Test Product".to_string(),
    description: "Description".to_string(),
    price: Decimal::new(1999, 2),
    inventory_count: 10,
});

// Get or create cart
let cart = cart_service.get_or_create_cart(1);
assert_eq!(cart.user_id, 1);
assert_eq!(cart.items.len(), 0);

// Add item to cart
let cart = cart_service.add_item(1, &product, 2);
assert_eq!(cart.items.len(), 1);
assert_eq!(cart.items[0].quantity, 2);

// Add same item again (should accumulate)
let cart = cart_service.add_item(1, &product, 3);
assert_eq!(cart.items.len(), 1);
assert_eq!(cart.items[0].quantity, 5);

// Remove item
let cart = cart_service.remove_item(1, product.id).unwrap();
assert_eq!(cart.items.len(), 0);

// API endpoint test
let req = test::TestRequest::post()
    .uri("/api/cart/add")
    .header("Authorization", "Bearer valid_token")
    .set_json(json!({"product_id": 1, "quantity": 2}))
    .to_request();

let resp = test::call_service(&app, req).await;
assert_eq!(resp.status(), StatusCode::OK);
```

## Definition of Done

This task is complete when:
1. All required files exist with correct implementations
2. CartService implements all methods correctly
3. All API endpoints implement JWT authentication
4. Product validation and inventory checking work
5. Thread-safe storage with Arc<Mutex<HashMap>> is implemented
6. API module configuration is updated
7. Code passes `cargo check` without errors
8. Integration with Tasks 3 and 4 is correct
9. Proper error handling and HTTP status codes
10. All acceptance criteria checkboxes can be marked complete
11. Module is ready for Task 7 integration testing
