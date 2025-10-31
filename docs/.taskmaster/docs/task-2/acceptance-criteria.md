# Acceptance Criteria: Task 2 - API Endpoints

## Completion Criteria

### 1. Dependencies Configuration
- [ ] Cargo.toml includes actix-web version 4.3.1
- [ ] Cargo.toml includes actix-rt version 2.8
- [ ] Cargo.toml includes serde with derive features
- [ ] Cargo.toml includes serde_json
- [ ] Cargo.toml includes env_logger and log
- [ ] cargo build completes without dependency errors

### 2. API Module Structure
- [ ] Directory `src/api/` exists
- [ ] File `src/api/mod.rs` exists and exports routes and errors
- [ ] File `src/api/routes.rs` exists with route configuration
- [ ] File `src/api/errors.rs` exists with error types
- [ ] All API modules are public

### 3. Route Configuration
- [ ] `configure_routes` function implemented in routes.rs
- [ ] Routes scoped under `/api` prefix
- [ ] Health check route registered at `/api/health`
- [ ] User routes scope created at `/api/users`
- [ ] Product routes scope created at `/api/products`
- [ ] Cart routes scope created at `/api/cart`
- [ ] Placeholder route configurations implemented for users, products, cart
- [ ] `not_implemented` handler returns 501 status code

### 4. Health Check Endpoint
- [ ] Health check endpoint responds to GET requests
- [ ] Returns HTTP 200 OK status
- [ ] Returns JSON with "status" field set to "ok"
- [ ] Returns JSON with "service" field
- [ ] Returns JSON with "version" field
- [ ] Content-Type header is application/json

### 5. Error Handling
- [ ] `ErrorResponse` struct defined with error, message, status fields
- [ ] `ApiError` enum defined with NotFound, BadRequest, InternalError, Unauthorized variants
- [ ] `Display` trait implemented for ApiError
- [ ] `ResponseError` trait implemented for ApiError
- [ ] Error responses return appropriate HTTP status codes
- [ ] Error responses return JSON format

### 6. Main Application Setup
- [ ] src/main.rs updated with actix_web imports
- [ ] dotenv().ok() called to load environment variables
- [ ] env_logger initialized
- [ ] HttpServer created with App configuration
- [ ] Logger middleware added to App
- [ ] configure_routes called in App setup
- [ ] Server binds to 127.0.0.1:8080
- [ ] Startup messages printed to console
- [ ] Main function is async with #[actix_web::main]

### 7. Module Integration
- [ ] src/main.rs imports api module
- [ ] src/main.rs imports config, models, schema modules from Task 1
- [ ] No circular dependencies exist
- [ ] All module paths resolve correctly

### 8. Logging
- [ ] Logger middleware configured in App
- [ ] Request logs appear in console when endpoints accessed
- [ ] Log format includes timestamp, method, path, status, duration
- [ ] Startup messages include server URL and health check URL

### 9. Server Functionality
- [ ] Server starts without panics or errors
- [ ] Server binds to specified address and port
- [ ] Server handles concurrent requests
- [ ] Server can be stopped with Ctrl+C

### 10. Endpoint Testing
- [ ] Health check endpoint accessible via curl/browser
- [ ] Health check returns expected JSON structure
- [ ] Placeholder user routes return 501 Not Implemented
- [ ] Placeholder product routes return 501 Not Implemented
- [ ] Placeholder cart routes return 501 Not Implemented
- [ ] Invalid routes return 404 Not Found

## Validation Commands

```bash
# 1. Build and check
cargo build
cargo check
cargo clippy

# 2. Start server
cargo run
# Expected output:
# 🚀 Starting E-Commerce API Server
# 📡 Server will listen on http://127.0.0.1:8080
# 🏥 Health check available at http://127.0.0.1:8080/api/health

# 3. Test health check (in new terminal)
curl http://localhost:8080/api/health
# Expected: {"status":"ok","service":"e-commerce-api","version":"0.1.0"}

# 4. Test placeholder endpoints
curl http://localhost:8080/api/users
# Expected: 501 status with error message

curl http://localhost:8080/api/products
# Expected: 501 status with error message

curl http://localhost:8080/api/cart
# Expected: 501 status with error message

# 5. Test invalid route
curl http://localhost:8080/api/invalid
# Expected: 404 Not Found

# 6. Verify logging
# Check server console for request logs
# Should see: "GET /api/health HTTP/1.1" 200 ...

# 7. Performance check
ab -n 100 -c 10 http://localhost:8080/api/health
# Server should handle concurrent requests

# 8. JSON validation
curl -i http://localhost:8080/api/health | grep "Content-Type"
# Expected: Content-Type: application/json
```

## Success Indicators

### Must Have (Blocking)
1. Server starts successfully on port 8080
2. Health check endpoint returns 200 OK with correct JSON
3. Placeholder routes return 501 Not Implemented
4. Logging middleware active and writing logs
5. No compilation errors or warnings
6. All route scopes properly configured

### Should Have (Important)
1. Error handling follows standardized format
2. Startup messages are clear and informative
3. Server handles graceful shutdown
4. Concurrent requests handled correctly

### Nice to Have (Optional)
1. Detailed API documentation comments
2. Request validation middleware
3. CORS configuration for frontend integration
4. Rate limiting middleware

## Rejection Criteria

Task will be considered incomplete if:
- ❌ Server fails to start
- ❌ Health check endpoint doesn't respond
- ❌ Health check returns incorrect JSON structure
- ❌ Placeholder routes return wrong status codes
- ❌ Compilation errors exist
- ❌ Missing required middleware
- ❌ Routes not properly scoped under /api

## Definition of Done

Task is complete when:
1. ✅ All "Must Have" criteria met
2. ✅ All validation commands execute successfully
3. ✅ Server tested with curl/Postman
4. ✅ Logging confirmed working
5. ✅ Code reviewed for best practices
6. ✅ Ready for Tasks 5 and 7 to build on this foundation

## Integration Points

### With Task 1 (Database Schema)
- [ ] Imports schema module without errors
- [ ] Imports models module without errors
- [ ] Imports config module without errors
- [ ] Database connection not required yet (deferred to Task 5)

### For Task 3 (User Authentication)
- [ ] `/api/users` scope ready for route implementation
- [ ] Error handling types available for auth errors

### For Task 4 (Product Catalog)
- [ ] `/api/products` scope ready for route implementation
- [ ] Error handling types available for product errors

### For Task 5 (Shopping Cart)
- [ ] `/api/cart` scope ready for route implementation
- [ ] Server configuration supports cart endpoints

### For Task 7 (Integration Tests)
- [ ] Health check endpoint testable
- [ ] Server can be initialized in test context
- [ ] Routes accessible from test client
