# Task 4: Product Catalog Module

## Overview
Create product catalog and inventory management functionality for the Rust API project. This is a Level 0 task (no dependencies) that implements product models, service logic, and filtering capabilities for an e-commerce catalog.

## Context
This task provides the product management foundation for the application. It enables product creation, retrieval, inventory management, and filtering that will be consumed by Task 5 (Shopping Cart API) when users add items to their carts.

## Objectives
1. Define product data models with pricing and inventory
2. Implement in-memory product service with CRUD operations
3. Create advanced filtering capabilities (price range, stock status, name search)
4. Provide thread-safe service for concurrent access
5. Support inventory management operations

## Dependencies
**None** - This is a Level 0 task that can run independently in parallel with Tasks 1, 3, and 6.

## Files to Create
- `src/catalog/mod.rs` - Catalog module exports
- `src/catalog/models.rs` - Product data models
- `src/catalog/service.rs` - Product service with business logic
- `Cargo.toml` - Updates for decimal number handling

## Technical Specifications

### Data Types
- **Decimal Handling**: rust_decimal 1.30 with serde support for precise price calculations
- **Thread Safety**: Arc<Mutex<T>> for shared mutable state
- **Concurrency**: Service designed for multi-threaded web server access

### Product Model
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,           // Precise decimal for money
    pub inventory_count: i32,
}
```

### New Product DTO
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}
```

### Product Filter
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}
```

## Implementation Plan

### Step 1: Update Cargo.toml
Add decimal handling dependency:

```toml
[dependencies]
rust_decimal = { version = "1.30", features = ["serde"] }
```

**Why rust_decimal**: Floating-point arithmetic (f32/f64) is imprecise for financial calculations. Decimal provides exact decimal arithmetic suitable for prices.

### Step 2: Create Module Exports (src/catalog/mod.rs)
```rust
pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
```

Clean public interface exposing key types.

### Step 3: Implement Data Models (src/catalog/models.rs)
Define three model types:

#### Product
Full product with ID (represents stored product)

#### NewProduct
Product without ID (for creation requests)

#### ProductFilter
Optional filter criteria for querying products

**Design Note**: Separation of Product and NewProduct follows the DTO pattern - creation doesn't have ID, existing products do.

### Step 4: Implement Product Service (src/catalog/service.rs)
In-memory service with thread-safe operations:

#### Data Structure
```rust
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}
```

Uses `Arc<Mutex<>>` for:
- **Arc**: Atomic Reference Counting for sharing across threads
- **Mutex**: Mutual exclusion for safe concurrent modification
- Separate locks for products and ID to minimize contention

#### Core Methods

**`new() -> Self`**
- Initializes empty product list
- Sets next_id to 1
- Returns ready-to-use service instance

**`create(&self, new_product: NewProduct) -> Product`**
- Acquires lock on products and next_id
- Assigns ID from next_id counter
- Increments next_id
- Converts NewProduct to Product
- Stores in vector
- Returns created product with ID

**`get_all(&self) -> Vec<Product>`**
- Acquires read lock on products
- Clones entire vector
- Returns owned copy (safe to use after lock released)

**`get_by_id(&self, id: i32) -> Option<Product>`**
- Acquires read lock on products
- Searches for product by ID
- Returns cloned product if found, None otherwise

**`update_inventory(&self, id: i32, new_count: i32) -> Option<Product>`**
- Acquires write lock on products
- Finds product by ID (mutable reference)
- Updates inventory_count
- Returns cloned updated product

**`filter(&self, filter: ProductFilter) -> Vec<Product>`**
- Acquires read lock on products
- Applies all filter criteria:
  - `name_contains`: Case-insensitive substring search
  - `min_price`: Price >= minimum
  - `max_price`: Price <= maximum
  - `in_stock`: inventory_count > 0 matches true
- Combines all filters with AND logic
- Returns cloned filtered products

### Step 5: Filtering Logic Implementation
```rust
products.iter().filter(|p| {
    let name_match = filter.name_contains
        .as_ref()
        .map_or(true, |name| p.name.to_lowercase().contains(&name.to_lowercase()));

    let min_price_match = filter.min_price
        .as_ref()
        .map_or(true, |min| p.price >= *min);

    let max_price_match = filter.max_price
        .as_ref()
        .map_or(true, |max| p.price <= *max);

    let in_stock_match = filter.in_stock
        .map_or(true, |in_stock| (p.inventory_count > 0) == in_stock);

    name_match && min_price_match && max_price_match && in_stock_match
}).cloned().collect()
```

**Design Decisions**:
- `map_or(true, ...)`: If filter field is None, it matches everything
- Case-insensitive name search for better UX
- All filters are inclusive (>= and <=)
- Explicit clone to return owned data

## Architectural Considerations

### In-Memory Storage
**For this test project**: Products stored in memory (Vec<Product>)
**Production alternative**: Replace with database repository pattern, keeping same interface

### Thread Safety
Service is `Send + Sync`:
- Can be shared across Actix-web worker threads
- Mutex ensures safe concurrent access
- Short critical sections (lock, operate, unlock)

### Decimal Precision
Using `rust_decimal::Decimal` instead of `f64`:
- Exact decimal representation
- No floating-point rounding errors
- Essential for financial calculations
- Example: 0.1 + 0.2 = 0.3 exactly (not 0.30000000000000004)

### Service Pattern
- Encapsulates business logic
- Hides storage implementation
- Testable without database
- Can be mocked for testing

### Memory Efficiency
Current implementation clones products on read:
- **Pro**: No lock held while consumer uses data
- **Con**: Memory overhead for large catalogs
- **Alternative**: Return references with lifetime (requires lifetime management)

## Risks and Considerations

1. **Memory Storage**: Data lost on server restart. Acceptable for test project, unacceptable for production.

2. **Linear Search**: `get_by_id` and `filter` are O(n). For large catalogs, use HashMap for ID lookups and indexed search.

3. **Lock Contention**: Single lock on products vector. High-traffic scenarios might benefit from read-write lock (RwLock) or sharding.

4. **Clone Overhead**: Every read clones products. For large product structures, consider Arc-wrapping individual products.

5. **No Pagination**: `get_all` returns entire catalog. Production would need pagination with limit/offset.

6. **ID Generation**: Simple counter. Production would use database sequences or UUIDs.

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All catalog files created
- Product CRUD operations work correctly
- Filtering combines multiple criteria correctly
- Inventory management updates products
- Thread-safe service compiles
- Compatible with Task 5's requirements

## Related Tasks
- **Task 5**: Shopping Cart API (depends on this task for product validation)
- **Task 7**: Integration Tests (will test product operations)
- **Independent of**: Tasks 1, 3, 6 (runs in parallel)

## Diagram
See `diagrams.mmd` for visual representation of the service architecture and data models.

## Production Improvements (Not in Scope)
- Database persistence layer
- Pagination for large catalogs
- Full-text search for product names
- Category/tag system
- Product images and media
- Price history and analytics
- Soft delete for discontinued products
- Audit log for inventory changes
- Bulk import/export capabilities

## References
- [rust_decimal Documentation](https://docs.rs/rust_decimal/)
- [Arc Documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Mutex Documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- Project PRD: `.taskmaster/docs/prd.txt`
