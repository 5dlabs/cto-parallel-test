# Acceptance Criteria: Task 5 - Shopping Cart API

## Must Have
- [ ] src/cart/ with mod.rs, service.rs created
- [ ] src/api/cart_routes.rs with all endpoints
- [ ] CartService implements get_or_create_cart, add_item, remove_item, get_cart, clear_cart
- [ ] All endpoints extract JWT from Authorization header
- [ ] Inventory validation before adding items
- [ ] Returns 401 for missing/invalid tokens
- [ ] cargo check passes
- [ ] Integration tests pass

## Validation
```bash
cargo test cart::
cargo test integration_tests::test_full_user_flow
```

## Definition of Done
✅ Cart API fully functional with auth
✅ Integrates with Task 3 and 4 modules
✅ Ready for Task 7 testing
