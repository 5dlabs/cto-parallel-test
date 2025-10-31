# Autonomous Agent Prompt: Product Catalog Module

## Role
Senior Rust developer specializing in service layer design and in-memory data management.

## Task
Implement product catalog module with CRUD operations, filtering, and inventory management.

## Deliverables
1. src/catalog/mod.rs
2. src/catalog/models.rs (Product, NewProduct, ProductFilter)
3. src/catalog/service.rs (ProductService with create, get_all, get_by_id, update_inventory, filter)

## Dependencies (Cargo.toml)
```toml
rust_decimal = { version = "1.30", features = ["serde"] }
```

## Success Criteria
✅ Thread-safe storage with Arc<Mutex>
✅ Auto-incrementing product IDs
✅ Decimal price handling
✅ Filtering by name, price range, stock status
✅ All tests pass

## Testing
```bash
cargo test catalog::
```
