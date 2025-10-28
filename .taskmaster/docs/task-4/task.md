# Task 4: Product Catalog Module

## Overview
Create product catalog and inventory management functionality using Rust. This is a foundational Level 0 task that has no dependencies and should execute in parallel with other Level 0 tasks (Tasks 1, 3, and 6).

## Context
This task is part of the parallel task execution test project. It establishes the product catalog foundation that Task 5 (Shopping Cart API) will depend on. The implementation uses in-memory storage with Arc<Mutex<Vec>> for thread-safe product management and rust_decimal for precise price handling.

## Objectives
1. Create product catalog module structure in `src/catalog/mod.rs`
2. Define product models with rust_decimal for prices in `src/catalog/models.rs`
3. Implement ProductService with inventory management in `src/catalog/service.rs`
4. Add rust_decimal dependency to `Cargo.toml`

## Dependencies
**None** - This is a Level 0 task that can run independently.

**Depended Upon By:**
- **Task 5 (Shopping Cart API)** - Level 1 - Uses ProductService to validate products and check inventory

## Files to Create/Modify

### 1. `src/catalog/mod.rs`
Module declaration file that exports catalog components:

```rust
pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
```

### 2. `src/catalog/models.rs`
Product data models with rust_decimal for precise price handling:

```rust
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}
```

**Key Features:**
- `Product`: Complete product with ID
- `NewProduct`: DTO for product creation
- `ProductFilter`: Query parameters for filtering
- Uses `rust_decimal::Decimal` for precise monetary values
- Implements `Serialize`/`Deserialize` for JSON API support
- `Clone` trait for in-memory operations

### 3. `src/catalog/service.rs`
ProductService with in-memory storage and inventory management:

```rust
use crate::catalog::models::{Product, NewProduct, ProductFilter};
use rust_decimal::Decimal;
use std::sync::{Arc, Mutex};

// In a real app, this would interact with the database
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    pub fn new() -> Self {
        ProductService {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let product = Product {
            id: *next_id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        *next_id += 1;
        products.push(product.clone());
        product
    }

    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().unwrap();
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    pub fn filter(&self, filter: ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products
            .iter()
            .filter(|p| {
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
            })
            .cloned()
            .collect()
    }
}
```

**Service Features:**
- Thread-safe in-memory storage using `Arc<Mutex<Vec<Product>>>`
- Auto-incrementing ID generation
- CRUD operations: create, get_all, get_by_id, update_inventory
- Advanced filtering by name, price range, and stock status
- Case-insensitive name search
- Returns cloned products to avoid holding locks

### 4. `Cargo.toml` Updates
Add rust_decimal dependency:

```toml
[dependencies]
rust_decimal = { version = "1.30", features = ["serde"] }
```

**Dependency Features:**
- `rust_decimal` for precise decimal arithmetic (critical for monetary values)
- `serde` feature for JSON serialization support

## Implementation Steps

1. **Create Module Structure**
   - Create `src/catalog/` directory if it doesn't exist
   - Create `src/catalog/mod.rs` with module exports
   - Ensure proper visibility with `pub use` statements

2. **Define Product Models**
   - Create `src/catalog/models.rs`
   - Define `Product`, `NewProduct`, and `ProductFilter` structs
   - Use `rust_decimal::Decimal` for the price field
   - Add appropriate derives: `Debug`, `Serialize`, `Deserialize`, `Clone`

3. **Implement ProductService**
   - Create `src/catalog/service.rs`
   - Implement `ProductService` struct with Arc<Mutex> storage
   - Implement `new()` constructor
   - Implement all service methods: create, get_all, get_by_id, update_inventory, filter
   - Ensure thread-safety with proper mutex locking

4. **Update Dependencies**
   - Modify `Cargo.toml` to include rust_decimal with serde feature
   - Ensure version compatibility (1.30+)

5. **Validation**
   - Run `cargo check` to ensure no syntax errors
   - Verify all structs compile correctly
   - Check that rust_decimal dependency resolves
   - Ensure thread-safety mechanisms are correct

## Technical Considerations

### Data Type Choice
- **rust_decimal**: Provides exact decimal arithmetic needed for monetary values
- Avoids floating-point precision issues (e.g., 0.1 + 0.2 != 0.3)
- Serializes cleanly to/from JSON

### Thread Safety
- **Arc<Mutex<Vec<Product>>>**: Allows shared ownership across threads
- Mutex ensures only one thread modifies data at a time
- Cloning products after read to release locks quickly
- Suitable for test project; production would use database

### Service Design Pattern
- Encapsulates all product operations in one service
- Stateful service with internal storage
- Can be easily converted to database-backed implementation
- Methods return owned data (Product) rather than references

### Filtering Implementation
- Flexible filtering with optional criteria
- All filters applied simultaneously (AND logic)
- Case-insensitive name matching
- Price comparisons using Decimal's Ord trait

## Integration Points

- **Task 5 (Shopping Cart API)**: Will use ProductService to:
  - Validate products exist before adding to cart
  - Check inventory availability
  - Retrieve product details for cart items
  - Display product names and prices in cart

## Risks and Mitigation

**Risk**: Cargo.toml conflicts with Task 1 (Database Schema)
- **Mitigation**: This is expected and will be handled by the CTO platform's conflict detection. Both tasks add different dependencies.

**Risk**: Thread safety issues with concurrent access
- **Mitigation**: Using Arc<Mutex> pattern ensures thread-safe access. In-memory design is simple for test purposes.

**Risk**: rust_decimal version compatibility
- **Mitigation**: Specifying version 1.30+ with serde feature ensures compatibility with other dependencies

## Success Criteria

1. ✅ `src/catalog/mod.rs` exists and exports Product and ProductService
2. ✅ `src/catalog/models.rs` exists with Product, NewProduct, and ProductFilter definitions
3. ✅ `src/catalog/service.rs` exists with complete ProductService implementation
4. ✅ All models use rust_decimal::Decimal for price fields
5. ✅ ProductService implements all required methods correctly
6. ✅ Thread-safe storage with Arc<Mutex<Vec<Product>>>
7. ✅ `Cargo.toml` includes rust_decimal dependency with serde feature
8. ✅ Code passes `cargo check` without errors
9. ✅ Filter method supports all query types
10. ✅ Module is ready for integration with Task 5

## Estimated Effort
**40 minutes** - Module creation with in-memory service implementation, filtering logic, and thread-safe storage patterns
