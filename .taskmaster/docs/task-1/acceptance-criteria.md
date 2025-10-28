# Task 1: Database Schema Setup - Acceptance Criteria

## File Creation Criteria

### ✅ Required Files Exist
- [ ] `src/schema.rs` exists
- [ ] `migrations/` directory exists
- [ ] Migration file `migrations/00000000000001_create_tables/up.sql` exists
- [ ] Migration file `migrations/00000000000001_create_tables/down.sql` exists
- [ ] `Cargo.toml` has been updated

## Code Quality Criteria

### ✅ Schema Definitions (src/schema.rs)
- [ ] File contains valid Rust syntax
- [ ] Uses Diesel's `table!` macro correctly
- [ ] Defines `users` table with all required columns (id, username, email, password_hash, created_at)
- [ ] Defines `products` table with all required columns (id, name, description, price, inventory_count)
- [ ] Defines `carts` table with all required columns (id, user_id, created_at)
- [ ] Defines `cart_items` table with all required columns (id, cart_id, product_id, quantity)
- [ ] Uses appropriate Diesel column types (Integer, Varchar, Text, Numeric, Timestamp)
- [ ] Each table specifies its primary key correctly

### ✅ Migration Files (migrations/*/up.sql)
- [ ] Contains valid PostgreSQL SQL syntax
- [ ] Creates all four tables (users, products, carts, cart_items)
- [ ] Uses SERIAL for auto-incrementing primary keys
- [ ] Includes proper data types matching schema.rs
- [ ] Defines foreign key constraints:
  - `carts.user_id` → `users.id`
  - `cart_items.cart_id` → `carts.id` with ON DELETE CASCADE
  - `cart_items.product_id` → `products.id`
- [ ] Includes UNIQUE constraints on `users.username` and `users.email`
- [ ] Includes UNIQUE constraint on `cart_items(cart_id, product_id)`
- [ ] Sets appropriate DEFAULT values (timestamps, inventory_count)

### ✅ Migration Rollback (migrations/*/down.sql)
- [ ] Contains DROP TABLE statements for all four tables
- [ ] Drops tables in correct order (respecting foreign key constraints)
- [ ] Uses `DROP TABLE IF EXISTS` for safety

### ✅ Dependencies (Cargo.toml)
- [ ] Includes `diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }`
- [ ] Includes `r2d2 = "0.8.10"`
- [ ] Includes `dotenv = "0.15.0"`
- [ ] Dependencies are in the `[dependencies]` section
- [ ] TOML syntax is valid

## Compilation and Validation Criteria

### ✅ Build Verification
- [ ] `cargo check` completes without errors
- [ ] No syntax errors in schema.rs
- [ ] Dependencies resolve correctly

### ✅ SQL Validation
- [ ] `up.sql` contains valid PostgreSQL syntax
- [ ] `down.sql` contains valid PostgreSQL syntax
- [ ] Table creation order respects dependencies (parent before child)
- [ ] Foreign key references exist before being used

## Integration Criteria

### ✅ Compatibility with Dependent Tasks
- [ ] Schema structure supports Task 2 (API Endpoints) requirements
- [ ] Users table supports Task 3 (User Authentication) requirements
- [ ] Products table supports Task 4 (Product Catalog) requirements
- [ ] Carts tables support Task 5 (Shopping Cart) requirements

### ✅ File Conflict Management
- [ ] `Cargo.toml` modifications are compatible with Task 2's changes
- [ ] No merge conflicts in generated files
- [ ] Standard formatting used for maintainability

## Testing Commands

### Manual Validation Steps

1. **Verify File Existence**
   ```bash
   ls -la src/schema.rs
   ls -la migrations/
   ```

2. **Check Rust Compilation**
   ```bash
   cargo check
   ```

3. **Validate Dependencies**
   ```bash
   cargo tree | grep diesel
   cargo tree | grep r2d2
   cargo tree | grep dotenv
   ```

4. **Syntax Check SQL Files**
   ```bash
   cat migrations/*/up.sql
   cat migrations/*/down.sql
   ```

5. **Verify Schema Structure**
   ```bash
   grep -c "table!" src/schema.rs  # Should output: 4
   ```

## Success Definition

**Task is COMPLETE when:**
1. All required files exist at specified paths
2. All code quality criteria are met
3. `cargo check` passes without errors
4. SQL files contain valid PostgreSQL syntax
5. Schema supports all dependent tasks' requirements

**Task is INCOMPLETE if:**
- Any required file is missing
- Compilation errors exist
- SQL syntax is invalid
- Table definitions are incomplete
- Dependencies are incorrectly specified

## Estimated Completion Time
30 minutes (as specified in PRD)

## Dependencies
None - This is a Level 0 task

## Blocks
- Task 2: API Endpoints (depends on this task completing)
