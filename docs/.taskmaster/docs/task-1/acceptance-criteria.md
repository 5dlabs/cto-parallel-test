# Acceptance Criteria: Task 1 - Database Schema Setup

## File Creation Requirements

### ✅ Required Files Must Exist
- [ ] `Cargo.toml` (updated with database dependencies)
- [ ] `src/config/db.rs` (database connection configuration)
- [ ] `src/schema.rs` (Diesel schema definitions)
- [ ] `src/models.rs` (ORM model structs)
- [ ] `migrations/TIMESTAMP_create_users/up.sql`
- [ ] `migrations/TIMESTAMP_create_users/down.sql`
- [ ] `migrations/TIMESTAMP_create_products/up.sql`
- [ ] `migrations/TIMESTAMP_create_products/down.sql`
- [ ] `migrations/TIMESTAMP_create_carts/up.sql`
- [ ] `migrations/TIMESTAMP_create_carts/down.sql`
- [ ] `migrations/TIMESTAMP_create_cart_items/up.sql`
- [ ] `migrations/TIMESTAMP_create_cart_items/down.sql`
- [ ] `.env.example` (database URL template)

## Dependency Requirements

### ✅ Cargo.toml Must Include
```bash
# Verify with: cargo tree | grep -E "(diesel|r2d2|dotenv|chrono)"
```
- [ ] diesel = "2.1.0" with features ["postgres", "r2d2", "chrono"]
- [ ] r2d2 = "0.8.10"
- [ ] dotenv = "0.15.0"
- [ ] chrono with serde feature
- [ ] serde with derive feature
- [ ] serde_json

## Schema Requirements

### ✅ src/schema.rs Must Define
- [ ] `users` table with columns: id, username, email, password_hash, created_at
- [ ] `products` table with columns: id, name, description, price, inventory_count
- [ ] `carts` table with columns: id, user_id, created_at
- [ ] `cart_items` table with columns: id, cart_id, product_id, quantity
- [ ] `joinable!` macro for cart_items -> carts relationship
- [ ] `joinable!` macro for cart_items -> products relationship
- [ ] `joinable!` macro for carts -> users relationship
- [ ] `allow_tables_to_appear_in_same_query!` macro including all tables

**Validation Command:**
```bash
grep -E "(table!|joinable!|allow_tables_to_appear)" src/schema.rs
```

## Model Requirements

### ✅ src/models.rs Must Implement
- [ ] `User` struct with Queryable, Identifiable, Serialize, Deserialize traits
- [ ] `NewUser` struct with Insertable, Deserialize traits
- [ ] `Product` struct with Queryable, Identifiable, Serialize, Deserialize traits
- [ ] `NewProduct` struct with Insertable, Deserialize traits
- [ ] `Cart` struct with Queryable, Identifiable, Associations, Serialize, Deserialize traits
- [ ] `NewCart` struct with Insertable trait
- [ ] `CartItem` struct with Queryable, Identifiable, Associations, Serialize, Deserialize traits
- [ ] `NewCartItem` struct with Insertable trait
- [ ] Proper `#[belongs_to]` annotations for associations

**Validation Command:**
```bash
grep -E "(struct User|struct NewUser|struct Product|struct Cart|struct CartItem)" src/models.rs
```

## Migration Requirements

### ✅ Users Migration (up.sql)
- [ ] Creates `users` table with SERIAL PRIMARY KEY
- [ ] username VARCHAR with UNIQUE constraint
- [ ] email VARCHAR with UNIQUE constraint
- [ ] password_hash VARCHAR NOT NULL
- [ ] created_at TIMESTAMP with DEFAULT CURRENT_TIMESTAMP

### ✅ Products Migration (up.sql)
- [ ] Creates `products` table with SERIAL PRIMARY KEY
- [ ] name VARCHAR NOT NULL
- [ ] description TEXT
- [ ] price NUMERIC NOT NULL
- [ ] inventory_count INTEGER NOT NULL

### ✅ Carts Migration (up.sql)
- [ ] Creates `carts` table with SERIAL PRIMARY KEY
- [ ] user_id INTEGER with FOREIGN KEY to users(id)
- [ ] created_at TIMESTAMP with DEFAULT CURRENT_TIMESTAMP

### ✅ Cart Items Migration (up.sql)
- [ ] Creates `cart_items` table with SERIAL PRIMARY KEY
- [ ] cart_id INTEGER with FOREIGN KEY to carts(id)
- [ ] product_id INTEGER with FOREIGN KEY to products(id)
- [ ] quantity INTEGER NOT NULL

