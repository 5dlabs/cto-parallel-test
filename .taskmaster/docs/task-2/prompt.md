# Autonomous Agent Prompt: API Endpoints

## Mission
Create REST API endpoints and HTTP server infrastructure using Actix-web framework for a Rust e-commerce application. Establish routing structure with health check endpoint and placeholders for future implementation.

## Prerequisites
- **Task 1 must be complete**: This task requires `src/schema.rs` from Task 1
- Verify Task 1 status before proceeding

## What You Need to Do

### 1. Create API Module Structure
Create `src/api/mod.rs`:
```rust
pub mod routes;
```

### 2. Implement Route Configuration
Create `src/api/routes.rs` with complete routing setup:

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

### 3. Set Up HTTP Server
Update `src/main.rs`:

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

### 4. Update Dependencies
Add to `Cargo.toml` [dependencies] section:

```toml
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Note**: Task 1 also modified Cargo.toml. If conflict exists, merge both sets of dependencies.

## Expected Behavior
- Server starts on localhost:8080
- `/api/health` returns `{"status": "ok"}` with 200 OK
- `/api/users` returns 501 Not Implemented (placeholder)
- `/api/products` returns 501 Not Implemented (placeholder)
- Schema module imports successfully

## API Endpoints

### Health Check
- **URL**: `GET /api/health`
- **Response**: `200 OK` with `{"status": "ok"}`
- **Purpose**: Service health monitoring

### User Routes (Placeholder)
- **URL**: `GET /api/users`
- **Response**: `501 Not Implemented`
- **Purpose**: Will be implemented by Task 3

### Product Routes (Placeholder)
- **URL**: `GET /api/products`
- **Response**: `501 Not Implemented`
- **Purpose**: Will be implemented by Task 4

## Validation Steps
Before marking complete:

1. **Directory Structure**:
   ```bash
   ls -la src/api/mod.rs
   ls -la src/api/routes.rs
   ```

2. **Compilation Check**:
   ```bash
   cargo check
   ```

3. **Dependency Verification**:
   ```bash
   grep actix-web Cargo.toml
   ```

4. **Optional Runtime Test** (if environment allows):
   ```bash
   cargo run &
   sleep 2
   curl http://localhost:8080/api/health
   ```

## Constraints
- Use exact route paths specified
- Import schema to establish Task 1 dependency
- Keep server simple - no middleware yet
- Placeholder routes must return 501 Not Implemented
- Use Actix-web 4.3.1 specifically

## Common Issues

**Issue**: Cannot find `schema` module
- **Solution**: Verify Task 1 is complete and `src/schema.rs` exists

**Issue**: Cargo.toml merge conflict
- **Solution**: Merge dependencies from both tasks

**Issue**: Async/await syntax errors
- **Solution**: Ensure `#[actix_web::main]` is present on main function

## Success Definition
Task is complete when:
- ✅ API module structure created (mod.rs, routes.rs)
- ✅ HTTP server implemented in main.rs
- ✅ Health check endpoint functional
- ✅ Placeholder routes configured
- ✅ Dependencies added to Cargo.toml
- ✅ `cargo check` passes without errors
- ✅ Schema module imports successfully

## Integration Notes
- Task 3 will add implementations to user_routes
- Task 4 will add implementations to product_routes
- Task 5 will add cart routes to configure_routes
- Task 7 will test all endpoints

Keep implementation simple and focused on structure - detailed logic comes in later tasks.
