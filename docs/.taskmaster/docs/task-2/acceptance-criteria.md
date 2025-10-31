# Acceptance Criteria: Task 2 - API Endpoints

## File Creation Requirements

### ✅ Required Files
- [ ] `Cargo.toml` (updated with actix-web dependencies)
- [ ] `src/api/mod.rs`
- [ ] `src/api/routes.rs`
- [ ] `src/api/errors.rs`
- [ ] `src/main.rs` (updated with HttpServer configuration)

## Dependency Requirements

### ✅ Cargo.toml Must Include
```bash
cargo tree | grep -E "(actix-web|actix-rt)"
```
- [ ] actix-web = "4.3.1"
- [ ] actix-rt = "2.8" (may be transitive)
- [ ] serde and serde_json (from Task 1)

## Server Configuration

### ✅ HTTP Server Requirements
- [ ] Server binds to 127.0.0.1:8080
- [ ] Server starts without compilation errors
- [ ] Logger middleware enabled
- [ ] Routes configured via `configure_routes()`

**Validation:**
```bash
cargo run
# Expected: "Starting API server on http://127.0.0.1:8080"
```

## Endpoint Requirements

### ✅ Health Check Endpoint
```bash
curl -v http://localhost:8080/api/health
```
- [ ] Returns HTTP 200 OK
- [ ] Returns Content-Type: application/json
- [ ] Response body contains: `{"status":"ok"}`
- [ ] Endpoint is accessible via GET method

### ✅ User Routes Scope
```bash
curl -v http://localhost:8080/api/users
```
- [ ] Route scope registered at /api/users
- [ ] Returns HTTP 501 Not Implemented (placeholder)
- [ ] Responds to GET method

### ✅ Product Routes Scope
```bash
curl -v http://localhost:8080/api/products
```
- [ ] Route scope registered at /api/products
- [ ] Returns HTTP 501 Not Implemented (placeholder)
- [ ] Responds to GET method

## Code Quality Requirements

### ✅ Route Configuration
- [ ] `configure_routes()` function exists in src/api/routes.rs
- [ ] Uses `web::scope("/api")` for API prefix
- [ ] Health check uses `#[actix_web::get("/health")]` macro
- [ ] Placeholder routes use async closures

### ✅ Error Handling Module
- [ ] ApiError enum defined with NotFound, BadRequest, InternalServerError variants
- [ ] Implements `Display` trait
- [ ] Implements `ResponseError` trait from actix-web
- [ ] Returns proper JSON error responses

### ✅ Main Application
- [ ] Uses `#[actix_web::main]` macro
- [ ] HttpServer::new() creates app factory
- [ ] App includes logger middleware
- [ ] Routes configured via `.configure(api::routes::configure_routes)`
- [ ] Binds to correct address and port

## Compilation Requirements

### ✅ Build Success
```bash
cargo check
cargo build
```
- [ ] No compilation errors
- [ ] No critical warnings
- [ ] All imports resolve correctly

## Runtime Requirements

### ✅ Server Startup
```bash
cargo run
```
- [ ] Server starts successfully
- [ ] Prints startup message with address
- [ ] Logger outputs to console
- [ ] No panic or crash on startup

### ✅ Request Handling
```bash
# Health check
curl http://localhost:8080/api/health

# User routes
curl http://localhost:8080/api/users

# Product routes
curl http://localhost:8080/api/products
```
- [ ] All endpoints respond within 1 second
- [ ] Proper HTTP status codes returned
- [ ] JSON responses properly formatted
- [ ] Logger outputs each request

## Module Integration

### ✅ Module Exports
Verify in `src/main.rs` or `src/lib.rs`:
- [ ] `mod api;` declared
- [ ] API module accessible to main

### ✅ Submodule Structure
In `src/api/mod.rs`:
- [ ] `pub mod routes;`
- [ ] `pub mod errors;`

## Testing Requirements

### ✅ Manual Testing Checklist
1. **Server Starts**: `cargo run` executes without errors
2. **Health Check Works**: curl returns JSON response
3. **Placeholders Work**: User/product routes return 501
4. **Logging Works**: Requests appear in console output
5. **Port Binding Works**: No "address already in use" errors

### ✅ Optional Unit Tests
```rust
#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

## Downstream Dependencies

### ✅ Integration Readiness
- [ ] Route configuration can be extended by Task 5 (cart routes)
- [ ] Error handling module can be used by other modules
- [ ] Server structure supports adding more middleware
- [ ] No blocking issues for Task 7 (integration tests)

## Definition of Done

**Complete when:**
1. All required files created and properly structured
2. Server starts on port 8080 without errors
3. Health check endpoint returns expected JSON
4. Placeholder routes respond with 501 status
5. Logger middleware outputs request information
6. Code compiles without errors or critical warnings
7. Manual testing confirms all endpoints accessible

**Rejection Criteria:**
- Server fails to start
- Health check returns wrong status code or format
- Routes not properly scoped under /api
- Missing error handling module
- Compilation errors
- Missing required dependencies
