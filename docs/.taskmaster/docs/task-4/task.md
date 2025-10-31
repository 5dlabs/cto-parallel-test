# Task 4: Product Catalog Module

## Overview
Implement product catalog with CRUD operations, inventory management, and filtering using in-memory storage.

## Objectives
- Product creation, retrieval, update
- Inventory management
- Product filtering (name, price range, stock status)
- Thread-safe in-memory storage with Arc<Mutex>
- Decimal price handling

## Context
**Level 0** - Independent task, parallel with 1, 3, 6. No dependencies.

## Technical Specifications
- **Storage**: Arc<Mutex<Vec<Product>>>
- **Price**: rust_decimal 1.30 with serde
- **Operations**: CRUD + filter + inventory updates

## Implementation

### Dependencies (Cargo.toml)
```toml
rust_decimal = { version = "1.30", features = ["serde"] }
```

### Files
- `src/catalog/mod.rs` - Module exports
- `src/catalog/models.rs` - Product, NewProduct, ProductFilter structs
- `src/catalog/service.rs` - ProductService with CRUD operations

## Validation
```bash
cargo test catalog::
```

## Estimated Effort
40 minutes
