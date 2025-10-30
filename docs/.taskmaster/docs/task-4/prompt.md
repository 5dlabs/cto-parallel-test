# Task 4: Product Catalog Module - Agent Prompt

You are a Rust backend developer tasked with implementing a product catalog and inventory management system for a test e-commerce API.

## Your Mission
Create a complete product catalog module with models, in-memory service, and advanced filtering capabilities. This provides the product management foundation that the shopping cart will use to validate items and check inventory.

## What You Must Create

### 1. Update `Cargo.toml`
Add this dependency to the `[dependencies]` section:
```toml
rust_decimal = { version = "1.30", features = ["serde"] }
```

### 2. Create `src/catalog/mod.rs`
Module exports and re-exports:
```rust
pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
```

### 3. Create `src/catalog/models.rs`
Define three model types with serde support:

**Product struct**:
- `id: i32`
- `name: String`
- `description: String`
- `price: Decimal` (from rust_decimal)
- `inventory_count: i32`
- Derive `Debug, Serialize, Deserialize, Clone`

**NewProduct struct** (for creation, no ID):
- `name: String`
- `description: String`
- `price: Decimal`
- `inventory_count: i32`
- Derive `Debug, Serialize, Deserialize`

**ProductFilter struct** (all optional):
- `name_contains: Option<String>`
- `min_price: Option<Decimal>`
- `max_price: Option<Decimal>`
- `in_stock: Option<bool>`
- Derive `Debug, Serialize, Deserialize`

### 4. Create `src/catalog/service.rs`
Implement thread-safe in-memory product service:

**ProductService struct**:
```rust
use std::sync::{Arc, Mutex};

pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}
```

**Required methods**:

**`new() -> Self`**
- Initialize empty products vector wrapped in Arc<Mutex<>>
- Initialize next_id to 1 wrapped in Arc<Mutex<>>

**`create(&self, new_product: NewProduct) -> Product`**
- Lock products and next_id
- Assign ID from next_id
- Increment next_id
- Create Product from NewProduct with assigned ID
- Push to products vector
- Clone and return created product

**`get_all(&self) -> Vec<Product>`**
- Lock products
- Clone and return entire vector

**`get_by_id(&self, id: i32) -> Option<Product>`**
- Lock products
- Find product by ID with `iter().find(|p| p.id == id)`
- Clone and return if found

**`update_inventory(&self, id: i32, new_count: i32) -> Option<Product>`**
- Lock products mutably
- Find product with `iter_mut().find(|p| p.id == id)`
- Update inventory_count
- Clone and return updated product

**`filter(&self, filter: ProductFilter) -> Vec<Product>`**
- Lock products
- Filter by all criteria (ALL must match):
  - `name_contains`: Case-insensitive substring match
  - `min_price`: price >= min_price
  - `max_price`: price <= max_price
  - `in_stock`: (inventory_count > 0) == in_stock
- Use `map_or(true, ...)` for None values (no filter)
- Combine with AND logic
- Clone and return filtered products

## Key Requirements

✅ **Thread Safety**:
- Use `Arc<Mutex<>>` for shared mutable state
- Separate locks for products and next_id
- Service must be Send + Sync

✅ **Decimal Precision**:
- Use `rust_decimal::Decimal` for prices (not f32/f64)
- Ensures exact decimal arithmetic for money

✅ **Filtering Logic**:
- All filters are optional (None means no filter)
- Name search is case-insensitive
- All filters combine with AND
- Empty filter returns all products

✅ **Data Ownership**:
- Return cloned products (not references)
- Safe to use after lock is released
- Methods don't hold locks longer than necessary

## Constraints
- This is a **Level 0** task with no dependencies on other tasks
- Use in-memory storage (Vec, not database)
- Keep implementations straightforward - this is a test project
- ID counter starts at 1 and increments

## Validation
After completing the work:
1. Verify all files exist at specified paths
2. Ensure `cargo check` passes
3. Test product creation assigns sequential IDs
4. Test inventory updates work correctly
5. Test filtering with various combinations
6. Verify thread safety (service can be shared)

## Success Definition
Task is complete when:
- All three catalog files exist with correct implementations
- Products can be created with auto-incrementing IDs
- All CRUD operations work correctly
- Filtering combines multiple criteria properly
- Inventory updates modify existing products
- Code compiles without errors
- Service is thread-safe (Send + Sync)

## Context
You're working on a parallel task execution test.

**Your independence**:
- No dependencies - you can start immediately

**Tasks depending on you**:
- Task 5: Shopping Cart API (needs product lookup and inventory check)
- Task 7: Integration Tests (will test product operations)

**Running in parallel (Level 0)**:
- Task 1: Database Schema
- Task 3: User Authentication
- Task 6: Frontend Components

---

**Start working now. Create the files, write the service code, and verify catalog operations work correctly.**
