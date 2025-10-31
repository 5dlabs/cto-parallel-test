# Task 2: API Endpoints

## Overview
Implement the REST API routing infrastructure using Actix-web 4.3.1, establishing the HTTP server and endpoint structure for the e-commerce application.

## Context
- **Execution Level**: 1 (depends on Task 1 - Database Schema)
- **Priority**: Medium
- **Estimated Time**: 50 minutes
- **Dependencies**: Task 1 (needs schema for future integration)
- **Dependents**: Task 5 (Shopping Cart API), Task 7 (Integration Tests)

## Objectives
1. Set up Actix-web HTTP server on port 8080
2. Implement route configuration with scoped endpoints
3. Create health check endpoint
4. Establish placeholder routes for users and products
5. Configure error handling structure

## Technical Specifications

### API Server Configuration
- **Framework**: Actix-web 4.3.1
- **Bind Address**: 127.0.0.1:8080
- **Route Prefix**: /api
- **Response Format**: JSON

### Endpoint Structure
```
/api
├── /health (GET) - Health check endpoint
├── /users (scope) - User management routes (placeholder)
└── /products (scope) - Product management routes (placeholder)
```

### Route Architecture Diagram
See `diagrams.mmd` for visual representation of the routing flow.

## Implementation Plan

### Step 1: Add Actix-web Dependencies
Update `Cargo.toml`:
```toml
[dependencies]
actix-web = "4.3.1"
actix-rt = "2.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Step 2: Create API Module Structure
Create `src/api/mod.rs`:
```rust
pub mod routes;
pub mod errors;  // For future error handling
```

### Step 3: Implement Route Configuration
Create `src/api/routes.rs`:
```rust
use actix_web::{web, HttpResponse};
use serde_json::json;

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
    HttpResponse::Ok().json(json!({"status": "ok", "message": "API is running"}))
}

fn user_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder for Task 3
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() }))
    );
}

fn product_routes(cfg: &mut web::ServiceConfig) {
    // Placeholder for Task 4
    cfg.service(
        web::resource("")
            .route(web::get().to(|| async { HttpResponse::NotImplemented().finish() }))
    );
}
```

### Step 4: Create Main Application Entry Point
Create or update `src/main.rs`:
```rust
use actix_web::{App, HttpServer, middleware};

mod api;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting API server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(api::routes::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Step 5: Create Error Handling Module (Foundation)
Create `src/api/errors.rs`:
```rust
use actix_web::{error::ResponseError, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            ApiError::InternalServerError(msg) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal_server_error",
                "message": msg
            })),
        }
    }
}
```

## Architecture Considerations

### Scoped Routes
Routes are organized into logical scopes (`/users`, `/products`, `/cart`) for:
- **Modularity**: Each resource has its own namespace
- **Middleware**: Can apply scope-specific middleware later
- **Maintainability**: Easy to add new routes within each scope

### Placeholder Pattern
Placeholder routes return HTTP 501 (Not Implemented) to:
- Allow server to compile and run
- Provide clear indication of future functionality
- Enable parallel development of other modules

### Async Handlers
All route handlers use async/await for:
- Non-blocking I/O operations
- Better concurrency under load
- Future database integration

## Testing Strategy

### Manual Testing
```bash
# Start the server
cargo run

# Test health check
curl http://localhost:8080/api/health

# Expected response
{"status":"ok","message":"API is running"}

# Test placeholder routes
curl http://localhost:8080/api/users
# Expected: HTTP 501 Not Implemented

curl http://localhost:8080/api/products
# Expected: HTTP 501 Not Implemented
```

### Integration Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

## Risks and Mitigation

### Risk: Port Already in Use
**Mitigation**: Check for existing processes on port 8080 before starting

### Risk: Missing Module Exports
**Mitigation**: Ensure all modules properly exported in lib.rs/main.rs

### Risk: Async Runtime Issues
**Mitigation**: Use `#[actix_web::main]` macro for proper runtime initialization

## Completion Criteria
✅ Actix-web server starts on port 8080
✅ Health check endpoint returns 200 OK with JSON
✅ User and product route scopes registered
✅ Placeholder handlers return 501 Not Implemented
✅ Error handling module created
✅ Project compiles and runs without errors
✅ Server logs requests with middleware logger

## References
- [Actix-web Documentation](https://actix.rs/docs/)
- [Actix-web Application Guide](https://actix.rs/docs/application/)
- [HTTP Status Codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
