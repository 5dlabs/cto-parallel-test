# Task 5: Shopping Cart API - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `src/cart/mod.rs` exists
- [ ] `src/cart/service.rs` exists
- [ ] `src/api/cart_routes.rs` exists
- [ ] `src/api/mod.rs` updated to include cart_routes
- [ ] `src/api/routes.rs` updated to configure cart routes
- [ ] `src/main.rs` updated to register CartService

## Code Quality Criteria

### ✅ Cart Service (src/cart/service.rs)
- [ ] Imports models from auth and catalog modules
- [ ] CartItem struct with product_id, quantity, product_name, unit_price
- [ ] Cart struct with id, user_id, items vector
- [ ] CartService uses Arc<Mutex<HashMap<i32, Cart>>>
- [ ] Implements new() method
- [ ] Implements get_or_create_cart() - finds or creates user's cart
- [ ] Implements add_item() - adds/updates items, denormalizes product data
- [ ] Implements remove_item() - removes by product_id
- [ ] Implements get_cart() - returns user's cart
- [ ] Implements clear_cart() - empties items vector

### ✅ API Routes (src/api/cart_routes.rs)
- [ ] Imports CartService, ProductService, validate_token
- [ ] AddItemRequest struct with product_id and quantity
- [ ] configure_cart_routes() creates /cart scope with 4 routes
- [ ] All handlers extract and validate JWT from Authorization header
- [ ] All handlers return 401 if authentication fails
- [ ] get_cart handler returns user's cart or creates empty one
- [ ] add_item handler validates product exists (404 if not)
- [ ] add_item handler checks inventory (400 if insufficient)
- [ ] add_item handler adds to cart and returns updated cart
- [ ] remove_item handler extracts product_id from path
- [ ] remove_item handler removes item and returns updated cart
- [ ] clear_cart handler empties cart and returns it

### ✅ Module Integration
- [ ] src/api/mod.rs includes cart_routes module
- [ ] src/api/routes.rs imports configure_cart_routes
- [ ] src/api/routes.rs calls configure_cart_routes in configure_routes
- [ ] src/main.rs declares cart module
- [ ] src/main.rs creates CartService instance
- [ ] src/main.rs registers cart_service with app_data

## Functional Criteria

### ✅ Cart Management
- [ ] New users get empty carts
- [ ] Each user has exactly one cart (by user_id)
- [ ] Adding existing product increments quantity
- [ ] Adding new product creates CartItem
- [ ] CartItem stores product_name and unit_price at add time
- [ ] Removing item deletes CartItem from vector
- [ ] Clearing cart empties items but cart persists
- [ ] Cart operations are user-specific (no cross-user access)

### ✅ Authentication
- [ ] All endpoints require Authorization header
- [ ] Header must have "Bearer {token}" format
- [ ] Token must be valid JWT from Task 3
- [ ] User ID extracted from claims.sub
- [ ] Missing token returns 401
- [ ] Invalid token returns 401
- [ ] Expired token returns 401 (via validate_token)

### ✅ Product Validation
- [ ] add_item looks up product with ProductService
- [ ] Returns 404 if product doesn't exist
- [ ] Checks product.inventory_count >= requested quantity
- [ ] Returns 400 with error message if insufficient inventory
- [ ] Successfully adds if product exists with sufficient stock

### ✅ API Response Formats
- [ ] get_cart returns Cart JSON
- [ ] add_item returns updated Cart JSON
- [ ] remove_item returns updated Cart JSON
- [ ] clear_cart returns empty Cart JSON
- [ ] 400 errors return JSON with "error" field
- [ ] 404 errors return JSON with "error" field
- [ ] 401 errors return empty body

## Integration Criteria

### ✅ Task 3 Integration (Authentication)
- [ ] Successfully imports validate_token function
- [ ] validate_token correctly validates JWT tokens
- [ ] Claims extraction works (claims.sub contains user_id)
- [ ] Authentication pattern works across all endpoints

### ✅ Task 4 Integration (Catalog)
- [ ] Successfully imports ProductService
- [ ] Successfully imports Product model
- [ ] get_by_id returns products correctly
- [ ] Product fields (id, name, price, inventory_count) accessible
- [ ] ProductService instance available via web::Data

### ✅ Task 2 Integration (Routing)
- [ ] Cart routes nest under /api scope correctly
- [ ] configure_cart_routes works with ServiceConfig
- [ ] Routes accessible at /api/cart/*
- [ ] Handlers work with Actix-web extractors (web::Data, web::Json, web::Path)

## Compilation and Testing Criteria

### ✅ Build Verification
- [ ] cargo check completes without errors
- [ ] cargo build completes successfully
- [ ] All module imports resolve
- [ ] No circular dependency issues

### ✅ Runtime Testing

**Test 1: Authenticated Cart Access**
```bash
# Create token (from Task 3)
TOKEN=$(create_test_token "1")

# Get empty cart
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/api/cart
# Expected: {"id":1,"user_id":1,"items":[]}
```

**Test 2: Add Item to Cart**
```bash
# First create a product (Task 4)
# Then add to cart
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"product_id":1,"quantity":2}' \
  http://localhost:8080/api/cart/add
# Expected: Cart with 1 item, quantity 2
```

**Test 3: Product Validation**
```bash
# Try non-existent product
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"product_id":9999,"quantity":1}' \
  http://localhost:8080/api/cart/add
# Expected: 404 with error message
```

**Test 4: Inventory Validation**
```bash
# Try to add more than available
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"product_id":1,"quantity":1000}' \
  http://localhost:8080/api/cart/add
# Expected: 400 with "Not enough inventory"
```

**Test 5: Remove Item**
```bash
curl -X DELETE \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/api/cart/remove/1
# Expected: Cart without product 1
```

**Test 6: Clear Cart**
```bash
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/api/cart/clear
# Expected: Cart with empty items array
```

**Test 7: Authentication Required**
```bash
# Try without token
curl http://localhost:8080/api/cart
# Expected: 401 Unauthorized
```

- [ ] All runtime tests pass
- [ ] Authenticated requests work
- [ ] Unauthenticated requests return 401
- [ ] Product validation works
- [ ] Inventory validation works

## Success Definition

**Task is COMPLETE when:**
1. All required files created/updated
2. Cart service manages user-specific carts
3. API endpoints authenticate with JWT
4. Products validated before adding
5. Inventory checked before adding
6. All cart operations work correctly
7. Integrates successfully with Tasks 2, 3, and 4
8. All tests pass

**Task is INCOMPLETE if:**
- Any required file missing or not updated
- Compilation errors exist
- Authentication doesn't work
- Product validation missing
- Inventory checking missing
- Cart operations fail
- Module integration fails

## Estimated Completion Time
45 minutes (as specified in PRD)

## Dependencies
- Task 3: User Authentication (required)
- Task 4: Product Catalog (required)
- Task 2: API Endpoints (provides infrastructure)

## Blocks
- Task 7: Integration Tests (needs cart API)
