# Acceptance Criteria: Task 1 - Database Schema Setup

## Completion Criteria

### 1. Dependencies Configuration
- [ ] `Cargo.toml` includes diesel with postgres, r2d2, and chrono features
- [ ] `Cargo.toml` includes r2d2 version 0.8.10
- [ ] `Cargo.toml` includes dotenv version 0.15.0
- [ ] `Cargo.toml` includes chrono with serde features
- [ ] `Cargo.toml` includes serde with derive features
- [ ] `Cargo.toml` includes serde_json
- [ ] All dependencies resolve successfully: `cargo build` completes without dependency errors

### 2. Database Configuration Module
- [ ] File `src/config/db.rs` exists
- [ ] File `src/config/mod.rs` exists and exports db module
- [ ] `establish_connection_pool()` function implemented
- [ ] Function returns `Pool` type alias
- [ ] Function reads DATABASE_URL from environment
- [ ] Function panics with clear error message if DATABASE_URL not set
- [ ] Pool uses r2d2::ConnectionManager<PgConnection>
- [ ] Type aliases `Pool` and `DbConnection` are exported

### 3. Schema Definitions
- [ ] File `src/schema.rs` exists
- [ ] Schema defines `users` table with all required fields (id, username, email, password_hash, created_at)
- [ ] Schema defines `products` table with all required fields (id, name, description, price, inventory_count)
- [ ] Schema defines `carts` table with all required fields (id, user_id, created_at)
- [ ] Schema defines `cart_items` table with all required fields (id, cart_id, product_id, quantity)
- [ ] Foreign key relationships defined using `joinable!` macro
- [ ] `allow_tables_to_appear_in_same_query!` macro includes all tables
- [ ] Schema compiles without errors: `cargo check` passes

### 4. Database Migrations
- [ ] Migrations directory exists at project root
- [ ] Migration for `create_users` exists with up.sql and down.sql
- [ ] Migration for `create_products` exists with up.sql and down.sql
- [ ] Migration for `create_carts` exists with up.sql and down.sql
- [ ] Migration for `create_cart_items` exists with up.sql and down.sql
- [ ] Users table up.sql creates table with PRIMARY KEY, UNIQUE constraints on username/email
- [ ] Products table up.sql creates table with NOT NULL constraints
- [ ] Carts table up.sql creates table with FOREIGN KEY to users
- [ ] Cart_items table up.sql creates table with FOREIGN KEYs to carts and products
- [ ] All down.sql files properly drop tables
- [ ] Migrations apply successfully: `diesel migration run` completes
- [ ] Migrations rollback successfully: `diesel migration redo` completes

### 5. Model Structs
- [ ] File `src/models.rs` exists
- [ ] `User` struct defined with Queryable, Identifiable, Serialize, Deserialize derives
- [ ] `NewUser` struct defined with Insertable, Deserialize derives
- [ ] `Product` struct defined with Queryable, Identifiable, Serialize, Deserialize derives
- [ ] `NewProduct` struct defined with Insertable, Deserialize derives
- [ ] `Cart` struct defined with Queryable, Identifiable, Associations, Serialize, Deserialize derives
- [ ] `NewCart` struct defined with Insertable derive
- [ ] `CartItem` struct defined with Queryable, Identifiable, Associations, Serialize, Deserialize derives
- [ ] `NewCartItem` struct defined with Insertable derive
- [ ] All structs use correct field types matching schema
- [ ] Timestamp fields use chrono::NaiveDateTime
- [ ] All models compile without errors

### 6. Environment Configuration
- [ ] `.env` file exists in project root
- [ ] `.env` contains DATABASE_URL variable
- [ ] `.env` is added to .gitignore (or .env.example provided)

### 7. Module Integration
- [ ] `src/lib.rs` or `src/main.rs` declares config module
- [ ] `src/lib.rs` or `src/main.rs` declares models module
- [ ] `src/lib.rs` or `src/main.rs` declares schema module
- [ ] All modules are public

### 8. Build and Compilation
- [ ] Project builds successfully: `cargo build` completes
- [ ] Project passes check: `cargo check` passes without warnings
- [ ] No compilation errors in any module
- [ ] No missing imports or unresolved references

### 9. Testing
- [ ] Simple test exists that creates a connection pool
- [ ] Test verifies connection pool can be created with valid DATABASE_URL
- [ ] Test passes: `cargo test` succeeds

### 10. Documentation
- [ ] README or inline comments explain how to set up DATABASE_URL
- [ ] Migration files include comments explaining table purpose
- [ ] Schema relationships documented in comments

## Validation Commands

Run these commands to verify acceptance criteria:

```bash
# 1. Check dependencies resolve
cargo build

# 2. Verify code compiles without warnings
cargo check

# 3. Verify migrations apply
diesel migration run

# 4. Test migration rollback
diesel migration redo

# 5. Run tests
cargo test

# 6. Verify schema structure
diesel print-schema

# 7. Check for compilation errors
cargo clippy

# 8. Verify connection pool can be created
cargo run --bin check-db-connection  # If such binary exists
```

## Success Indicators

### Must Have (Blocking)
1. All 4 migration pairs created and apply successfully
2. All 8 model structs (4 Queryable + 4 Insertable) defined correctly
3. Schema.rs compiles without errors
4. Connection pool can be established with valid DATABASE_URL
5. `cargo check` passes with zero errors

### Should Have (Important)
1. No compiler warnings
2. Proper foreign key constraints in migrations
3. Unique constraints on username and email
4. Proper error messages if DATABASE_URL missing

### Nice to Have (Optional)
1. Database connection test in test suite
2. Example .env.example file
3. Inline documentation on model structs

## Rejection Criteria

The task will be considered incomplete if:
- ❌ Any migration fails to apply
- ❌ Schema.rs has compilation errors
- ❌ Model structs missing required derives
- ❌ Foreign key relationships not properly defined
- ❌ Connection pool cannot be created
- ❌ `cargo check` fails
- ❌ Dependencies missing or incorrect versions

## Definition of Done

Task is complete when:
1. ✅ All "Must Have" criteria met
2. ✅ All validation commands execute successfully
3. ✅ Code reviewed for best practices
4. ✅ No blocking issues remain
5. ✅ Ready for Task 2 (API Endpoints) to begin
