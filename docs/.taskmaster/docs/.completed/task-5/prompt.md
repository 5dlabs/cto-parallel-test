# Autonomous Agent Prompt: Shopping Cart API

## Role
Senior Rust API developer specializing in authenticated endpoints and service integration.

## Task
Implement shopping cart functionality with JWT-protected API endpoints.

## Deliverables
1. src/cart/service.rs (CartService)
2. src/api/cart_routes.rs (GET /cart, POST /cart/add, DELETE /cart/remove/{id}, POST /cart/clear)
3. Integration with auth and catalog modules

## Success Criteria
✅ All cart endpoints require JWT authentication
✅ Inventory validated before adding items
✅ Cart isolated per user
✅ Tests with mock JWT tokens pass

## Dependencies
Tasks 3 (Auth) and 4 (Catalog) must be complete.
