# Task 2: API Endpoints - Agent Prompt

You are a Rust backend developer tasked with creating the REST API endpoint structure using Actix-web.

## Your Mission
Set up the HTTP server and routing infrastructure for a test e-commerce API. Create modular routing with a health check endpoint and placeholders for user and product routes that will be implemented by other tasks.

## What You Must Create

### 1. Update `Cargo.toml`
Add these dependencies to the `[dependencies]` section (Task 1 already added database deps):
```toml
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2. Create `src/api/mod.rs`
Simple module export:
```rust
pub mod routes;
```

### 3. Create `src/api/routes.rs`
Implement complete routing structure with:
- `configure_routes(cfg: &mut web::ServiceConfig)` function
- `/api` scope with nested `/users` and `/products` scopes
- `health_check()` async handler returning JSON `{"status": "ok"}`
- `user_routes()` placeholder returning NotImplemented
- `product_routes()` placeholder returning NotImplemented
- Import `crate::schema` to validate Task 1 dependency

### 4. Create/Update `src/main.rs`
Implement server startup:
- Use `#[actix_web::main]` macro
- Import `api` and `schema` modules
- Create `HttpServer` binding to `127.0.0.1:8080`
- Configure with `api::routes::configure_routes`
- Return `std::io::Result<()>`

## Key Requirements

✅ **Routing Structure**:
- Base scope: `/api`
- Nested scopes: `/api/users`, `/api/products`
- Direct endpoint: `/api/health`

✅ **Health Check**:
- HTTP GET method
- Returns 200 OK
- JSON body: `{"status": "ok"}`

✅ **Placeholder Routes**:
- User and product routes return 501 Not Implemented
- Use `HttpResponse::NotImplemented()`
- Can be empty endpoints (just for structure)

✅ **Dependencies**:
- Must import `schema` module from Task 1
- Server must compile after Task 1 completes

## Constraints
- This is a **Level 1** task depending on Task 1
- Also modifies `Cargo.toml` (like Task 1) - use standard formatting
- Keep implementations simple - this is a test project
- Placeholder routes are intentional - other tasks will implement

## Validation
After completing the work:
1. Verify all files exist at specified paths
2. Run `cargo check` to ensure compilation
3. Run `cargo run` to start server (if Task 1 complete)
4. Test health endpoint: `curl http://localhost:8080/api/health`
5. Verify placeholder routes exist but return 501

## Success Definition
Task is complete when:
- `src/api/mod.rs` and `src/api/routes.rs` exist
- `src/main.rs` is properly configured
- `Cargo.toml` includes all required dependencies
- Server starts and binds to port 8080
- Health check returns correct JSON response
- Code compiles with schema import from Task 1

## Context
You're working on a parallel task execution test.

**Your dependencies**:
- Task 1: Database Schema (must complete before you finish)

**Tasks depending on you**:
- Task 5: Shopping Cart API (needs your route structure)
- Task 7: Integration Tests (will test your endpoints)

**Parallel to you (Level 1)**:
- Task 5: Shopping Cart API (also Level 1, different dependencies)

**Your work enables**:
- Task 3 to add authentication routes
- Task 4 to add product routes
- Task 5 to add cart routes
- Task 7 to test all endpoints

---

**Start working now. Create the files, write the code, and verify the server starts correctly.**
