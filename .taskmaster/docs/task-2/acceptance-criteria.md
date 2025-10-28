# Task 2: API Endpoints - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `src/api/mod.rs` exists
- [ ] `src/api/routes.rs` exists
- [ ] `src/main.rs` exists (created or updated)
- [ ] `Cargo.toml` has been updated with web dependencies

## Code Quality Criteria

### ✅ API Module (src/api/mod.rs)
- [ ] Contains `pub mod routes;` export
- [ ] Valid Rust module syntax
- [ ] Properly exposes routes submodule

### ✅ Routes Module (src/api/routes.rs)
- [ ] Imports required Actix-web types (web, HttpResponse, Scope)
- [ ] Imports `crate::schema` (validates Task 1 dependency)
- [ ] Contains `configure_routes(cfg: &mut web::ServiceConfig)` function
- [ ] Creates `/api` base scope
- [ ] Registers `health_check` handler
- [ ] Creates nested `/users` scope with `user_routes` configuration
- [ ] Creates nested `/products` scope with `product_routes` configuration
- [ ] Health check handler:
  - Uses `#[actix_web::get("/health")]` attribute macro
  - Declared as `async fn health_check() -> HttpResponse`
  - Returns `HttpResponse::Ok().json(...)` with status object
  - JSON body contains `{"status": "ok"}`
- [ ] `user_routes(cfg: &mut web::ServiceConfig)` placeholder exists
  - Returns `HttpResponse::NotImplemented()` for empty resource
- [ ] `product_routes(cfg: &mut web::ServiceConfig)` placeholder exists
  - Returns `HttpResponse::NotImplemented()` for empty resource

### ✅ Main Application (src/main.rs)
- [ ] Uses `#[actix_web::main]` attribute macro
- [ ] Imports `actix_web::{App, HttpServer}`
- [ ] Declares `mod api;`
- [ ] Declares `mod schema;`
- [ ] `main()` function signature: `async fn main() -> std::io::Result<()>`
- [ ] Creates `HttpServer::new()` with app factory
- [ ] App factory creates `App::new()`
- [ ] Configures app with `.configure(api::routes::configure_routes)`
- [ ] Binds to `"127.0.0.1:8080"`
- [ ] Calls `.run().await`
- [ ] Prints startup message (e.g., "Starting API server")

### ✅ Dependencies (Cargo.toml)
- [ ] Includes `actix-web = "4.3.1"`
- [ ] Includes `serde = { version = "1.0", features = ["derive"] }`
- [ ] Includes `serde_json = "1.0"`
- [ ] Dependencies from Task 1 still present (diesel, r2d2, dotenv)
- [ ] All dependencies in `[dependencies]` section
- [ ] Valid TOML syntax

## Compilation and Runtime Criteria

### ✅ Build Verification
- [ ] `cargo check` completes without errors
- [ ] `cargo build` completes successfully
- [ ] No warnings related to unused imports
- [ ] Schema module import resolves (Task 1 complete)

### ✅ Server Startup
- [ ] `cargo run` starts server without panicking
- [ ] Server binds to port 8080 successfully
- [ ] Startup message printed to console
- [ ] Server accepts HTTP connections

### ✅ Endpoint Functionality
- [ ] Health check endpoint accessible at `/api/health`
- [ ] Health check returns HTTP 200 OK
- [ ] Health check returns JSON with `Content-Type: application/json`
- [ ] Health check body matches `{"status":"ok"}` (spacing may vary)
- [ ] User routes scope exists at `/api/users`
- [ ] User placeholder returns HTTP 501 Not Implemented
- [ ] Product routes scope exists at `/api/products`
- [ ] Product placeholder returns HTTP 501 Not Implemented

## Integration Criteria

### ✅ Task 1 Integration
- [ ] Successfully imports `crate::schema`
- [ ] Compilation succeeds with schema definitions present
- [ ] No conflicts in Cargo.toml dependencies

### ✅ Extensibility for Future Tasks
- [ ] Route configuration pattern allows Task 3 to implement user authentication
- [ ] Route configuration pattern allows Task 4 to implement product catalog
- [ ] Route configuration pattern allows Task 5 to add cart routes
- [ ] Modular structure supports testing in Task 7

## Testing Commands

### Manual Validation Steps

1. **Verify File Existence**
   ```bash
   ls -la src/api/mod.rs
   ls -la src/api/routes.rs
   ls -la src/main.rs
   ```

2. **Check Rust Compilation**
   ```bash
   cargo check
   cargo build
   ```

3. **Validate Dependencies**
   ```bash
   cargo tree | grep actix-web
   cargo tree | grep serde
   ```

4. **Start Server**
   ```bash
   cargo run &
   sleep 2  # Wait for server startup
   ```

5. **Test Health Check Endpoint**
   ```bash
   curl -i http://localhost:8080/api/health
   # Expected: HTTP/1.1 200 OK
   # Expected body: {"status":"ok"}
   ```

6. **Test Placeholder Routes**
   ```bash
   curl -i http://localhost:8080/api/users
   # Expected: HTTP/1.1 501 Not Implemented

   curl -i http://localhost:8080/api/products
   # Expected: HTTP/1.1 501 Not Implemented
   ```

7. **Stop Server**
   ```bash
   pkill -f "cargo run"
   ```

### Automated Test Script
```bash
#!/bin/bash
set -e

echo "Building project..."
cargo build --quiet

echo "Starting server..."
cargo run &
SERVER_PID=$!
sleep 3

echo "Testing health endpoint..."
RESPONSE=$(curl -s http://localhost:8080/api/health)
echo $RESPONSE | grep -q '"status":"ok"' || (echo "Health check failed"; exit 1)

echo "Testing user placeholder..."
STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/users)
[ "$STATUS" = "501" ] || (echo "User route should return 501"; exit 1)

echo "Testing product placeholder..."
STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/products)
[ "$STATUS" = "501" ] || (echo "Product route should return 501"; exit 1)

echo "Stopping server..."
kill $SERVER_PID

echo "All tests passed!"
```

## Success Definition

**Task is COMPLETE when:**
1. All required files exist and contain correct implementations
2. Server compiles and starts without errors
3. Health check endpoint returns correct JSON response
4. Placeholder routes return 501 status
5. Schema import from Task 1 resolves successfully
6. No dependency conflicts in Cargo.toml

**Task is INCOMPLETE if:**
- Any required file is missing
- Compilation errors exist
- Server fails to start or bind to port
- Health check doesn't return expected JSON
- Schema import fails (Task 1 not complete)

## Estimated Completion Time
50 minutes (as specified in PRD)

## Dependencies
- **Task 1**: Database Schema Setup (required)

## Blocks
- **Task 5**: Shopping Cart API (partially - provides route structure)
- **Task 7**: Integration Tests (provides endpoints to test)

## Notes
- Port 8080 must be available during testing
- Server should gracefully handle Ctrl+C shutdown
- Placeholder implementations are intentional and expected
