# Task 5: Shopping Cart API

## Overview
Create shopping cart functionality and API endpoints with JWT authentication, product validation, and inventory checking. This is a Level 1 task that depends on Task 3 (User Authentication) and Task 4 (Product Catalog), integrating multiple modules into a complete cart management system.

## Context
This task brings together authentication and catalog modules to provide secure, user-specific shopping cart functionality. It demonstrates complex task dependency orchestration where multiple Level 0 tasks must complete before this Level 1 task can begin.

## Objectives
1. Implement cart service with user-specific cart management
2. Create cart models (Cart, CartItem)
3. Build REST API endpoints with JWT authentication
4. Validate products exist and have sufficient inventory
5. Support cart operations: get, add item, remove item, clear
6. Integrate with API routing from Task 2

## Dependencies
- **Task 3: User Authentication** - Required for JWT token validation
- **Task 4: Product Catalog** - Required for product lookup and inventory checking

## Files to Create/Modify
- `src/cart/mod.rs` - Cart module exports
- `src/cart/service.rs` - Cart service with business logic
- `src/api/cart_routes.rs` - Cart API endpoints with authentication
- `src/api/mod.rs` - Update to include cart_routes module
- `src/api/routes.rs` - Update to configure cart routes

## Technical Specifications

### Cart Data Models
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub items: Vec<CartItem>,
}
```

### API Endpoints
```
POST   /api/cart/add        - Add item to cart (authenticated)
GET    /api/cart            - Get user's cart (authenticated)
DELETE /api/cart/remove/:id - Remove item from cart (authenticated)
POST   /api/cart/clear      - Clear all items (authenticated)
```

### Authentication Flow
1. Extract Authorization header from request
2. Parse "Bearer {token}" format
3. Validate JWT token using Task 3's `validate_token`
4. Extract user_id from token claims
5. Use user_id for cart operations

## Implementation Plan

### Step 1: Create Cart Service (src/cart/service.rs)
Implement in-memory cart management with thread-safe operations:

#### Data Structures
```rust
pub struct CartService {
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    next_id: Arc<Mutex<i32>>,
}
```

Uses HashMap for O(1) cart lookup by cart_id, with user_id searchable within carts.

#### Core Methods

**`new() -> Self`**
- Initializes empty cart HashMap
- Sets next_id to 1

**`get_or_create_cart(&self, user_id: i32) -> Cart`**
- Searches existing carts for matching user_id
- Creates new cart if none exists
- Returns user's cart

**`add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart`**
- Gets or creates user's cart
- Checks if product already in cart (by product_id)
- If exists: increment quantity
- If new: add CartItem with product details
- Returns updated cart

**`remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart>`**
- Finds user's cart
- Removes item matching product_id
- Returns updated cart or None

**`get_cart(&self, user_id: i32) -> Option<Cart>`**
- Searches for cart by user_id
- Returns cart or None

**`clear_cart(&self, user_id: i32) -> Option<Cart>`**
- Finds user's cart
- Clears items vector
- Returns empty cart or None

### Step 2: Create API Routes (src/api/cart_routes.rs)
Implement authenticated endpoints:

#### Request DTOs
```rust
#[derive(Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}
```

#### Route Configuration
```rust
pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart))
    );
}
```

#### Authentication Helper Pattern
Each handler extracts and validates JWT:

```rust
// Extract Authorization header
let auth_header = req.headers().get("Authorization")?;
let auth_str = auth_header.to_str()?;

// Parse Bearer token
if auth_str.starts_with("Bearer ") {
    let token = &auth_str[7..];

    // Validate with Task 3's function
    let claims = validate_token(token)?;

    // Extract user_id
    let user_id = claims.sub.parse::<i32>()?;

    // Use user_id for cart operations
}
```

Returns `401 Unauthorized` if authentication fails at any step.

#### get_cart Handler
- Authenticates user
- Gets cart from service
- Returns cart JSON or creates empty cart

#### add_item Handler
- Authenticates user
- Parses AddItemRequest from body
- Looks up product with ProductService
- Validates inventory >= requested quantity
- Adds item to cart via CartService
- Returns updated cart JSON
- Returns 400 if insufficient inventory
- Returns 404 if product not found

#### remove_item Handler
- Authenticates user
- Extracts product_id from path parameter
- Removes item from cart
- Returns updated cart JSON
- Returns 404 if item not in cart

#### clear_cart Handler
- Authenticates user
- Clears all items from cart
- Returns empty cart JSON

### Step 3: Update API Module (src/api/mod.rs)
Add cart_routes to module exports:

```rust
pub mod routes;
pub mod cart_routes;
```

### Step 4: Update Route Configuration (src/api/routes.rs)
Integrate cart routes into main API:

```rust
use crate::api::cart_routes::configure_cart_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .configure(configure_cart_routes)  // Add cart routes
    );
}
```

Note: Cart routes already have `/cart` scope, so they nest under `/api/cart`.

### Step 5: Update Main Application (src/main.rs)
Register CartService with Actix-web application data:

```rust
use crate::cart::CartService;

