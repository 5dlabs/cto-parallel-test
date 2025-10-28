# Task 2: API Endpoints

## Overview
Create REST API endpoints for core operations of the application using Actix-web framework. This is a Level 1 task that depends on Task 1 (Database Schema) and provides the HTTP interface layer for the application.

## Context
This task establishes the web server and routing infrastructure for the e-commerce test API. It creates the foundation that other modules (authentication, catalog, cart) will build upon by registering their route handlers.

## Objectives
1. Set up Actix-web HTTP server
2. Create modular routing structure
3. Implement health check endpoint
4. Define placeholder route configurations for users and products
5. Configure the main application entry point

## Dependencies
- **Task 1: Database Schema Setup** - Required to import schema definitions

## Blocked By
None initially, but needs Task 1's schema.rs to complete successfully.

## Files to Create/Modify
- `src/api/mod.rs` - API module exports
- `src/api/routes.rs` - Route definitions and configuration
- `src/main.rs` - Application entry point and server setup
- `Cargo.toml` - Add Actix-web dependencies

## Technical Specifications

### Web Framework
- **Framework**: Actix-web 4.3.1
- **Serialization**: Serde 1.0 with derive feature
- **JSON**: serde_json 1.0
- **Server**: Asynchronous HTTP server on 127.0.0.1:8080

### Architecture Pattern
- **Modular routing**: Separate modules for different API domains
- **Scope-based organization**: `/api` base scope with nested scopes
- **Configuration pattern**: ServiceConfig-based route registration
- **Async runtime**: Actix-web's built-in async runtime

### API Structure
```
/api
├── /health (GET) - Health check endpoint
├── /users/* - User-related endpoints (placeholder)
└── /products/* - Product-related endpoints (placeholder)
```

## Implementation Plan

### Step 1: Update Cargo.toml
Add web framework dependencies:

```toml
[dependencies]
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Note**: Task 1 already added database dependencies. Ensure no conflicts.

### Step 2: Create API Module (src/api/mod.rs)
Simple module export file:

```rust
pub mod routes;
```

This establishes the API module namespace and exports the routes submodule.

### Step 3: Create Route Configuration (src/api/routes.rs)
Implement the core routing logic:

```rust
use actix_web::{web, HttpResponse, Scope};
use crate::schema;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
    );
}

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

fn user_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder - Task 3 will implement
    cfg.service(web::resource("").route(web::get().to(|| HttpResponse::NotImplemented())));
}

fn product_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder - Task 4 will implement
    cfg.service(web::resource("").route(web::get().to(|| HttpResponse::NotImplemented())));
}
```

**Key Design Decisions**:
- `configure_routes` accepts `ServiceConfig` for modular composition
- Health check uses Actix-web's macro routing (`#[actix_web::get]`)
- Placeholder routes return 501 Not Implemented
- Import schema to validate Task 1 dependency

### Step 4: Create Main Application (src/main.rs)
Set up the HTTP server:

```rust
use actix_web::{App, HttpServer};
mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API server");

    HttpServer::new(|| {
        App::new()
            .configure(api::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Key Features**:
- Uses `#[actix_web::main]` macro for async runtime
- Imports both `api` and `schema` modules
- Single app instance with route configuration
- Binds to localhost port 8080
- Returns IO errors for proper error propagation

### Step 5: Verify Dependencies Resolution
After all file changes:

```bash
cargo check
cargo tree
```

Ensure no dependency conflicts between Task 1's database deps and Task 2's web deps.

## Architectural Considerations

### Modularity
The routing structure is designed for extension:
- Each domain (users, products, cart) gets its own scope
- Route configuration is delegated to domain-specific functions
- Future tasks can implement actual handlers in separate modules

### Async Design
- All handlers are `async fn`
- Actix-web provides actor-based concurrency
- Non-blocking I/O for database and external services

### Error Handling
- Server startup errors propagate via `std::io::Result`
- Route handlers will implement proper error responses
- Health check always succeeds (for monitoring)

### Conflict Points
This task modifies `Cargo.toml`, which Task 1 also modified. The orchestrator must merge:
- Task 1's database dependencies
- Task 2's web framework dependencies

Both sets are independent and should merge cleanly.

## Risks and Considerations

1. **Cargo.toml Conflicts**: Both Task 1 and Task 2 modify this file. Git merge should handle cleanly since different dependencies are added.

2. **Schema Import**: The `use crate::schema;` line validates that Task 1 completed. Without it, compilation fails.

3. **Placeholder Routes**: The NotImplemented responses are intentional. Tasks 3, 4, and 5 will replace these.

4. **Port Binding**: Port 8080 must be available. In a real deployment, this would be configurable.

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- HTTP server starts without errors
- Health check endpoint responds with 200 OK
- Placeholder routes return 501 Not Implemented
- Code compiles with Task 1's schema
- Dependencies resolve without conflicts

## Related Tasks
- **Task 1**: Database Schema (dependency - must complete first)
- **Task 3**: User Authentication (will implement user_routes)
- **Task 4**: Product Catalog (will implement product_routes)
- **Task 5**: Shopping Cart (will add cart routes)
- **Task 7**: Integration Tests (will test these endpoints)

## Diagram
See `diagrams.mmd` for visual representation of the routing structure.

## References
- [Actix-web Documentation](https://actix.rs/)
- [Actix-web Routing](https://actix.rs/docs/url-dispatch/)
- Project PRD: `.taskmaster/docs/prd.txt`
