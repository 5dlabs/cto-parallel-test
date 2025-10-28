# Task 5: Shopping Cart API - Agent Prompt

You are a Rust backend developer tasked with implementing shopping cart functionality with JWT authentication and product validation.

## Your Mission
Create a complete shopping cart system that integrates authentication (Task 3) and product catalog (Task 4). Implement cart service, API endpoints with JWT authentication, product validation, and inventory checking.

## What You Must Create

### 1. Create `src/cart/mod.rs`
```rust
pub mod service;

pub use self::service::CartService;
```

### 2. Create `src/cart/service.rs`
Implement thread-safe cart management:

**Imports**:
- `use crate::auth::models::User;`
- `use crate::catalog::models::Product;`
- `use serde::{Serialize, Deserialize};`
- `use std::sync::{Arc, Mutex};`
- `use std::collections::HashMap;`
- `use rust_decimal::Decimal;`

**CartItem struct**:
- `product_id: i32`
- `quantity: i32`
- `product_name: String`
- `unit_price: Decimal`
- Derive `Debug, Clone, Serialize, Deserialize`

**Cart struct**:
- `id: i32`
- `user_id: i32`
- `items: Vec<CartItem>`
- Derive `Debug, Clone, Serialize, Deserialize`

**CartService struct**:
- `carts: Arc<Mutex<HashMap<i32, Cart>>>`
- `next_id: Arc<Mutex<i32>>`

**Methods to implement**:
- `new() -> Self` - Initialize with empty HashMap
- `get_or_create_cart(&self, user_id: i32) -> Cart` - Find or create user's cart
- `add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart` - Add/update item
- `remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart>` - Remove item
- `get_cart(&self, user_id: i32) -> Option<Cart>` - Get user's cart
- `clear_cart(&self, user_id: i32) -> Option<Cart>` - Clear all items

### 3. Create `src/api/cart_routes.rs`
Implement authenticated API endpoints:

**Imports**:
- `use actix_web::{web, HttpResponse, Responder, HttpRequest};`
- `use serde::{Serialize, Deserialize};`
- `use crate::cart::CartService;`
- `use crate::catalog::ProductService;`
- `use crate::auth::jwt::validate_token;`

**AddItemRequest struct**:
- `product_id: i32`
- `quantity: i32`
- Derive `Deserialize`

**configure_cart_routes function**:
- Create `/cart` scope with 4 routes
- GET "" → get_cart
- POST "/add" → add_item
- DELETE "/remove/{product_id}" → remove_item
- POST "/clear" → clear_cart

**Authentication pattern for all handlers**:
1. Extract Authorization header
2. Check "Bearer " prefix
3. Extract token
4. Validate with `validate_token(token)`
5. Parse user_id from claims.sub
6. Return 401 if any step fails

**get_cart handler**: Return user's cart or create empty one

**add_item handler**:
- Validate JWT
- Parse AddItemRequest
- Lookup product with ProductService
- Check inventory >= quantity
- Add to cart
- Return cart JSON

**remove_item handler**: Remove item by product_id from path

**clear_cart handler**: Clear all items from cart

### 4. Update `src/api/mod.rs`
Add: `pub mod cart_routes;`

### 5. Update `src/api/routes.rs`
Import: `use crate::api::cart_routes::configure_cart_routes;`
Add to configure_routes: `.configure(configure_cart_routes)`

### 6. Update `src/main.rs`
Add cart module: `mod cart;`
Create cart service: `let cart_service = web::Data::new(CartService::new());`
Add to app: `.app_data(cart_service.clone())`

## Key Requirements

✅ **Dependencies**:
- Import and use `validate_token` from Task 3
- Import and use `ProductService` from Task 4
- Import and use `Product` model from Task 4

✅ **Authentication**:
- All cart endpoints require valid JWT
- Extract user_id from token claims
- Return 401 Unauthorized if authentication fails

✅ **Product Validation**:
- Look up product before adding to cart
- Return 404 if product doesn't exist
- Check inventory before adding
- Return 400 if insufficient inventory

✅ **Cart Logic**:
- One cart per user (identified by user_id)
- If product already in cart, increment quantity
- Store product_name and unit_price in CartItem
- Support add, get, remove, clear operations

## Constraints
- This is a **Level 1** task depending on Tasks 3 and 4
- Must successfully import from auth and catalog modules
- Use in-memory storage (HashMap)
- Keep implementations straightforward

## Validation
After completing the work:
1. Verify all files exist and compile
2. Test cart creation for new users
3. Test adding products with authentication
4. Test inventory validation
5. Test removing items
6. Test clearing cart
7. Test 401 responses for missing/invalid tokens

## Success Definition
Task is complete when:
- All cart files created
- Cart service manages user-specific carts
- API endpoints authenticate with JWT
- Products validated before adding
- Inventory checked before adding
- All operations work correctly
- Integrates with Tasks 2, 3, and 4

## Context
**Your dependencies**:
- Task 3: User Authentication (JWT validation)
- Task 4: Product Catalog (product lookup)
- Task 2: API Endpoints (routing infrastructure)

**Tasks depending on you**:
- Task 7: Integration Tests (will test cart flow)

---

**Start working now. Create the files, integrate the modules, and verify cart operations work with authentication.**
