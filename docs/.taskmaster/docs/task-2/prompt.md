# Autonomous Agent Prompt: API Endpoints Setup

## Mission
Implement the RESTful API routing infrastructure using Actix-web to serve as the HTTP backbone for the e-commerce application.

## Task Context
- **Task ID**: 2
- **Priority**: Medium
- **Execution Level**: 1 (depends on Task 1)
- **Estimated Time**: 50 minutes

## Objective
Create a fully functional Actix-web HTTP server with health check endpoint and placeholder route structure for users, products, and cart resources.

## Required Deliverables

### 1. Cargo.toml Updates
Add web framework dependencies:
```toml
actix-web = "4.3.1"
actix-rt = "2.8"
env_logger = "0.10"
```

### 2. src/api/mod.rs
Module exports file

### 3. src/api/routes.rs
Route configuration with:
- `configure_routes()` function
- Health check endpoint at GET /api/health
- User routes scope (placeholder)
- Product routes scope (placeholder)

### 4. src/api/errors.rs
Error handling module with ApiError enum

### 5. src/main.rs
Application entry point with:
- HttpServer configuration
- Logger middleware
- Route registration
- Port binding (127.0.0.1:8080)

## Implementation Steps

1. **Update Dependencies**: Add Actix-web to Cargo.toml
2. **Create API Module**: Create src/api/ directory with mod.rs
3. **Implement Routes**: Create routes.rs with scoped configuration
4. **Add Error Handling**: Create errors.rs with basic error types
5. **Update Main**: Configure HttpServer in main.rs
6. **Test Server**: Run and verify health check responds correctly

## Success Criteria
- Server starts without errors on port 8080
- GET /api/health returns JSON: `{"status":"ok","message":"API is running"}`
- Placeholder routes return HTTP 501 Not Implemented
- Logger middleware outputs request information
- All endpoints use async handlers

## Testing Commands
```bash
# Build and run
cargo run

# In another terminal, test health check
curl http://localhost:8080/api/health

# Should return: {"status":"ok","message":"API is running"}
```

## Quality Standards
- Use async/await for all handlers
- Implement proper HTTP status codes
- Include logging middleware
- Return JSON responses
- Follow Actix-web best practices

Execute autonomously following Actix-web conventions.
