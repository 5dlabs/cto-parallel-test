# Autonomous Agent Prompt: Integration Tests

## Role
Senior QA engineer specializing in Rust integration testing and end-to-end flows.

## Task
Create comprehensive integration test suite covering all application functionality.

## Deliverables
1. tests/integration_tests.rs (health check, full user flow)
2. tests/api_tests.rs (product endpoints)
3. tests/auth_tests.rs (JWT, password hashing)

## Success Criteria
✅ All tests pass with cargo test
✅ Health check test verifies API availability
✅ Full user flow test covers product → cart → checkout
✅ Auth tests verify token and password functions
✅ Tests are independent and repeatable

## Testing
```bash
cargo test
```
