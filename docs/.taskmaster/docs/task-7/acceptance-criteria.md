# Acceptance Criteria: Task 7 - Integration Tests

## Must Have
- [ ] tests/ directory with integration_tests.rs, api_tests.rs, auth_tests.rs
- [ ] test_health_check verifies /api/health endpoint
- [ ] test_full_user_flow creates product, adds to cart with JWT
- [ ] test_product_routes verifies product CRUD
- [ ] test_jwt_creation_and_validation verifies tokens
- [ ] test_password_hashing_and_verification verifies Argon2
- [ ] cargo test passes all tests
- [ ] Tests use actix_web::test utilities
- [ ] Mock services initialized properly

## Validation
```bash
cargo test
cargo test --test integration_tests
cargo test --test api_tests
cargo test --test auth_tests
```

## Definition of Done
✅ All tests pass
✅ Coverage of major flows
✅ Independent test execution
✅ Application validated end-to-end
