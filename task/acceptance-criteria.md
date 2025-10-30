# Task 4: Product Catalog Module - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `src/catalog/mod.rs` exists
- [ ] `src/catalog/models.rs` exists
- [ ] `src/catalog/service.rs` exists
- [ ] `Cargo.toml` has been updated with rust_decimal dependency

## Code Quality Criteria

### ✅ Module Exports (src/catalog/mod.rs)
- [ ] Contains `pub mod models;`
- [ ] Contains `pub mod service;`
- [ ] Re-exports Product: `pub use self::models::Product;`
- [ ] Re-exports ProductService: `pub use self::service::ProductService;`
- [ ] Valid Rust module syntax

### ✅ Data Models (src/catalog/models.rs)
- [ ] Imports rust_decimal: `use rust_decimal::Decimal;`
- [ ] Imports serde: `use serde::{Serialize, Deserialize};`
- [ ] Product struct defined with:
  - `id: i32` field
  - `name: String` field
  - `description: String` field
  - `price: Decimal` field
  - `inventory_count: i32` field
  - Derives: `Debug, Serialize, Deserialize` (Clone recommended)
- [ ] NewProduct struct defined with:
  - `name: String` field
  - `description: String` field
  - `price: Decimal` field
  - `inventory_count: i32` field
  - Derives: `Debug, Serialize, Deserialize`
- [ ] ProductFilter struct defined with:
  - `name_contains: Option<String>` field
  - `min_price: Option<Decimal>` field
  - `max_price: Option<Decimal>` field
  - `in_stock: Option<bool>` field
  - Derives: `Debug, Serialize, Deserialize`

### ✅ Product Service (src/catalog/service.rs)
- [ ] Imports models: `use crate::catalog::models::{Product, NewProduct, ProductFilter};`
- [ ] Imports rust_decimal: `use rust_decimal::Decimal;`
- [ ] Imports sync types: `use std::sync::{Arc, Mutex};`
- [ ] ProductService struct defined with:
  - `products: Arc<Mutex<Vec<Product>>>` field
  - `next_id: Arc<Mutex<i32>>` field
- [ ] ProductService implements new() method:
  - Returns Self
  - Initializes products with Arc::new(Mutex::new(Vec::new()))
  - Initializes next_id with Arc::new(Mutex::new(1))
- [ ] ProductService implements create() method:
  - Signature: `pub fn create(&self, new_product: NewProduct) -> Product`
  - Locks both products and next_id
  - Assigns ID from next_id
  - Increments next_id
  - Creates Product with all fields from NewProduct plus ID
  - Pushes to products vector
  - Returns cloned product
- [ ] ProductService implements get_all() method:
  - Signature: `pub fn get_all(&self) -> Vec<Product>`
  - Locks products
  - Returns cloned vector
- [ ] ProductService implements get_by_id() method:
  - Signature: `pub fn get_by_id(&self, id: i32) -> Option<Product>`
  - Locks products
  - Finds product by ID
  - Returns cloned product or None
- [ ] ProductService implements update_inventory() method:
  - Signature: `pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product>`
  - Locks products mutably
  - Finds product by ID
  - Updates inventory_count
  - Returns cloned updated product or None
- [ ] ProductService implements filter() method:
  - Signature: `pub fn filter(&self, filter: ProductFilter) -> Vec<Product>`
  - Locks products
  - Filters by name_contains (case-insensitive substring)
  - Filters by min_price (>= comparison)
  - Filters by max_price (<= comparison)
  - Filters by in_stock (inventory_count > 0)
  - Treats None as no filter (matches all)
  - Combines all filters with AND logic
  - Returns cloned filtered products

### ✅ Dependencies (Cargo.toml)
- [ ] Includes `rust_decimal = { version = "1.30", features = ["serde"] }`
- [ ] Dependency is in the `[dependencies]` section
- [ ] TOML syntax is valid

## Functional Criteria

### ✅ Product Creation
- [ ] Creating product assigns ID starting from 1
- [ ] Each created product gets unique sequential ID
- [ ] Created product contains all fields from NewProduct
- [ ] Created product includes assigned ID

### ✅ Product Retrieval
- [ ] get_all() returns empty vector initially
- [ ] get_all() returns all created products
- [ ] get_by_id() returns None for non-existent ID
- [ ] get_by_id() returns correct product for existing ID

### ✅ Inventory Management
- [ ] update_inventory() returns None for non-existent product
- [ ] update_inventory() updates and returns product for existing ID
- [ ] update_inventory() correctly modifies inventory_count
- [ ] Subsequent get_by_id() reflects updated inventory

### ✅ Filtering
- [ ] Empty filter (all None) returns all products
- [ ] name_contains filter matches substring case-insensitively
- [ ] min_price filter includes products with price >= min_price
- [ ] max_price filter includes products with price <= max_price
- [ ] in_stock=true filters products with inventory_count > 0
- [ ] in_stock=false filters products with inventory_count == 0
- [ ] Multiple filters combine with AND logic
- [ ] None values in filter don't restrict results

## Thread Safety Criteria

### ✅ Concurrency
- [ ] ProductService is Send (can be sent across threads)
- [ ] ProductService is Sync (can be shared across threads)
- [ ] Multiple threads can safely call methods concurrently
- [ ] No data races possible (ensured by Mutex)

## Compilation and Testing Criteria

### ✅ Build Verification
- [ ] `cargo check` completes without errors
- [ ] `cargo build` completes successfully
- [ ] No warnings related to unused imports or dead code
- [ ] rust_decimal dependency resolves correctly

