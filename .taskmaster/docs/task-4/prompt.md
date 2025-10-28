# Autonomous Agent Prompt: Product Catalog Module

## Mission
You are tasked with creating a product catalog module with inventory management for a Rust API project. This module will provide product CRUD operations, filtering capabilities, and inventory tracking using in-memory storage with thread-safe access patterns.

## Prerequisites
**None** - This is a Level 0 task with no dependencies. You can proceed immediately.

## What You Need to Do

### 1. Create Module Structure (`src/catalog/mod.rs`)
Create a new file at `src/catalog/mod.rs` to export catalog components:

```rust
pub mod models;
pub mod service;

pub use self::models::Product;
pub use self::service::ProductService;
```

### 2. Define Product Models (`src/catalog/models.rs`)
Create `src/catalog/models.rs` with product data structures:

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

**What these models do:**
- `Product`: Complete product with auto-generated ID
- `NewProduct`: Data for creating new products (no ID yet)
- `ProductFilter`: Optional query parameters for searching products

### 3. Implement ProductService (`src/catalog/service.rs`)
Create `src/catalog/service.rs` with the service implementation:

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

**Service Methods Explained:**
- `new()`: Creates empty product catalog
- `create()`: Adds new product with auto-generated ID
- `get_all()`: Returns all products
- `get_by_id()`: Finds specific product by ID
- `update_inventory()`: Updates stock count for a product
- `filter()`: Searches products by name, price range, and stock status

### 4. Update Cargo.toml
Add the rust_decimal dependency to the `[dependencies]` section:

```toml
rust_decimal = { version = "1.30", features = ["serde"] }
```

**Why rust_decimal?**
- Provides exact decimal arithmetic for monetary values
- Avoids floating-point precision errors
- The `serde` feature enables JSON serialization

## Expected Behavior

After implementation:
- ProductService can store and retrieve products in memory
- All operations are thread-safe (can be used from multiple threads)
- Products use precise decimal arithmetic for prices
- Filter method supports flexible product queries
- Service is ready for Task 5 (Shopping Cart) to use

## Validation Steps

Before marking this task complete, verify:

1. **File Structure**
   ```bash
   ls -la src/catalog/mod.rs
   ls -la src/catalog/models.rs
   ls -la src/catalog/service.rs
   ```

2. **Syntax Check**
   ```bash
   cargo check
   ```
   Should complete without errors.

3. **Dependency Check**
   ```bash
   grep "rust_decimal" Cargo.toml
   ```
   Should show the dependency with serde feature.

4. **Module Compilation**
   The catalog module should compile and be importable by other modules.

## Constraints

- This is a test project - keep implementation simple
- Use in-memory storage (Arc<Mutex<Vec>>) not a database
- Follow exact struct definitions and method signatures provided
- Do not add extra features beyond requirements
- Ensure thread safety with proper mutex locking
- Use rust_decimal for all price fields

## Common Issues and Solutions

**Issue**: Cargo.toml conflict with Task 1
- **Solution**: This is expected. The CTO platform will handle merging dependencies from both tasks.

**Issue**: Mutex lock() might panic
- **Solution**: Using unwrap() is acceptable for this test project. Production code would handle poisoned locks.

**Issue**: Clone performance concerns
- **Solution**: For this in-memory test project, cloning is acceptable. Production would use database queries.

## Success Definition

Task is complete when:
- ✅ `src/catalog/mod.rs` created with proper exports
- ✅ `src/catalog/models.rs` created with Product, NewProduct, and ProductFilter
- ✅ `src/catalog/service.rs` created with complete ProductService
- ✅ All price fields use rust_decimal::Decimal type
- ✅ Service uses Arc<Mutex<Vec<Product>>> for thread-safe storage
- ✅ All five methods implemented: new, create, get_all, get_by_id, update_inventory, filter
- ✅ `Cargo.toml` updated with rust_decimal dependency
- ✅ `cargo check` passes without errors

## Integration Notes

- **Task 5 (Shopping Cart API)** will depend on this module
- Task 5 will use ProductService to validate products and check inventory
- The cart will need to call `get_by_id()` and check `inventory_count`
- Ensure the module is public and can be imported by other modules

## Next Steps

After completing this task:
1. Task 5 (Shopping Cart API) can begin (it depends on Tasks 3 and 4)
2. The ProductService will be integrated into the main application
3. API routes will be added to expose product operations
