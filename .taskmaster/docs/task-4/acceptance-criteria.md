# Acceptance Criteria: Product Catalog Module

## Required Files

### ✅ `src/catalog/mod.rs`
- [ ] File exists at `src/catalog/mod.rs`
- [ ] Contains `pub mod models;` declaration
- [ ] Contains `pub mod service;` declaration
- [ ] Contains `pub use self::models::Product;` export
- [ ] Contains `pub use self::service::ProductService;` export
- [ ] File is properly formatted

### ✅ `src/catalog/models.rs`
- [ ] File exists at `src/catalog/models.rs`
- [ ] Contains `Product` struct with fields: id (i32), name (String), description (String), price (Decimal), inventory_count (i32)
- [ ] `Product` derives Debug, Serialize, Deserialize, and Clone
- [ ] Contains `NewProduct` struct with fields: name, description, price, inventory_count (no id)
- [ ] `NewProduct` derives Debug, Serialize, and Deserialize
- [ ] Contains `ProductFilter` struct with optional fields: name_contains, min_price, max_price, in_stock
- [ ] `ProductFilter` derives Debug, Serialize, and Deserialize
- [ ] All price fields use `rust_decimal::Decimal` type
- [ ] Proper use of `Option<T>` for optional filter fields
- [ ] All necessary imports included (serde, rust_decimal)

### ✅ `src/catalog/service.rs`
- [ ] File exists at `src/catalog/service.rs`
- [ ] Contains `ProductService` struct with fields: products (Arc<Mutex<Vec<Product>>>), next_id (Arc<Mutex<i32>>)
- [ ] Implements `new()` constructor that initializes empty storage with next_id = 1
- [ ] Implements `create(&self, new_product: NewProduct) -> Product` method
- [ ] `create()` generates auto-incrementing IDs correctly
- [ ] `create()` stores products in the vector
- [ ] Implements `get_all(&self) -> Vec<Product>` method
- [ ] Implements `get_by_id(&self, id: i32) -> Option<Product>` method
- [ ] Implements `update_inventory(&self, id: i32, new_count: i32) -> Option<Product>` method
- [ ] Implements `filter(&self, filter: ProductFilter) -> Vec<Product>` method
- [ ] Filter supports case-insensitive name matching
- [ ] Filter supports min_price comparison (>=)
- [ ] Filter supports max_price comparison (<=)
- [ ] Filter supports in_stock boolean check
- [ ] All filter criteria applied with AND logic
- [ ] Proper mutex locking and unlocking in all methods
- [ ] Returns cloned products to avoid holding locks
- [ ] All necessary imports included (Arc, Mutex, models)

### ✅ `Cargo.toml` Updates
- [ ] File contains `rust_decimal = { version = "1.30", features = ["serde"] }` dependency
- [ ] Dependency is in the `[dependencies]` section
- [ ] Version is 1.30 or higher
- [ ] Includes `serde` feature for serialization support

## Functional Requirements

### Product Creation
- [ ] ProductService can create products with all required fields
- [ ] Product IDs are auto-generated starting from 1
- [ ] IDs increment correctly for each new product
- [ ] Created products are stored in internal vector
- [ ] Create method returns the newly created product with ID

### Product Retrieval
- [ ] `get_all()` returns all stored products
- [ ] `get_all()` returns empty vector when no products exist
- [ ] `get_by_id()` returns Some(Product) when product exists
- [ ] `get_by_id()` returns None when product doesn't exist
- [ ] Retrieved products have correct data

### Inventory Management
- [ ] `update_inventory()` correctly updates inventory count
- [ ] `update_inventory()` returns updated product when successful
- [ ] `update_inventory()` returns None when product not found
- [ ] Inventory count can be increased and decreased
- [ ] Inventory updates persist in storage

### Product Filtering
- [ ] Filter by name_contains works (case-insensitive)
- [ ] Filter by min_price works (inclusive)
- [ ] Filter by max_price works (inclusive)
- [ ] Filter by in_stock works (true = inventory > 0, false = inventory == 0)
- [ ] Multiple filters work together (AND logic)
- [ ] Filter returns empty vector when no matches
- [ ] Filter with all None values returns all products

## Validation Tests

### Compilation Check
```bash
cargo check
```
- [ ] Command executes without errors
- [ ] No compilation warnings
- [ ] All dependencies resolve correctly
- [ ] rust_decimal types work correctly

### File Structure Check
```bash
ls -la src/catalog/
cat src/catalog/mod.rs
cat src/catalog/models.rs
cat src/catalog/service.rs
grep "rust_decimal" Cargo.toml
```
- [ ] All required files exist
- [ ] Files are in correct locations
- [ ] File contents match requirements
- [ ] Dependency is properly specified

### Module Import Check
- [ ] Module can be imported: `use crate::catalog::{Product, ProductService};`
- [ ] Structs are accessible from other modules
- [ ] Service methods are callable
- [ ] No visibility issues with pub/private

## Non-Functional Requirements

