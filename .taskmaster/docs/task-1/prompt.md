# Autonomous Agent Prompt: Database Schema Setup

## Mission
You are tasked with creating database schema files and configuration for a Rust API project using Diesel ORM. This is a straightforward file creation task with predefined structure.

## What You Need to Do

### 1. Create Database Schema (`src/schema.rs`)
Create a new file at `src/schema.rs` with the following table definitions:

```rust
// Basic schema definitions
table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    products (id) {
        id -> Integer,
        name -> Varchar,
        description -> Text,
        price -> Numeric,
        inventory_count -> Integer,
    }
}

table! {
    carts (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    cart_items (id) {
        id -> Integer,
        cart_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
    }
}
```

### 2. Create Migration Files (`migrations/`)
Create the migrations directory and add basic migration files:
- Create `migrations/` directory in the project root
- Add initial migration for schema creation
- Follow Diesel migration conventions (timestamp-based naming)

Example migration structure:
```
migrations/
  └── 00000000000000_create_tables/
      ├── up.sql
      └── down.sql
```

### 3. Update Cargo.toml
Add these dependencies to the `[dependencies]` section:

```toml
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
```

## Expected Behavior
- All schema definitions use Diesel's `table!` macro
- Migration files can be applied to create database schema
- Dependencies are correctly specified for PostgreSQL support

## Validation
Before marking this task complete:
1. Verify `src/schema.rs` exists and compiles
2. Verify `migrations/` directory has proper structure
3. Verify `Cargo.toml` has database dependencies
4. Run `cargo check` to ensure no syntax errors

## Constraints
- This is a test project - keep implementation simple
- Use exact table names and column types as specified
- No actual database connection required - files only
- Do not add extra features beyond requirements

## Success Definition
Task is complete when:
- ✅ `src/schema.rs` created with 4 table definitions
- ✅ Migration directory structure created
- ✅ `Cargo.toml` updated with dependencies
- ✅ Code passes syntax validation

## Notes
- This task has no dependencies and can run in parallel
- Task 2 will depend on this schema being available
- File conflicts with Task 2 on Cargo.toml are expected and acceptable
