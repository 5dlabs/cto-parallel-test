# Task 5: Shopping Cart API

## Overview
Implement shopping cart functionality with JWT authentication, integrating Task 3 (Auth) and Task 4 (Catalog).

## Objectives
- Cart management (create, add, remove, clear)
- JWT-protected cart endpoints
- Inventory validation
- Integration with auth and catalog modules

## Context
**Level 1** - Depends on Tasks 3 & 4. Parallel with Task 2.

## Technical Specifications
- Cart service with user isolation
- API routes at /api/cart
- JWT token extraction from Authorization header
- Inventory checks before adding items

## Implementation
- src/cart/mod.rs, service.rs
- src/api/cart_routes.rs
- Integration with auth::jwt and catalog::ProductService

## Validation
```bash
cargo test cart::
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/cart
```

## Estimated Effort
45 minutes
