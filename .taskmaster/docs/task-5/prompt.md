# Autonomous Agent Prompt: Shopping Cart API

## Mission
You are tasked with creating shopping cart functionality and API endpoints for a Rust API project. This module integrates with the authentication system (Task 3) for user verification and the product catalog (Task 4) for product validation and inventory management.

## Prerequisites

**IMPORTANT**: This is a Level 1 task with dependencies on:
- **Task 3 (User Authentication Module)** - Must be complete for JWT validation
- **Task 4 (Product Catalog Module)** - Must be complete for product operations

Before starting, verify these modules exist:
- `src/auth/jwt.rs` with `validate_token()` function
- `src/catalog/service.rs` with `ProductService`

If these are not available, WAIT for Tasks 3 and 4 to complete.

## What You Need to Do

### 1. Create Cart Module Structure (`src/cart/mod.rs`)
Create a new file at `src/cart/mod.rs` to export cart components:

```rust
pub mod service;

pub use self::service::CartService;
```

### 2. Implement Cart Service (`src/cart/service.rs`)
Create `src/cart/service.rs` with cart data models and service:

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

**Service Methods Explained:**
- `new()`: Creates empty cart storage
- `get_or_create_cart()`: Gets existing cart for user or creates new one
- `add_item()`: Adds product to cart, accumulating quantity if already present
- `remove_item()`: Removes specific product from cart
- `get_cart()`: Retrieves user's cart if it exists
- `clear_cart()`: Removes all items from user's cart

**Why CartItem stores product snapshot:**
- Stores product_name and unit_price at time of adding
- Prevents issues if product is deleted or price changes
- Cart shows what user added, not current product state

### 3. Create Cart API Routes (`src/api/cart_routes.rs`)
Create `src/api/cart_routes.rs` with authenticated endpoints:

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

**API Endpoints:**
- `GET /api/cart` - Get user's cart (creates if doesn't exist)
- `POST /api/cart/add` - Add product to cart (with inventory check)
- `DELETE /api/cart/remove/{product_id}` - Remove product from cart
- `POST /api/cart/clear` - Clear all items from cart

**Authentication Flow:**
1. Extract Authorization header
2. Check for "Bearer " prefix
3. Extract token (skip first 7 characters)
4. Validate token using Task 3's validate_token()
5. Parse user_id from JWT claims
6. Return 401 Unauthorized if any step fails

### 4. Update API Module (`src/api/mod.rs`)
Add cart_routes module declaration:

```rust
pub mod routes;
pub mod cart_routes;
```

### 5. Update API Routes Configuration (`src/api/routes.rs`)
Import and configure cart routes in the existing configure_routes function:

```rust
// Add to imports at top of file
use crate::api::cart_routes::configure_cart_routes;

// In the configure_routes function, add cart routes:
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            // ... other routes ...
            .configure(configure_cart_routes)  // Add this line
    );
}
```

## Expected Behavior

After implementation:
- All cart operations require valid JWT token
- Users can add products to their cart with inventory validation
- Duplicate products accumulate quantity instead of creating separate items
- Each user has their own isolated cart
- Cart persists product snapshot (name, price) to avoid stale data
- Returns proper HTTP status codes (200, 400, 401, 404)
- JSON responses for cart data and errors

## Validation Steps

Before marking this task complete, verify:

1. **Dependency Check**
   ```bash
   grep -r "validate_token" src/auth/jwt.rs
   grep -r "ProductService" src/catalog/service.rs
   ```
   Ensure dependencies from Tasks 3 and 4 exist.

2. **File Structure**
   ```bash
   ls -la src/cart/mod.rs
   ls -la src/cart/service.rs
   ls -la src/api/cart_routes.rs
   cat src/api/mod.rs | grep cart_routes
   ```

3. **Syntax Check**
   ```bash
   cargo check
   ```
   Should complete without errors.

4. **Module Integration**
   Verify imports work correctly for auth::jwt and catalog modules.

## Constraints

- This is a test project - use in-memory storage
- Wait for Tasks 3 and 4 to complete before starting
- Use JWT authentication from Task 3
- Use ProductService from Task 4
- Follow exact struct definitions and method signatures
- Do not add extra features beyond requirements
- Ensure thread safety with Arc<Mutex>

## Common Issues and Solutions

**Issue**: Cannot find `validate_token` function
- **Solution**: Ensure Task 3 is complete and exports this function from auth::jwt module

**Issue**: Cannot find `ProductService`
- **Solution**: Ensure Task 4 is complete and ProductService is exported from catalog module

**Issue**: JWT claims.sub is string but need i32
- **Solution**: Use `claims.sub.parse::<i32>().unwrap_or(0)` to convert

**Issue**: Mutex lock might panic
- **Solution**: Using unwrap() is acceptable for this test project

**Issue**: add_item locks twice (get_or_create_cart and then add)
- **Solution**: This is acceptable for test purposes; production would optimize

## Success Definition

Task is complete when:
- ✅ `src/cart/mod.rs` created with proper exports
- ✅ `src/cart/service.rs` created with CartItem, Cart, and CartService
- ✅ CartService uses Arc<Mutex<HashMap>> for thread-safe storage
- ✅ All five methods implemented: new, get_or_create_cart, add_item, remove_item, get_cart, clear_cart
- ✅ `src/api/cart_routes.rs` created with all four endpoints
- ✅ JWT authentication implemented for all endpoints
- ✅ Product validation and inventory checking in add_item
- ✅ `src/api/mod.rs` updated to declare cart_routes
- ✅ `src/api/routes.rs` updated to configure cart routes
- ✅ Proper error responses (401, 404, 400) implemented
- ✅ `cargo check` passes without errors

## Integration Notes

### With Task 3 (Authentication)
- Import: `use crate::auth::jwt::validate_token;`
- Call: `validate_token(token)` returns `Result<Claims, ...>`
- Claims struct has `sub` field with user ID as string

### With Task 4 (Product Catalog)
- Import: `use crate::catalog::ProductService;`
- Inject: `product_service: web::Data<ProductService>` in handlers
- Call: `product_service.get_by_id(id)` returns `Option<Product>`
- Check: `product.inventory_count >= quantity` before adding

## Next Steps

After completing this task:
1. Task 7 (Integration Tests) can begin (it depends on Tasks 2, 5, and 6)
2. The CartService will be integrated into the main application
3. Full e-commerce flow will be testable: login → browse products → add to cart
