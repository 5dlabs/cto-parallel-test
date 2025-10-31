# Autonomous Agent Prompt: Database Schema Setup

## Mission
You are a senior Rust backend engineer tasked with implementing the database schema layer for an e-commerce API using Diesel ORM and PostgreSQL. This is the foundational data layer upon which other services will be built.

## Task Context
- **Task ID**: 1
- **Priority**: High
- **Execution Level**: 0 (no dependencies, can run in parallel)
- **Estimated Time**: 30 minutes
- **Downstream Impact**: Task 2 (API Endpoints) depends on this

## Objective
Set up a complete, production-ready database schema with connection pooling, migrations, and ORM models for four core tables: users, products, carts, and cart_items.

## Constraints
- Must use Diesel ORM 2.1.0 with PostgreSQL
- Must implement connection pooling with r2d2
- All migrations must be reversible
- Schema must support the relationships: users → carts → cart_items → products
- No external API calls or network dependencies

## Required Deliverables

### 1. Update Cargo.toml
Add these exact dependencies:
```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2. Create src/config/db.rs
Implement database connection pooling configuration with proper error handling.

### 3. Create src/schema.rs
Define all four tables with proper relationships:
- `users`: id, username (unique), email (unique), password_hash, created_at
- `products`: id, name, description, price, inventory_count
- `carts`: id, user_id (FK), created_at
- `cart_items`: id, cart_id (FK), product_id (FK), quantity

Include `joinable!` and `allow_tables_to_appear_in_same_query!` macros.

### 4. Create Migration Files
Generate migrations using Diesel CLI:
- One migration per table
- Each with up.sql and down.sql
- Include proper foreign key constraints
- Add indices where appropriate

### 5. Create src/models.rs
Implement Queryable and Insertable structs for all entities with proper traits:
- User / NewUser
- Product / NewProduct
- Cart / NewCart
- CartItem / NewCartItem

### 6. Create .env Template
Provide a `.env.example` file with:
```
DATABASE_URL=postgres://username:password@localhost/ecommerce_db
```

## Implementation Steps

1. **Install Diesel CLI** (if not available):
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

2. **Initialize Diesel**:
   ```bash
   diesel setup
   ```

3. **Create migrations**:
   ```bash
   diesel migration generate create_users
   diesel migration generate create_products
   diesel migration generate create_carts
   diesel migration generate create_cart_items
   ```

4. **Implement migration SQL files** for each table

5. **Run migrations**:
   ```bash
   diesel migration run
   ```

6. **Create configuration and model files** as specified

7. **Update module exports** in `src/lib.rs` or `src/main.rs`:
   ```rust
   pub mod config;
   pub mod models;
   pub mod schema;
   ```

8. **Validate** with `cargo check` and `cargo test`

## Success Criteria
- All files created successfully
- `cargo check` completes without errors
- `diesel migration run` executes successfully
- Database pool can be established
- Model structs serialize/deserialize correctly
- Foreign key relationships are properly defined

## Error Handling
If you encounter issues:
1. Verify PostgreSQL is installed and running
2. Check DATABASE_URL is correctly formatted
3. Ensure Diesel CLI is installed with postgres feature
4. Validate SQL syntax in migration files
5. Confirm all imports are correct

## Validation Commands
```bash
# Build check
cargo check

# Run migrations
diesel migration run

# Test migration rollback
diesel migration redo

# Run tests
cargo test
```

## Expected Output Structure
```
src/
├── config/
│   └── db.rs
├── models.rs
└── schema.rs

migrations/
├── 2024*_create_users/
│   ├── up.sql
│   └── down.sql
├── 2024*_create_products/
│   ├── up.sql
│   └── down.sql
├── 2024*_create_carts/
│   ├── up.sql
│   └── down.sql
└── 2024*_create_cart_items/
    ├── up.sql
    └── down.sql

.env.example
Cargo.toml (updated)
```

## Quality Standards
- Follow Rust naming conventions (snake_case for fields/functions)
- Include proper error messages in connection pool setup
- Add comments explaining foreign key relationships
- Ensure all structs derive necessary traits (Serialize, Deserialize, etc.)
- Use proper PostgreSQL data types (SERIAL, VARCHAR, INTEGER, NUMERIC, TIMESTAMP)

## Notes
- This task runs in parallel with Tasks 3, 4, and 6
- Task 2 will depend on your schema being complete
- Focus on correctness over optimization at this stage
- Placeholder implementation is acceptable for this test project

Execute this task autonomously, following best practices for Rust and Diesel ORM development.