### ✅ Unit Test Validation
Implement and run these tests:

**Creation Test**:
```rust
#[test]
fn test_product_creation() {
    let service = ProductService::new();
    let product1 = service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "Description".to_string(),
        price: Decimal::new(1999, 2), // $19.99
        inventory_count: 10,
    });
    assert_eq!(product1.id, 1);
    assert_eq!(product1.name, "Test Product");

    let product2 = service.create(NewProduct {
        name: "Product 2".to_string(),
        description: "Desc 2".to_string(),
        price: Decimal::new(2999, 2), // $29.99
        inventory_count: 5,
    });
    assert_eq!(product2.id, 2);
}
```

**Retrieval Test**:
```rust
#[test]
fn test_product_retrieval() {
    let service = ProductService::new();
    let created = service.create(/* ... */);

    let found = service.get_by_id(created.id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, created.id);

    let not_found = service.get_by_id(9999);
    assert!(not_found.is_none());

    let all = service.get_all();
    assert_eq!(all.len(), 1);
}
```

**Inventory Test**:
```rust
#[test]
fn test_inventory_update() {
    let service = ProductService::new();
    let product = service.create(/* inventory_count: 10 */);

    let updated = service.update_inventory(product.id, 5);
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().inventory_count, 5);

    let retrieved = service.get_by_id(product.id).unwrap();
    assert_eq!(retrieved.inventory_count, 5);
}
```

**Filtering Test**:
```rust
#[test]
fn test_product_filtering() {
    let service = ProductService::new();

    // Create test products
    service.create(/* name: "Apple", price: 1.50, inventory: 10 */);
    service.create(/* name: "Banana", price: 0.75, inventory: 0 */);
    service.create(/* name: "Orange", price: 2.00, inventory: 5 */);

    // Test name filter
    let filtered = service.filter(ProductFilter {
        name_contains: Some("app".to_string()),
        ..Default::default()
    });
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Apple");

    // Test price range filter
    let filtered = service.filter(ProductFilter {
        min_price: Some(Decimal::new(100, 2)), // $1.00
        max_price: Some(Decimal::new(180, 2)), // $1.80
        ..Default::default()
    });
    assert_eq!(filtered.len(), 1);

    // Test in_stock filter
    let filtered = service.filter(ProductFilter {
        in_stock: Some(true),
        ..Default::default()
    });
    assert_eq!(filtered.len(), 2); // Apple and Orange

    // Test combined filters
    let filtered = service.filter(ProductFilter {
        name_contains: Some("a".to_string()),
        in_stock: Some(true),
        ..Default::default()
    });
    assert_eq!(filtered.len(), 2); // Apple and Orange contain "a"
}
```

- [ ] All unit tests pass
- [ ] Creation test verifies sequential ID assignment
- [ ] Retrieval test verifies get_all and get_by_id
- [ ] Inventory test verifies updates persist
- [ ] Filtering test verifies all filter combinations

## Integration Criteria

### ✅ Compatibility with Dependent Tasks
- [ ] Task 5 can import and use ProductService
- [ ] Task 5 can import and use Product model
- [ ] Task 7 can test product operations
- [ ] Module exports are accessible from other modules

## Testing Commands

### Manual Validation Steps

1. **Verify File Existence**
   ```bash
   ls -la src/catalog/mod.rs
   ls -la src/catalog/models.rs
   ls -la src/catalog/service.rs
   ```

2. **Check Rust Compilation**
   ```bash
   cargo check
   cargo build
   ```

3. **Validate Dependencies**
   ```bash
   cargo tree | grep rust_decimal
   ```

4. **Run Unit Tests**
   ```bash
   cargo test catalog
   ```

5. **Test Service Functionality (via rust code)**
   ```rust
   use crate::catalog::{ProductService, Product};
   use rust_decimal::Decimal;

   let service = ProductService::new();

   // Test creation
   let product = service.create(NewProduct {
       name: "Test".to_string(),
       description: "Test product".to_string(),
       price: Decimal::new(1999, 2),
       inventory_count: 10,
   });
   println!("Created: {:?}", product);

   // Test retrieval
   let found = service.get_by_id(product.id);
   println!("Found: {:?}", found);

   // Test inventory
   let updated = service.update_inventory(product.id, 5);
   println!("Updated: {:?}", updated);

   // Test filtering
   let filtered = service.filter(ProductFilter {
       in_stock: Some(true),
       ..Default::default()
   });
   println!("Filtered: {:?}", filtered);
   ```

## Success Definition

**Task is COMPLETE when:**
1. All required files exist with correct implementations
2. Product CRUD operations work correctly
3. Filtering combines multiple criteria with AND logic
4. Inventory updates persist correctly
5. Service is thread-safe (Send + Sync)
6. All unit tests pass
7. Code compiles without errors
8. rust_decimal dependency resolves

**Task is INCOMPLETE if:**
- Any required file is missing
- Compilation errors exist
- CRUD operations don't work as specified
- Filtering logic is incorrect
- Inventory updates don't persist
- Service isn't thread-safe
- Unit tests fail

## Estimated Completion Time
40 minutes (as specified in PRD)

## Dependencies
None - This is a Level 0 task

## Blocks
- Task 5: Shopping Cart API (needs product lookup)
- Task 7: Integration Tests (needs product operations)

## Performance Considerations
- [ ] Service creates are O(1) (append to vector)
- [ ] Service retrievals are O(n) (linear search acceptable for test)
- [ ] Filtering is O(n) (full scan acceptable for test)
- [ ] Locks are held for minimal time
- [ ] Cloning is used instead of holding locks