let cart_service = web::Data::new(CartService::new());

HttpServer::new(move || {
    App::new()
        .app_data(product_service.clone())
        .app_data(cart_service.clone())  // Add cart service
        .configure(api::routes::configure_routes)
})
```

## Architectural Considerations

### Multi-Module Integration
This task integrates three previous tasks:
- **Task 2**: Provides routing infrastructure
- **Task 3**: Provides JWT validation
- **Task 4**: Provides product lookup

Clean dependency injection via Actix-web's `Data` extractor.

### Authentication Pattern
JWT-based stateless authentication:
- No server-side session storage
- User identity travels with each request
- Token validation on every cart operation
- Consistent auth pattern across all endpoints

### Data Denormalization
CartItem stores product_name and unit_price:
- **Pro**: Cart remains valid if product changes or is deleted
- **Pro**: Faster cart display (no additional product lookups)
- **Con**: Price updates don't affect existing carts
- **Real-world**: This matches e-commerce behavior (cart locks in price)

### Inventory Validation
Checks inventory at add time only:
- Prevents adding unavailable items
- Doesn't reserve inventory (race condition possible)
- Real-world: Would need distributed locking or optimistic concurrency

### Thread Safety
Same Arc<Mutex<>> pattern as ProductService:
- Safe for concurrent access
- HashMap provides O(1) cart lookup
- Short lock durations

## Risks and Considerations

1. **Race Conditions**: Inventory check then add is not atomic. Multiple users could add the last item simultaneously. Production needs pessimistic locking or optimistic concurrency control.

2. **Cart Expiration**: No TTL on carts. Memory grows unbounded. Production needs cart expiration and cleanup.

3. **Inventory Reservation**: Items in cart aren't reserved. Checkout might fail if inventory runs out. Production needs reservation system.

4. **Price Changes**: Cart stores snapshot price. Good for user experience but requires business logic for handling price changes at checkout.

5. **Authentication Error Handling**: Returns generic 401 Unauthorized. Production should distinguish between expired tokens, invalid tokens, etc.

6. **User ID Parsing**: Converts string user_id to i32. Could fail if JWT contains non-integer ID. Production needs robust error handling.

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All cart files created
- Cart service manages user-specific carts
- API endpoints require valid JWT authentication
- Products are validated before adding to cart
- Inventory is checked before adding items
- All cart operations work correctly
- Integration with Tasks 2, 3, and 4 succeeds

## Related Tasks
- **Task 2**: API Endpoints (provides routing infrastructure)
- **Task 3**: User Authentication (provides JWT validation)
- **Task 4**: Product Catalog (provides product lookup)
- **Task 7**: Integration Tests (will test complete cart flow)

## Diagram
See `diagrams.mmd` for visual representation of cart architecture, authentication flow, and module integration.

## Production Improvements (Not in Scope)
- Database persistence
- Cart expiration/TTL
- Inventory reservation system
- Optimistic locking for race conditions
- Better error types (not generic 401)
- Cart item quantity limits
- Calculate cart totals server-side
- Apply discounts/promotions
- Guest cart support (pre-authentication)
- Cart merging (guest → authenticated user)
- Websocket for real-time inventory updates

## References
- [Actix-web Data Extraction](https://actix.rs/docs/extractors/)
- [Actix-web Middleware](https://actix.rs/docs/middleware/)
- Project PRD: `.taskmaster/docs/prd.txt`
- Task 3 Authentication: `.taskmaster/docs/task-3/`
- Task 4 Catalog: `.taskmaster/docs/task-4/`
