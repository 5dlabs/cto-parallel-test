# Acceptance Criteria: API Endpoints

## Required Files

### ✅ `src/api/mod.rs`
- [ ] File exists at `src/api/mod.rs`
- [ ] Contains `pub mod routes;` declaration
- [ ] Properly exports routes module

### ✅ `src/api/routes.rs`
- [ ] File exists at `src/api/routes.rs`
- [ ] Contains `configure_routes` function accepting `&mut web::ServiceConfig`
- [ ] Implements health check handler at `/api/health` route
- [ ] Health check returns JSON `{"status": "ok"}` with 200 status
- [ ] Contains `user_routes` placeholder function returning 501 Not Implemented
- [ ] Contains `product_routes` placeholder function returning 501 Not Implemented
- [ ] Imports `crate::schema` to establish Task 1 dependency
- [ ] Uses Actix-web routing with `web::scope` pattern
- [ ] Health check handler uses `#[actix_web::get("/health")]` attribute

### ✅ `src/main.rs`
- [ ] File exists at `src/main.rs`
- [ ] Imports `actix_web::{App, HttpServer}`
- [ ] Declares `mod api;` module
- [ ] Declares `mod schema;` module
- [ ] Contains async main function with `#[actix_web::main]` attribute
- [ ] Creates HttpServer with App configuration
- [ ] Calls `api::routes::configure_routes` in app configuration
- [ ] Binds server to `127.0.0.1:8080`
- [ ] Returns `std::io::Result<()>`
- [ ] Includes startup message logging

### ✅ `Cargo.toml` Updates
- [ ] Contains `actix-web = "4.3.1"` dependency
- [ ] Contains `serde = { version = "1.0", features = ["derive"] }` dependency
- [ ] Contains `serde_json = "1.0"` dependency
- [ ] All dependencies in `[dependencies]` section
- [ ] Successfully merges with Task 1 database dependencies

## Functional Requirements

### Route Structure
- [ ] `/api` base scope configured
- [ ] `/api/health` endpoint accessible
- [ ] `/api/users` scope configured (placeholder)
- [ ] `/api/products` scope configured (placeholder)
- [ ] Route hierarchy matches specification

### Health Check Endpoint
- [ ] Responds to GET requests
- [ ] Returns 200 OK status code
- [ ] Response body is valid JSON
- [ ] JSON contains `status` field with value `"ok"`
- [ ] No authentication required

### Placeholder Routes
- [ ] User routes return 501 Not Implemented
- [ ] Product routes return 501 Not Implemented
- [ ] Placeholders are ready for Task 3/4 implementation
- [ ] Comments indicate future implementation

## Validation Tests

### Compilation Tests
```bash
cargo check
```
- [ ] Compiles without errors
- [ ] No warnings related to new code
- [ ] All imports resolve correctly
- [ ] Async syntax is correct

### Dependency Resolution
```bash
cargo tree | grep actix-web
```
- [ ] Actix-web dependency resolves
- [ ] Version 4.3.1 is used
- [ ] No conflicting dependencies

### Module Structure
```bash
ls -la src/api/
```
- [ ] Directory exists
- [ ] Contains mod.rs
- [ ] Contains routes.rs

### Import Validation
- [ ] `use crate::schema;` compiles successfully
- [ ] Confirms Task 1 dependency is satisfied
- [ ] Schema types are accessible (even if not used yet)

## Integration Tests (Optional)

### Server Startup Test
```bash
cargo run &
sleep 2
curl http://localhost:8080/api/health
pkill -f "cargo run"
```
- [ ] Server starts without errors
- [ ] Binds to port 8080 successfully
- [ ] Health endpoint returns expected response

### Health Endpoint Response
Expected output from curl:
```json
{"status":"ok"}
```
- [ ] Valid JSON format
- [ ] Correct status field
- [ ] HTTP 200 status code

## Non-Functional Requirements

### Code Quality
- [ ] Follows Rust naming conventions
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] Proper use of async/await
- [ ] Clear function organization

### Architecture
- [ ] Modular structure (routes separated from main)
- [ ] Clear separation of concerns
- [ ] Extensible design for future routes
- [ ] Follows Actix-web best practices

### Documentation
- [ ] Comments explain placeholder routes
- [ ] Startup message is informative
- [ ] Code structure is self-documenting

### Error Handling
- [ ] Main function returns proper Result type
- [ ] Server bind errors propagated correctly
- [ ] No unwrap() calls without justification

## Dependency Validation

### Task 1 Dependency
- [ ] Task 1 (Database Schema) marked as complete
- [ ] `src/schema.rs` exists and is importable
- [ ] Schema import does not cause compilation errors

### Future Task Integration
- [ ] Route structure ready for Task 3 user endpoints
- [ ] Route structure ready for Task 4 product endpoints
- [ ] Route structure ready for Task 5 cart endpoints
- [ ] No blocking issues for dependent tasks

## Conflict Resolution

### Cargo.toml Merge
- [ ] Database dependencies from Task 1 preserved
- [ ] Web framework dependencies from Task 2 added
- [ ] No duplicate dependency entries
- [ ] All dependencies compatible with each other

Expected merged dependencies:
```toml
[dependencies]
# From Task 1
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
# From Task 2
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Edge Cases

### Port Already in Use
- [ ] Error message is clear if port 8080 occupied
- [ ] Server fails gracefully with descriptive error

### Missing Schema Module
- [ ] Compilation error clearly indicates missing dependency
- [ ] Error message references Task 1 requirement

## Performance Considerations

- [ ] Server starts quickly (under 5 seconds)
- [ ] Health endpoint responds rapidly
- [ ] No unnecessary allocations in hot path
- [ ] Appropriate use of async patterns

## Success Metrics

- **Completion**: All files created with correct implementations
- **Quality**: Code passes `cargo check` and follows best practices
- **Functionality**: Health endpoint works as specified
- **Integration**: Ready for Tasks 3, 4, and 5 to add implementations
- **Dependencies**: Successfully establishes Task 1 dependency

## Manual Verification Checklist

1. [ ] Verify all 3 files created/modified (mod.rs, routes.rs, main.rs)
2. [ ] Confirm Cargo.toml has all 3 new dependencies
3. [ ] Run `cargo check` - must pass
4. [ ] Verify schema import works
5. [ ] Check route structure matches specification
6. [ ] Confirm health check implementation
7. [ ] Verify placeholder routes return 501
8. [ ] Validate async/await usage
9. [ ] Check code formatting
10. [ ] Confirm Task 1 dependency satisfied
