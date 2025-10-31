# Acceptance Criteria: Task 4 - Product Catalog Module

## Must Have
- [ ] rust_decimal 1.30 dependency added
- [ ] src/catalog/ directory created with mod.rs, models.rs, service.rs
- [ ] Product, NewProduct, ProductFilter structs implemented
- [ ] ProductService with create, get_all, get_by_id, update_inventory, filter methods
- [ ] Thread-safe storage using Arc<Mutex>
- [ ] Decimal price handling
- [ ] cargo check passes
- [ ] Unit tests for all service methods pass

## Validation
```bash
cargo build
cargo test catalog::
```

## Definition of Done
✅ All CRUD operations work
✅ Filtering by name, price, stock status works
✅ Thread-safe concurrent access
✅ Ready for Task 5 integration
