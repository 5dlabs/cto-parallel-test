# Autonomous Agent Prompt: API Endpoints Setup

## Role
You are a senior Rust web developer specializing in Actix-web framework and REST API design.

## Task
Set up the foundational REST API structure for an e-commerce application using Actix-web 4.3.1, including server configuration, route organization, health check endpoint, and error handling.

## Objectives
1. Configure Actix-web HTTP server with proper middleware
2. Implement scoped routing structure under `/api` prefix
3. Create functional health check endpoint
4. Set up placeholder routes for users, products, and cart
5. Implement standardized error handling
6. Integrate with database schema from Task 1

## Required Deliverables

### 1. Dependencies (Cargo.toml)
Add:
```toml
actix-web = "4.3.1"
actix-rt = "2.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
env_logger = "0.10"
log = "0.4"
```

### 2. API Module Structure (src/api/)
- `mod.rs` - Export routes and errors modules
- `routes.rs` - Route configuration and handlers
- `errors.rs` - Error types and responses

### 3. Route Configuration (src/api/routes.rs)
Implement:
- `configure_routes(cfg: &mut web::ServiceConfig)` - Main configuration
- `health_check()` - GET /api/health endpoint
- `user_routes(cfg)` - Placeholder for Task 3
- `product_routes(cfg)` - Placeholder for Task 4
- `cart_routes(cfg)` - Placeholder for Task 5
- `not_implemented()` - Handler returning 501

### 4. Error Handling (src/api/errors.rs)
Define:
- `ErrorResponse` struct with error, message, status fields
- `ApiError` enum with NotFound, BadRequest, InternalError, Unauthorized variants
- Implement `Display` and `ResponseError` traits

### 5. Main Server (src/main.rs)
- Load environment with dotenv
- Initialize logging with env_logger
- Create HttpServer with App and middleware
- Configure routes using `configure_routes`
- Bind to 127.0.0.1:8080
- Add startup log messages

## Implementation Steps
1. Update Cargo.toml with Actix-web dependencies
2. Create src/api directory structure
3. Implement errors.rs with ApiError types
4. Implement routes.rs with configure_routes function
5. Implement health_check endpoint
6. Create placeholder route configurations for users, products, cart
7. Update src/main.rs with server setup
8. Add logging middleware
9. Test server startup: `cargo run`
10. Test health endpoint: `curl http://localhost:8080/api/health`
11. Verify placeholder routes return 501

## Success Criteria
✅ Server starts on port 8080 without errors
✅ Health check endpoint returns 200 OK with JSON response
✅ Health response includes status, service, and version fields
✅ Placeholder endpoints return 501 Not Implemented
✅ Error responses follow standardized JSON format
✅ Request logging active and visible in console
✅ All routes properly scoped under /api
✅ Code compiles without warnings: `cargo check`

## Testing Commands
```bash
cargo build
cargo run

# In separate terminal:
curl http://localhost:8080/api/health
curl http://localhost:8080/api/users
curl http://localhost:8080/api/products
curl http://localhost:8080/api/cart
```

## Expected Responses
- Health: `{"status":"ok","service":"e-commerce-api","version":"0.1.0"}`
- Placeholders: `{"error":"This endpoint is not yet implemented",...}` (status 501)

## Constraints
- Must depend on Task 1 (import schema and models modules)
- Must not implement actual business logic (reserved for Tasks 3, 4, 5)
- Must use async handlers
- Must bind to 127.0.0.1:8080
- Must use /api prefix for all routes

## Dependencies
**Task 1 (Database Schema Setup)** must be complete before starting this task.

## Output
Confirm:
1. Server starts successfully
2. Health check endpoint tested and working
3. Placeholder routes return correct 501 status
4. Logging middleware active
5. No compilation warnings or errors
