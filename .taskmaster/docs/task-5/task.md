# Task 5: Shopping Cart API

## Overview
Create shopping cart functionality and API endpoints with JWT authentication. This is a Level 1 task that depends on Task 3 (User Authentication) and Task 4 (Product Catalog), demonstrating the dependency management capabilities of the parallel task execution system.

## Context
This task is part of the parallel task execution test project. It integrates authentication from Task 3 with the product catalog from Task 4 to provide a complete shopping cart experience. The implementation uses in-memory storage with thread-safe patterns and JWT-based authentication for all cart operations.

## Objectives
1. Create cart service module in `src/cart/mod.rs` and `src/cart/service.rs`
2. Implement CartService with user-specific cart management
3. Create cart API endpoints in `src/api/cart_routes.rs`
4. Integrate JWT authentication for all cart operations
5. Update API module configuration to include cart routes

## Dependencies

**Depends On:**
- **Task 3 (User Authentication Module)** - Level 0 - Provides JWT token validation and authentication
- **Task 4 (Product Catalog Module)** - Level 0 - Provides product validation and inventory checking

**Depended Upon By:**
- **Task 7 (Integration Tests)** - Level 2 - Will test complete cart functionality

## Files to Create/Modify

### 1. `src/cart/mod.rs`
Module declaration file that exports cart components:

```rust
pub mod service;

pub use self::service::CartService;
```

### 2. `src/cart/service.rs`
Cart service with user-specific cart management and in-memory storage:

```rust
use crate::auth::models::User;
use crate::catalog::models::Product;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub items: Vec<CartItem>,
}

pub struct CartService {
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    next_id: Arc<Mutex<i32>>,
}

impl CartService {
    pub fn new() -> Self {
        CartService {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        // Find existing cart for user
        for (_, cart) in carts.iter() {
            if cart.user_id == user_id {
                return cart.clone();
            }
        }

        // Create new cart if none exists
        let mut next_id = self.next_id.lock().unwrap();
        let cart = Cart {
            id: *next_id,
            user_id,
            items: Vec::new(),
        };

        *next_id += 1;
        carts.insert(cart.id, cart.clone());
        cart
    }

    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        // Find or create cart
        let cart_id = self.get_or_create_cart(user_id).id;
        let cart = carts.get_mut(&cart_id).unwrap();

        // Check if product already in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            item.quantity += quantity;
        } else {
            // Add new item
            cart.items.push(CartItem {
                product_id: product.id,
                quantity,
                product_name: product.name.clone(),
                unit_price: product.price,
            });
        }

        cart.clone()
    }

    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        // Find cart for user
        for (_, cart) in carts.iter_mut() {
            if cart.user_id == user_id {
                cart.items.retain(|item| item.product_id != product_id);
                return Some(cart.clone());
            }
        }

        None
    }

    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();

        for (_, cart) in carts.iter() {
            if cart.user_id == user_id {
                return Some(cart.clone());
            }
        }

        None
    }

    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        for (_, cart) in carts.iter_mut() {
            if cart.user_id == user_id {
                cart.items.clear();
                return Some(cart.clone());
            }
        }

        None
    }
}
```

**Service Features:**
- Thread-safe storage using HashMap with Arc<Mutex>
- User-specific cart management (one cart per user)
- CartItem stores product snapshot (name, price) to avoid stale data
- Automatic cart creation on first access
- Quantity accumulation for duplicate products
- Returns cloned carts to avoid holding locks

### 3. `src/api/cart_routes.rs`
Cart API endpoints with JWT authentication:

```rust
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::cart::CartService;
use crate::catalog::ProductService;
use crate::auth::jwt::validate_token;

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

pub fn configure_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("", web::get().to(get_cart))
            .route("/add", web::post().to(add_item))
            .route("/remove/{product_id}", web::delete().to(remove_item))
            .route("/clear", web::post().to(clear_cart))
    );
}

async fn get_cart(
    cart_service: web::Data<CartService>,
    req: web::HttpRequest,
) -> impl Responder {
    // Extract user_id from JWT token in header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Skip "Bearer " prefix
                if let Ok(claims) = validate_token(token) {
                    let user_id = claims.sub.parse::<i32>().unwrap_or(0);
                    if let Some(cart) = cart_service.get_cart(user_id) {
                        return HttpResponse::Ok().json(cart);
                    }
                    return HttpResponse::Ok().json(cart_service.get_or_create_cart(user_id));
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

async fn add_item(
    cart_service: web::Data<CartService>,
    product_service: web::Data<ProductService>,
    req: web::HttpRequest,
    item: web::Json<AddItemRequest>,
) -> impl Responder {
    // Extract user_id from JWT token in header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Skip "Bearer " prefix
                if let Ok(claims) = validate_token(token) {
                    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

                    // Get product
                    if let Some(product) = product_service.get_by_id(item.product_id) {
                        // Check inventory
                        if product.inventory_count >= item.quantity {
                            let cart = cart_service.add_item(user_id, &product, item.quantity);
                            return HttpResponse::Ok().json(cart);
                        }
                        return HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Not enough inventory"
                        }));
                    }
                    return HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Product not found"
                    }));
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

async fn remove_item(
    cart_service: web::Data<CartService>,
    req: web::HttpRequest,
    path: web::Path<i32>,
) -> impl Responder {
    let product_id = path.into_inner();

    // Extract user_id from JWT token in header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Skip "Bearer " prefix
                if let Ok(claims) = validate_token(token) {
                    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

                    if let Some(cart) = cart_service.remove_item(user_id, product_id) {
                        return HttpResponse::Ok().json(cart);
                    }
                    return HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Item not found in cart"
                    }));
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

async fn clear_cart(
    cart_service: web::Data<CartService>,
    req: web::HttpRequest,
) -> impl Responder {
    // Extract user_id from JWT token in header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Skip "Bearer " prefix
                if let Ok(claims) = validate_token(token) {
                    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

                    if let Some(cart) = cart_service.clear_cart(user_id) {
                        return HttpResponse::Ok().json(cart);
                    }
                    return HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Cart not found"
                    }));
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}
```

