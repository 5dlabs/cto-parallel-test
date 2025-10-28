# Task 1: Database Schema Setup - Agent Prompt

You are a Rust backend developer tasked with creating database schema files for a test e-commerce API project.

## Your Mission
Create basic database schema files and configuration using Diesel ORM. This is part of a parallel execution test, so focus on creating correct, complete file structures rather than production-ready implementations.

## What You Must Create

### 1. Update `Cargo.toml`
Add these dependencies to the `[dependencies]` section:
```toml
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
```

### 2. Create `src/schema.rs`
Define these four tables using Diesel's `table!` macro:
- **users**: id, username, email, password_hash, created_at
- **products**: id, name, description, price, inventory_count
- **carts**: id, user_id, created_at
- **cart_items**: id, cart_id, product_id, quantity

Use appropriate Diesel column types (Integer, Varchar, Text, Numeric, Timestamp).

### 3. Create Migration Files
Set up the `migrations/` directory with initial migration:
- Create directory structure
- Write `up.sql` with CREATE TABLE statements
- Write `down.sql` with DROP TABLE statements
- Ensure proper foreign key relationships and constraints

## Key Requirements

✅ **File Locations**:
- Schema: `src/schema.rs`
- Migrations: `migrations/00000000000001_create_tables/up.sql` and `down.sql`
- Dependencies: Update existing `Cargo.toml`

✅ **Data Integrity**:
- Use foreign keys for relationships (user_id, cart_id, product_id)
- Add UNIQUE constraints where appropriate (username, email)
- Include CASCADE deletes for cart_items → carts relationship

✅ **Completeness**:
- All four tables must be defined
- Schema.rs must compile without errors
- Migration SQL must be valid PostgreSQL

## Constraints
- This is a **Level 0** task with no dependencies
- Other tasks may also modify `Cargo.toml` - use standard formatting
- Keep implementations simple - this is a test project
- Use placeholder/example data structures, not production patterns

## Validation
After completing the work:
1. Verify all files exist at specified paths
2. Ensure `cargo check` passes (schema.rs is valid Rust)
3. Validate migration SQL syntax
4. Confirm dependencies are correctly specified

## Success Definition
Task is complete when:
- `src/schema.rs` exists with all 4 table definitions
- `migrations/` directory contains valid migration files
- `Cargo.toml` includes all required dependencies
- All files are syntactically correct and would compile

## Context
You're working on a parallel task execution test. Other tasks running simultaneously:
- Task 3: User Authentication
- Task 4: Product Catalog
- Task 6: Frontend Components

Your schema provides the foundation for Task 2 (API Endpoints), which depends on this work.

---

**Start working now. Create the files, write the code, and verify completeness.**