### Code Quality
- [ ] Code follows Rust naming conventions (snake_case for functions, PascalCase for types)
- [ ] Proper use of Rust idioms (Option, Result where appropriate)
- [ ] No unnecessary complexity
- [ ] Code is well-organized and readable
- [ ] Appropriate use of references vs owned values

### Thread Safety
- [ ] ProductService can be safely shared across threads
- [ ] Arc<Mutex> pattern used correctly
- [ ] No data races possible
- [ ] Mutex locks are acquired and released properly
- [ ] No deadlock potential

### Type Safety
- [ ] rust_decimal::Decimal used for all monetary values
- [ ] No floating-point types (f32/f64) for prices
- [ ] Strong typing for IDs (i32)
- [ ] Proper use of Option<T> for nullable values

### Documentation
- [ ] Code is self-documenting with clear names
- [ ] Struct fields are obvious in purpose
- [ ] Method signatures clearly indicate behavior

## Integration Readiness

- [ ] Module is ready for use by Task 5 (Shopping Cart API)
- [ ] ProductService can be instantiated and shared as web::Data
- [ ] Methods are compatible with async handlers
- [ ] Service can validate products exist before cart operations
- [ ] Inventory checks can be performed before adding to cart
- [ ] No blocking issues for dependent tasks

## Edge Cases and Error Handling

- [ ] get_by_id handles non-existent IDs gracefully (returns None)
- [ ] update_inventory handles non-existent IDs gracefully (returns None)
- [ ] Filter handles empty product list correctly
- [ ] Filter handles all-None criteria correctly
- [ ] Service handles concurrent access safely
- [ ] ID counter doesn't overflow (acceptable for test project)

## Performance Considerations

- [ ] Mutex locks released quickly (products cloned before returning)
- [ ] No long-held locks that could block other threads
- [ ] Filter operation is efficient for small datasets
- [ ] Acceptable performance for in-memory test implementation

## Success Metrics

- **Completion**: All required files created with correct content
- **Quality**: Code passes `cargo check` without errors or warnings
- **Functionality**: All CRUD operations work correctly
- **Thread Safety**: Service can be safely used from multiple threads
- **Integration**: Module is ready for Task 5 to depend on
- **Simplicity**: Implementation is straightforward and maintainable

## Manual Verification Checklist

1. **File Existence**
   - [ ] Check `src/catalog/mod.rs` exists
   - [ ] Check `src/catalog/models.rs` exists
   - [ ] Check `src/catalog/service.rs` exists

2. **Model Definitions**
   - [ ] Verify Product has all required fields
   - [ ] Verify NewProduct has all required fields except id
   - [ ] Verify ProductFilter has all optional fields
   - [ ] Confirm all price fields use Decimal type

3. **Service Implementation**
   - [ ] Verify ProductService has correct fields
   - [ ] Verify all 6 methods exist: new, create, get_all, get_by_id, update_inventory, filter
   - [ ] Check Arc<Mutex> pattern is used correctly
   - [ ] Verify filter logic implements all criteria

4. **Dependencies**
   - [ ] Confirm rust_decimal in Cargo.toml
   - [ ] Verify serde feature is enabled
   - [ ] Check version is 1.30 or higher

5. **Compilation**
   - [ ] Run `cargo check` and verify success
   - [ ] Ensure no warnings or errors

6. **Integration Preparation**
   - [ ] Verify module can be imported by other code
   - [ ] Confirm ProductService can be instantiated
   - [ ] Check that methods have correct signatures for Task 5 integration

## Automated Testing

While this task doesn't require unit tests, the following validation should pass:

```rust
// Example validation (not required to implement, just concept)
let service = ProductService::new();

// Test create
let new_product = NewProduct {
    name: "Test Product".to_string(),
    description: "Test Description".to_string(),
    price: Decimal::new(1999, 2), // $19.99
    inventory_count: 10,
};
let product = service.create(new_product);
assert_eq!(product.id, 1);
assert_eq!(product.name, "Test Product");

// Test get_by_id
let found = service.get_by_id(1);
assert!(found.is_some());

// Test get_all
let all = service.get_all();
assert_eq!(all.len(), 1);

// Test update_inventory
let updated = service.update_inventory(1, 5);
assert!(updated.is_some());
assert_eq!(updated.unwrap().inventory_count, 5);

// Test filter
let filter = ProductFilter {
    name_contains: Some("Test".to_string()),
    min_price: None,
    max_price: None,
    in_stock: Some(true),
};
let filtered = service.filter(filter);
assert_eq!(filtered.len(), 1);
```

## Definition of Done

This task is complete when:
1. All required files exist with correct implementations
2. All structs and methods match specifications exactly
3. rust_decimal is used for all price fields
4. Thread-safe storage with Arc<Mutex> is implemented
5. All service methods work correctly
6. Filter supports all query types
7. Cargo.toml has rust_decimal dependency
8. Code passes `cargo check` without errors
9. Module is ready for Task 5 integration
10. All acceptance criteria checkboxes can be marked complete
