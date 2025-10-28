# Task 2: API Endpoints

## Overview
Create REST API endpoints for core operations of the application using Actix-web framework. This is a Level 1 task that depends on Task 1 (Database Schema Setup) and establishes the HTTP server and routing infrastructure.

## Context
This task builds the backbone REST API structure for the e-commerce application. It creates the main HTTP server, configures routing, and sets up placeholder endpoints that will be implemented by other tasks. This is part of the parallel task execution test to validate dependency management.

## Objectives
1. Create modular API structure with `src/api/mod.rs` and `src/api/routes.rs`
2. Configure Actix-web HTTP server in `src/main.rs`
3. Set up health check endpoint and placeholder routes
4. Add Actix-web and serialization dependencies to `Cargo.toml`

## Dependencies
- **Task 1 (Database Schema Setup)**: Required - needs schema definitions in `src/schema.rs`
- Level: 1 (runs after Level 0 tasks complete)

## Files to Create/Modify

### 1. `src/api/mod.rs`
Module declaration file to export API components:

```rust
pub mod routes;
```

### 2. `src/api/routes.rs`
Main routing configuration with health check and placeholder routes:

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
    // User routes will be implemented in Task 3
    cfg.service(web::resource("").route(web::get().to(|| HttpResponse::NotImplemented())));
}

fn product_routes(cfg: &mut web::ServiceConfig) {
    // Product routes will be implemented in Task 4
    cfg.service(web::resource("").route(web::get().to(|| HttpResponse::NotImplemented())));
}
```

**Key Design Decisions:**
- Nested route scopes for organization (`/api/users`, `/api/products`)
- Health check endpoint for monitoring
- Placeholder routes return 501 Not Implemented
- Import schema to establish dependency on Task 1

### 3. `src/main.rs`
HTTP server setup and application entry point:

```rust
use actix_web::{App, HttpServer};
mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API server on 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(api::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Implementation Notes:**
- Uses `#[actix_web::main]` for async runtime
- Binds to localhost port 8080
- Imports both api and schema modules

### 4. `Cargo.toml` Updates
Add web framework and serialization dependencies:

```toml
[dependencies]
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Expected Conflict**: Task 1 also modifies Cargo.toml (database deps). This conflict is intentional for testing the platform's merge capabilities.

## Implementation Steps

1. **Create API Module Structure**
   - Create `src/api/` directory
   - Add `mod.rs` with module exports
   - Set up basic module organization

2. **Implement Route Configuration**
   - Create `routes.rs` with configure_routes function
   - Implement health check endpoint
   - Add placeholder route functions
   - Import schema to establish dependency

3. **Set Up HTTP Server**
   - Update `main.rs` with Actix-web server setup
   - Configure async runtime
   - Add route configuration
   - Import required modules

4. **Add Dependencies**
   - Update Cargo.toml with Actix-web
   - Add serde for JSON serialization
   - Add serde_json for JSON handling

5. **Test Compilation**
   - Run `cargo check` to verify syntax
   - Ensure imports resolve correctly
   - Validate async/await usage

## Technical Considerations

### Framework Choice: Actix-web
- High-performance async web framework
- Actor-based architecture
- Built on Tokio async runtime
- Compile-time type checking for routes

### Routing Architecture
- Hierarchical scope-based routing
- Centralized route configuration
- Easy to extend with new endpoints
- Clear separation of concerns

### Placeholder Strategy
- NotImplemented (501) responses for unfinished routes
- Allows dependent tasks to add implementations
- Maintains valid route structure

### Module Organization
```
src/
├── main.rs          (server entry point)
├── schema.rs        (from Task 1)
└── api/
    ├── mod.rs       (module exports)
    └── routes.rs    (route configuration)
```

## Integration Points

- **Task 1 (Database Schema)**: Imports schema.rs to establish dependency
- **Task 3 (User Authentication)**: Will implement user_routes
- **Task 4 (Product Catalog)**: Will implement product_routes
- **Task 5 (Shopping Cart API)**: Will add cart routes
- **Task 7 (Integration Tests)**: Will test all endpoints

## Risks and Mitigation

**Risk**: Cargo.toml merge conflict with Task 1
- **Mitigation**: Expected - tests platform's conflict resolution
- **Resolution**: Manual merge or platform auto-merge

**Risk**: Schema import fails if Task 1 incomplete
- **Mitigation**: Task 2 runs at Level 1 after Task 1 completes
- **Validation**: Check Task 1 status before starting

**Risk**: Port 8080 already in use during testing
- **Mitigation**: Documentation notes this is test code
- **Alternative**: Use dynamic port assignment if needed

## Success Criteria

1. ✅ `src/api/mod.rs` exists with module exports
2. ✅ `src/api/routes.rs` exists with route configuration
3. ✅ `src/main.rs` updated with HTTP server setup
4. ✅ `Cargo.toml` includes Actix-web dependencies
5. ✅ Health check endpoint implemented at `/api/health`
6. ✅ Placeholder routes exist for `/api/users` and `/api/products`
7. ✅ Schema module imported successfully
8. ✅ Code compiles with `cargo check`
9. ✅ Server can start and bind to port 8080

## Estimated Effort
**50 minutes** - Module setup, routing configuration, and server implementation
