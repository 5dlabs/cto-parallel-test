# Task 1: Database Schema Setup

## Overview
Create basic database schema files and configuration for the Rust API project. This is a Level 0 task (no dependencies) that establishes the foundational data layer for the application.

## Context
This task is part of a parallel task execution test to validate the CTO platform's orchestration capabilities. The database schema will define tables for users, products, carts, and cart items that subsequent tasks will build upon.

## Objectives
1. Define database schema with Diesel ORM
2. Create initial migration files
3. Configure database dependencies in Cargo.toml
4. Establish data models for users, products, carts, and cart items

## Dependencies
**None** - This is a Level 0 task that can run independently in parallel with Tasks 3, 4, and 6.

## Files to Create
- `src/schema.rs` - Diesel table definitions
- `migrations/` - Directory containing database migration files
- `Cargo.toml` - Updates for database dependencies

## Technical Specifications

### Database Technology
- **ORM**: Diesel 2.1.0
- **Database**: PostgreSQL (implied by Diesel features)
- **Connection Pool**: r2d2 0.8.10
- **Configuration**: dotenv 0.15.0

### Schema Design

#### Users Table
```rust
table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}
```

#### Products Table
```rust
table! {
    products (id) {
        id -> Integer,
        name -> Varchar,
        description -> Text,
        price -> Numeric,
        inventory_count -> Integer,
    }
}
```

#### Carts Table
```rust
table! {
    carts (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Timestamp,
    }
}
```

#### Cart Items Table
```rust
table! {
    cart_items (id) {
        id -> Integer,
        cart_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
    }
}
```

## Implementation Plan

### Step 1: Update Cargo.toml
Add the following dependencies to `Cargo.toml`:

```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
```

### Step 2: Create Schema File
Create `src/schema.rs` with all table definitions using Diesel's `table!` macro. This file will be auto-generated initially by Diesel CLI but can be created manually for this test.

### Step 3: Create Migrations Directory
Create `migrations/` directory structure:
- `migrations/00000000000000_diesel_initial_setup/`
- `migrations/00000000000001_create_tables/`

Each migration should contain:
- `up.sql` - SQL to apply the migration
- `down.sql` - SQL to rollback the migration

### Step 4: Write Migration SQL
Example `up.sql` for initial tables:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2) NOT NULL,
    inventory_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE carts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cart_items (
    id SERIAL PRIMARY KEY,
    cart_id INTEGER NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL DEFAULT 1,
    UNIQUE(cart_id, product_id)
);
```

## Architectural Considerations

### Database Choice
PostgreSQL is selected via Diesel's features. The schema uses standard SQL types that map cleanly to Rust types through Diesel's type system.

### Relationship Design
- **Users → Carts**: One-to-many relationship
- **Carts → Cart Items**: One-to-many with CASCADE delete
- **Products → Cart Items**: One-to-many
- Enforced at the database level with foreign keys

### Scalability Notes
This is a test/placeholder implementation. In production:
- Consider adding indexes on frequently queried columns
- Add created_at/updated_at to all tables
- Implement soft deletes for audit trails
- Consider partitioning strategies for large tables

## Risks and Considerations

1. **File Conflicts**: Task 2 (API Endpoints) will also modify `Cargo.toml` to add web framework dependencies. The orchestrator should detect and merge these changes.

2. **Migration Order**: Migrations must maintain referential integrity. The provided structure ensures parent tables are created before child tables.

3. **Minimal Implementation**: This is intentionally minimal to focus on task orchestration testing, not production-ready database design.

## Testing Strategy
See `acceptance-criteria.md` for detailed validation steps.

## Success Criteria
- All files created in correct locations
- Schema definitions are syntactically valid Rust code
- Migration SQL is valid PostgreSQL
- Dependencies resolve correctly
- Code compiles without errors

## Related Tasks
- **Task 2**: API Endpoints (depends on this task)
- **Task 3**: User Authentication (uses users table)
- **Task 4**: Product Catalog (uses products table)
- **Task 5**: Shopping Cart (depends on Tasks 3 & 4, uses carts tables)

## References
- [Diesel Documentation](https://diesel.rs/)
- [PostgreSQL Data Types](https://www.postgresql.org/docs/current/datatype.html)
- Project PRD: `.taskmaster/docs/prd.txt`