### ✅ All Migrations (down.sql)
- [ ] Each down.sql drops its corresponding table
- [ ] Drop statements are in reverse dependency order

**Validation Command:**
```bash
diesel migration run && diesel migration redo
```

## Database Configuration Requirements

### ✅ src/config/db.rs Must Provide
- [ ] `Pool` type alias for `r2d2::Pool<ConnectionManager<PgConnection>>`
- [ ] `DbConnection` type alias for pooled connection
- [ ] `establish_connection_pool()` function that:
  - [ ] Loads environment variables with dotenv
  - [ ] Reads DATABASE_URL from environment
  - [ ] Creates ConnectionManager with database URL
  - [ ] Builds and returns connection pool
  - [ ] Includes proper error handling with expect/unwrap messages

**Validation Command:**
```bash
grep -E "(pub type Pool|pub fn establish_connection_pool)" src/config/db.rs
```

## Build and Compilation Requirements

### ✅ Compilation Must Succeed
```bash
cargo check
# Expected: No errors, project compiles successfully
```
- [ ] No compilation errors
- [ ] No warnings (or only acceptable warnings)
- [ ] All dependencies resolve correctly

### ✅ Migration Application Must Succeed
```bash
diesel migration run
# Expected: All migrations apply successfully
```
- [ ] Migrations execute without errors
- [ ] All tables created in database
- [ ] Foreign key constraints properly established

### ✅ Migration Rollback Must Succeed
```bash
diesel migration redo
# Expected: Down migration works, then up migration reapplies
```
- [ ] Down migrations drop tables successfully
- [ ] Up migrations recreate tables successfully

## Module Integration Requirements

### ✅ Module Exports
Verify in `src/lib.rs` or `src/main.rs`:
- [ ] `pub mod config;` (if using separate config module)
- [ ] `pub mod models;`
- [ ] `pub mod schema;`

## Code Quality Requirements

### ✅ Code Style
- [ ] Follows Rust naming conventions (snake_case for functions/variables)
- [ ] Proper use of pub/private visibility
- [ ] Includes necessary use/import statements
- [ ] No unused imports or variables

### ✅ Documentation
- [ ] Key structs have doc comments (optional but recommended)
- [ ] Foreign key relationships are clear from code structure

## Testing Requirements

### ✅ Manual Tests Must Pass
1. **Database Connection Test**:
   ```bash
   # With valid DATABASE_URL in .env
   cargo test --lib
   ```
   - [ ] Connection pool can be established

2. **Schema Validation**:
   ```bash
   diesel database reset
   ```
   - [ ] Database can be dropped and recreated
   - [ ] All migrations reapply successfully

3. **Model Instantiation**:
   - [ ] Can create NewUser struct instances
   - [ ] Can create NewProduct struct instances
   - [ ] Structs serialize to JSON correctly

## Environment Configuration

### ✅ .env.example Must Contain
```bash
cat .env.example
```
- [ ] DATABASE_URL with PostgreSQL connection string format
- [ ] Example values that clearly indicate placeholders

## Downstream Dependencies

### ✅ Task 2 Integration Readiness
- [ ] schema.rs is importable by other modules
- [ ] models.rs exports all necessary structs
- [ ] Connection pool can be shared across application
- [ ] No blocking issues for API endpoint implementation

## Final Validation Checklist

Run this complete validation sequence:
```bash
# 1. Check dependencies
cargo tree | grep diesel

# 2. Compile check
cargo check

# 3. Test migrations
diesel migration run
diesel migration redo

# 4. Database reset
diesel database reset

# 5. Verify schema
psql $DATABASE_URL -c "\dt"

# 6. Check foreign keys
psql $DATABASE_URL -c "SELECT conname FROM pg_constraint WHERE contype = 'f';"
```

Expected Results:
- [ ] All commands execute without errors
- [ ] All tables present: users, products, carts, cart_items
- [ ] All foreign keys established: cart_items_cart_id_fkey, cart_items_product_id_fkey, carts_user_id_fkey

## Definition of Done

**This task is considered complete when:**
1. All required files exist and contain proper implementations
2. `cargo check` completes without errors
3. All migrations apply and rollback successfully
4. Database schema matches specifications exactly
5. Model structs can be instantiated and serialized
6. No blocking issues for dependent tasks (Task 2)
7. Code follows Rust and Diesel best practices

**Rejection Criteria:**
- Missing any required file
- Compilation errors
- Migration failures
- Incorrect table schema or relationships
- Missing foreign key constraints
- Improperly configured traits on model structs