**API Features:**
- JWT authentication required for all endpoints
- GET /api/cart - Retrieve user's cart
- POST /api/cart/add - Add product to cart with inventory validation
- DELETE /api/cart/remove/{product_id} - Remove item from cart
- POST /api/cart/clear - Clear all items from cart
- Proper error responses for auth failures, missing products, and insufficient inventory

### 4. `src/api/mod.rs`
Update to include cart routes module:

```rust
pub mod routes;
pub mod cart_routes;
```

### 5. `src/api/routes.rs`
Update to configure cart routes:

```rust
// Add to existing imports
use crate::api::cart_routes::configure_cart_routes;

// Update configure_routes function
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .configure(configure_cart_routes)  // Add this line
    );
}
```

## Implementation Steps

1. **Create Cart Service Module**
   - Create `src/cart/` directory if it doesn't exist
   - Create `src/cart/mod.rs` with service export
   - Ensure proper visibility with `pub use` statement

2. **Implement Cart Data Models and Service**
   - Create `src/cart/service.rs`
   - Define `CartItem` and `Cart` structs with appropriate derives
   - Implement `CartService` with HashMap-based storage
   - Implement all service methods: new, get_or_create_cart, add_item, remove_item, get_cart, clear_cart
   - Ensure thread-safety with Arc<Mutex<HashMap>>

3. **Create Cart API Routes**
   - Create `src/api/cart_routes.rs`
   - Define `AddItemRequest` DTO
   - Implement `configure_cart_routes` function
   - Implement all handler functions with JWT validation
   - Add product validation and inventory checking
   - Return appropriate HTTP status codes

4. **Update API Module Configuration**
   - Modify `src/api/mod.rs` to declare cart_routes module
   - Modify `src/api/routes.rs` to import and configure cart routes
   - Ensure cart routes are under /api/cart path

5. **Validation**
   - Run `cargo check` to ensure no syntax errors
   - Verify all imports are correct (auth::jwt, catalog::ProductService)
   - Check that JWT validation is implemented correctly
   - Ensure thread-safety mechanisms are correct

## Technical Considerations

### Authentication Integration
- All cart operations require valid JWT token
- Token extracted from Authorization header (Bearer scheme)
- User ID parsed from JWT claims (sub field)
- 401 Unauthorized returned for invalid/missing tokens

### Product Integration
- ProductService used to validate products exist
- Inventory checked before adding to cart
- Product snapshot (name, price) stored in CartItem
- Prevents issues with product changes after adding to cart

### Data Model Design
- **CartItem**: Stores product snapshot to avoid stale references
- **Cart**: User-specific with list of items
- HashMap storage allows O(1) cart lookup by ID
- User-to-cart relationship maintained through iteration

### Thread Safety
- **Arc<Mutex<HashMap>>**: Allows shared ownership across handlers
- Mutex ensures only one thread modifies carts at a time
- Cloning carts after operations to release locks quickly
- Suitable for test project; production would use database

### Error Handling
- Returns appropriate HTTP status codes (200, 400, 401, 404)
- JSON error messages for bad requests
- Graceful handling of missing products and inventory issues

## Integration Points

### With Task 3 (User Authentication)
- Uses `validate_token()` function from auth::jwt module
- Extracts user_id from JWT claims
- Relies on JWT token format and validation logic

### With Task 4 (Product Catalog)
- Uses `ProductService::get_by_id()` to validate products
- Checks `product.inventory_count` for availability
- Stores product snapshot (name, price) in cart items

### Used By Task 7 (Integration Tests)
- Full user flow tests will:
  - Create products
  - Generate JWT tokens
  - Add products to cart
  - Verify cart contents

## Risks and Mitigation

**Risk**: Dependency on Tasks 3 and 4 completion
- **Mitigation**: This is a Level 1 task that should wait for Level 0 tasks to complete. The CTO platform manages this dependency.

**Risk**: JWT validation might fail if Task 3 implementation changes
- **Mitigation**: Using standard JWT validation pattern. Task 3 provides stable interface.

**Risk**: Product service integration might have issues
- **Mitigation**: Using simple get_by_id method that's straightforward to implement in Task 4.

**Risk**: Race conditions with concurrent cart modifications
- **Mitigation**: Using Mutex ensures atomic operations. In-memory design is simple for test purposes.

## Success Criteria

1. ✅ `src/cart/mod.rs` exists and exports CartService
2. ✅ `src/cart/service.rs` exists with complete implementation
3. ✅ CartItem and Cart structs properly defined
4. ✅ CartService implements all required methods
5. ✅ Thread-safe storage with Arc<Mutex<HashMap>>
6. ✅ `src/api/cart_routes.rs` exists with all four endpoints
7. ✅ JWT authentication implemented for all endpoints
8. ✅ Product validation and inventory checking implemented
9. ✅ `src/api/mod.rs` updated to include cart_routes
10. ✅ `src/api/routes.rs` updated to configure cart routes
11. ✅ Code passes `cargo check` without errors
12. ✅ Proper error responses for auth, validation, and inventory failures
13. ✅ Integration with Tasks 3 and 4 is correct

## Estimated Effort
**45 minutes** - Service implementation, API routes with authentication, integration with product catalog, and error handling
