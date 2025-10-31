# Task 2: API Endpoints

## Overview
Establish the REST API endpoint structure for the e-commerce application using Actix-web. This task creates the HTTP server foundation, route configuration, and health check endpoint that will serve as the backbone for all API operations.

## Objectives
- Set up Actix-web HTTP server with proper configuration
- Implement route configuration with scoped endpoints
- Create health check endpoint for monitoring
- Establish placeholder routes for future modules (users, products, cart)
- Configure error handling and JSON response serialization
- Integrate with database schema from Task 1

## Context
This is a **Level 1** task that depends on Task 1 (Database Schema Setup). It cannot begin until the database schema and models are in place. This task runs in parallel with Task 5 and blocks Task 7 (Integration Tests).

**Dependency Chain**: Task 1 → Task 2 → Task 7

## Technical Specifications

### Framework and Dependencies
- **Web Framework**: Actix-web 4.3.1
- **Serialization**: serde 1.0 with derive features
- **JSON**: serde_json 1.0
- **Server Binding**: 127.0.0.1:8080
- **Runtime**: Actix-web async runtime

### API Structure
```
/api
├── /health (GET) - Health check endpoint
├── /users (scope) - User-related endpoints (placeholder for Task 3)
├── /products (scope) - Product catalog endpoints (placeholder for Task 4)
└── /cart (scope) - Shopping cart endpoints (placeholder for Task 5)
```

### Route Organization
- **Scoped routing**: All routes under `/api` prefix
- **Modular configuration**: Separate configuration functions for each resource
- **Placeholder implementations**: Return 501 Not Implemented for future routes
- **Health check**: Immediate implementation for system monitoring

## Implementation Plan

### Step 1: Add Actix-web Dependencies
**File**: `Cargo.toml`

Add web framework dependencies:
```toml
[dependencies]
actix-web = "4.3.1"
actix-rt = "2.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Ensure these exist from Task 1
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
```

### Step 2: Create API Module Structure
**File**: `src/api/mod.rs`

Export API submodules:
```rust
pub mod routes;
pub mod errors;

pub use self::routes::configure_routes;
```

### Step 3: Implement Route Configuration
**File**: `src/api/routes.rs`

```rust
use actix_web::{web, HttpResponse, HttpRequest, Result};
use serde_json::json;

/// Main route configuration function
/// Registers all API endpoints under /api scope
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes))
            .service(web::scope("/cart").configure(cart_routes))
    );
}

/// Health check endpoint for monitoring
/// GET /api/health
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "e-commerce-api",
        "version": "0.1.0"
    }))
}

/// User routes configuration (Task 3)
/// Placeholder for authentication and user management endpoints
fn user_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 3 (User Authentication)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented))
    );
}

/// Product routes configuration (Task 4)
/// Placeholder for product catalog endpoints
fn product_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 4 (Product Catalog)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented))
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(not_implemented))
    );
}

/// Cart routes configuration (Task 5)
/// Placeholder for shopping cart endpoints
fn cart_routes(cfg: &mut web::ServiceConfig) {
    // Routes will be implemented in Task 5 (Shopping Cart)
    cfg.service(
        web::resource("")
            .route(web::get().to(not_implemented))
            .route(web::post().to(not_implemented))
    );
}

/// Placeholder handler for not-yet-implemented endpoints
async fn not_implemented() -> HttpResponse {
    HttpResponse::NotImplemented().json(json!({
        "error": "This endpoint is not yet implemented",
        "message": "This functionality will be added in a future task"
    }))
}
```

### Step 4: Implement Error Handling
**File**: `src/api/errors.rs`

```rust
use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    Unauthorized(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ApiError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            status: status.as_u16(),
        })
    }
}
```

### Step 5: Create Main Application Entry Point
**File**: `src/main.rs`

```rust
use actix_web::{App, HttpServer, middleware};
use dotenv::dotenv;

mod api;
mod config;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting E-Commerce API Server");
    println!("📡 Server will listen on http://127.0.0.1:8080");
    println!("🏥 Health check available at http://127.0.0.1:8080/api/health");

    HttpServer::new(|| {
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Configure routes
            .configure(api::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Step 6: Add Logging Support
**File**: `Cargo.toml` (additional dependency)

```toml
[dependencies]
env_logger = "0.10"
log = "0.4"
```

### Step 7: Update Module Exports
Ensure `src/api/mod.rs` properly exports the errors module:
```rust
pub mod routes;
pub mod errors;

pub use self::routes::configure_routes;
pub use self::errors::{ApiError, ErrorResponse};
```

## Architectural Considerations

### Design Decisions
1. **Scoped Routing**: Using `web::scope()` for clean URL structure and logical grouping
2. **Modular Configuration**: Separate configuration functions allow parallel development of features
3. **Placeholder Pattern**: Not-yet-implemented endpoints return 501 status for clear communication
4. **Error Handling**: Centralized error types with consistent JSON responses
5. **Middleware**: Logger middleware for request/response monitoring

### Route Hierarchy
```
HttpServer
  └─ App
       ├─ Logger Middleware
       └─ /api Scope
            ├─ /health (GET) → Implemented
            ├─ /users Scope → Placeholder (Task 3)
            ├─ /products Scope → Placeholder (Task 4)
            └─ /cart Scope → Placeholder (Task 5)
```

### HTTP Status Codes
- **200 OK**: Successful requests (health check)
- **400 Bad Request**: Invalid request data
- **401 Unauthorized**: Missing or invalid authentication
- **404 Not Found**: Resource doesn't exist
- **500 Internal Server Error**: Server-side errors
- **501 Not Implemented**: Placeholder endpoints

### Integration Points
- **Task 1 (Database Schema)**: Imports schema and models modules
- **Task 3 (User Authentication)**: Will implement `/api/users` routes
- **Task 4 (Product Catalog)**: Will implement `/api/products` routes
- **Task 5 (Shopping Cart)**: Will implement `/api/cart` routes

## Dependencies
- **Task 1: Database Schema Setup** - Required before this task can begin

## Dependent Tasks
- **Task 5: Shopping Cart API** - Depends on this API structure
- **Task 7: Integration Tests** - Will test these endpoints

## Risks and Mitigation

### Risk: Port Already in Use
**Mitigation**: Configure port via environment variable; document how to change binding address.

### Risk: Missing Database Connection
**Mitigation**: Defer database connection pool integration to Task 5; health check doesn't require DB.

### Risk: Route Conflicts
**Mitigation**: Use clear scoping and document route structure; follow REST conventions.

## Testing Strategy
Detailed in `acceptance-criteria.md`. Key validation points:
- Server starts without errors
- Health check endpoint returns 200 OK
- Placeholder endpoints return 501 Not Implemented
- Logger middleware captures requests
- Error responses follow consistent format

## Validation Commands

```bash
# Build and check compilation
cargo build
cargo check

# Run the server
cargo run

# In another terminal, test endpoints
curl http://localhost:8080/api/health
# Expected: {"status":"ok","service":"e-commerce-api","version":"0.1.0"}

curl http://localhost:8080/api/users
# Expected: 501 Not Implemented

curl http://localhost:8080/api/products
# Expected: 501 Not Implemented

# Check logging output
# Should see request logs in server console
```

## References
- [Actix-web Documentation](https://actix.rs/)
- Architecture Document: `.taskmaster/docs/architecture.md` (API Endpoints section, lines 182-199)
- PRD: `.taskmaster/docs/prd.txt` (Task 2 specification, lines 41-47)
- Task 1 Documentation: `.taskmaster/docs/task-1/task.md`

## Estimated Effort
50 minutes (as per PRD)
