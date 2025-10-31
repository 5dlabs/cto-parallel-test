# Autonomous Agent Prompt: Database Schema Setup

## Role
You are a senior Rust backend developer specializing in database design and ORM implementation with Diesel.

## Task
Set up the complete database schema and configuration for an e-commerce API using Diesel ORM with PostgreSQL.

## Objectives
1. Configure Diesel ORM with all necessary dependencies
2. Create database schema definitions for 4 tables (users, products, carts, cart_items)
3. Implement database migration files with up/down scripts
4. Set up connection pooling with r2d2
5. Define Rust model structs with Queryable and Insertable traits

## Technical Context
- **Framework**: Diesel 2.1.0 with PostgreSQL
- **Connection Pooling**: r2d2 0.8.10
- **Environment Variables**: dotenv 0.15.0
- **Serialization**: serde with derive features
- **Database**: PostgreSQL

## Required Deliverables

### 1. Cargo.toml Updates
Add these dependencies:
```toml
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2. Database Configuration (src/config/db.rs)
- Implement `establish_connection_pool()` function
- Use r2d2 for connection pooling
- Read DATABASE_URL from environment via dotenv
- Export Pool and DbConnection type aliases

### 3. Schema Definitions (src/schema.rs)
Define 4 tables using Diesel's table! macro:
- **users**: id, username (unique), email (unique), password_hash, created_at
- **products**: id, name, description, price (Numeric), inventory_count
- **carts**: id, user_id (FK to users), created_at
- **cart_items**: id, cart_id (FK to carts), product_id (FK to products), quantity

Include joinable! and allow_tables_to_appear_in_same_query! macros.

### 4. Database Migrations
Create 4 migration pairs (up.sql and down.sql) for:
1. create_users
2. create_products
3. create_carts (with FK to users)
4. create_cart_items (with FKs to carts and products)

Use:
```bash
diesel migration generate <name>
```

### 5. Model Structs (src/models.rs)
For each table, create:
- **Queryable struct**: For reading from database (with Serialize)
- **Insertable struct**: For inserting records (with Deserialize)
- Appropriate derives: Identifiable, Associations where relevant

### 6. Environment Configuration
Create `.env` file:
```
DATABASE_URL=postgres://username:password@localhost/database_name
```

### 7. Module Exports
Update `src/lib.rs` or `src/main.rs`:
```rust
pub mod config;
pub mod models;
pub mod schema;
```

## Implementation Steps
1. Install Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`
2. Run `diesel setup` to initialize migrations directory
3. Add dependencies to Cargo.toml
4. Create src/config/db.rs with connection pool
5. Generate and implement migrations for each table in order
6. Run `diesel migration run` to apply migrations
7. Create src/schema.rs (Diesel auto-generates base, then customize)
8. Create src/models.rs with all model structs
9. Update module exports
10. Run `cargo check` to verify compilation

## Validation Commands
```bash
# Verify dependencies resolve
cargo build

# Check schema compiles
cargo check

# Apply migrations
diesel migration run

# Test rollback
diesel migration redo

# Verify connection pool
cargo test
```

## Success Criteria
✅ All dependencies added and resolve without errors
✅ Schema compiles without warnings
✅ All 4 migration pairs created and apply successfully
✅ Migration rollback works correctly
✅ Model structs implement correct traits
✅ Connection pool initializes without errors
✅ cargo check passes
✅ Foreign key relationships properly defined

## Constraints
- Use PostgreSQL-specific features only where necessary
- Follow Diesel naming conventions
- Ensure migrations are idempotent where possible
- Include proper error handling in connection pool setup
- Use chrono::NaiveDateTime for timestamp fields
- Password field must be password_hash, never store plaintext

## Error Handling
- Connection pool creation must panic with descriptive error if DATABASE_URL missing
- Migrations must include proper down.sql for rollback
- Model derives must match schema definitions exactly

## Dependencies
None - this is a foundational task that runs independently.

## Output
When complete, confirm:
1. Number of migration files created
2. Result of `cargo check`
3. Result of `diesel migration run`
4. Any warnings or issues encountered
5. Location of all created files

## Notes
- This task establishes the foundation for Task 2 (API Endpoints)
- No actual API implementation is needed in this task
- Focus on schema correctness and migration reliability
- Follow Rust naming conventions (snake_case for fields, PascalCase for types)
