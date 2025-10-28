# Task 1: Database Schema Setup

## Overview
Create basic database schema files and configuration for the Rust API project. This is a foundational Level 0 task that has no dependencies and should execute in parallel with other Level 0 tasks.

## Context
This task is part of the parallel task execution test project. It establishes the database foundation that Task 2 (API Endpoints) will depend on. The implementation uses Diesel ORM for PostgreSQL database management.

## Objectives
1. Create database schema definitions in `src/schema.rs`
2. Set up initial migration files in the `migrations/` directory
3. Add database dependencies to `Cargo.toml`

## Dependencies
**None** - This is a Level 0 task that can run independently.

## Files to Create/Modify

### 1. `src/schema.rs`
Create basic table definitions for the application:
- **users** table: User authentication and profile data
- **products** table: Product catalog information
- **carts** table: Shopping cart instances
- **cart_items** table: Items within shopping carts

```rust
// Basic schema definitions using Diesel
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

### 2. `migrations/` Directory
Create basic migration files for initial schema setup:
- Migration files follow Diesel's naming convention
- Include up.sql and down.sql for each migration
- Ensure migrations can be applied and rolled back cleanly

### 3. `Cargo.toml` Updates
Add database-related dependencies:

```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
```

## Implementation Steps

1. **Create Schema File**
   - Create `src/schema.rs` with all table definitions
   - Use Diesel's `table!` macro for type-safe schema definitions
   - Define relationships between tables implicitly through foreign key columns

2. **Set Up Migrations**
   - Create `migrations/` directory if it doesn't exist
   - Add initial migration files for schema creation
   - Ensure migration structure follows Diesel conventions

3. **Update Dependencies**
   - Modify `Cargo.toml` to include Diesel with PostgreSQL support
   - Add r2d2 for connection pooling
   - Include dotenv for environment variable management

4. **Validation**
   - Ensure schema definitions are syntactically correct
   - Verify migration files can be parsed
   - Check that dependencies resolve correctly

## Technical Considerations

### Database Choice
- Using PostgreSQL as the database backend
- Diesel 2.1.0 provides compile-time query verification
- R2D2 connection pool for efficient connection management

### Schema Design
- Simple normalized schema appropriate for e-commerce use case
- Foreign key relationships implicit in column names
- Timestamp fields for audit trails

### Migration Strategy
- Migrations should be idempotent where possible
- Down migrations should cleanly reverse up migrations
- Follow Diesel's migration conventions for tooling compatibility

## Integration Points

- **Task 2 (API Endpoints)**: Will import and use schema definitions
- **Task 3 (User Authentication)**: Will use users table
- **Task 4 (Product Catalog)**: Will use products table
- **Task 5 (Shopping Cart)**: Will use carts and cart_items tables

## Risks and Mitigation

**Risk**: Cargo.toml conflicts with Task 2
- **Mitigation**: This is expected and will be handled by the CTO platform's conflict detection

**Risk**: Schema changes needed after initial creation
- **Mitigation**: Keep schema simple and focused on core requirements

## Success Criteria

1. ✅ `src/schema.rs` exists with all four table definitions
2. ✅ Migration files created in `migrations/` directory
3. ✅ `Cargo.toml` includes correct database dependencies
4. ✅ Schema definitions are syntactically valid
5. ✅ Dependencies can be resolved by Cargo
6. ✅ Migration files follow Diesel conventions

## Estimated Effort
**30 minutes** - Simple file creation with predefined structure
