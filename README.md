# CTO Parallel Test - E-Commerce API

A Rust-based e-commerce API backend with PostgreSQL database integration.

## Database Schema

This project implements a complete database schema for an e-commerce application with the following tables:

- **users**: User accounts with authentication
- **products**: Product catalog with inventory tracking
- **carts**: Shopping carts associated with users
- **cart_items**: Items in shopping carts with quantities

## Prerequisites

- Rust 1.70+ (edition 2021)
- PostgreSQL 12+
- Diesel CLI (install with: `cargo install diesel_cli --no-default-features --features postgres`)

## Database Setup

1. **Configure Database Connection**

   Create a `.env` file in the project root:
   ```
   DATABASE_URL=postgres://your_username:your_password@localhost:5432/ecommerce_db
   ```

2. **Initialize Database**

   ```bash
   # Set up the database and migrations directory
   diesel setup
   
   # Run all migrations to create tables
   diesel migration run
   ```

3. **Verify Schema**

   The migrations will create the following schema:

   - `users` table with unique username and email
   - `products` table with NUMERIC price field for financial accuracy
   - `carts` table with foreign key to users (ON DELETE CASCADE)
   - `cart_items` table with foreign keys to carts and products (ON DELETE CASCADE)

## Project Structure

```
src/
├── config/
│   ├── db.rs          # Database connection pooling with r2d2
│   └── mod.rs
├── models.rs          # Diesel ORM models (User, Product, Cart, CartItem)
├── schema.rs          # Auto-generated Diesel schema definitions
├── lib.rs             # Library entry point
└── tests.rs           # Unit tests for models

migrations/
├── 2025-11-13-*-create_users/
├── 2025-11-13-*-create_products/
├── 2025-11-13-*-create_carts/
└── 2025-11-13-*-create_cart_items/
```

## Dependencies

- **diesel** (2.3.x): ORM and query builder
- **r2d2** (0.8.x): Connection pooling
- **dotenv** (0.15.x): Environment variable management
- **chrono** (0.4.x): Date and time handling
- **serde** (1.0.x): Serialization/deserialization
- **bigdecimal** (0.4.x): Precise decimal arithmetic for prices

## Development

### Building

```bash
# Check compilation
cargo check

# Build the project
cargo build

# Build optimized release
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings -W clippy::pedantic
```

### Database Operations

```bash
# Revert last migration
diesel migration revert

# Test migrations (revert and re-apply)
diesel migration redo

# Generate new migration
diesel migration generate <name>
```

## Features

### Database Models

All models include:
- Proper Diesel derive macros (`Queryable`, `Insertable`, `Associations`)
- Serde serialization support
- Debug and Clone implementations where appropriate
- Type-safe foreign key relationships

### Connection Pooling

The `establish_connection_pool()` function in `src/config/db.rs` provides:
- r2d2 connection pooling for efficient connection management
- Environment-based configuration via `DATABASE_URL`
- Panic-based error handling for critical initialization failures

### Schema Management

Diesel migrations ensure:
- Version-controlled schema changes
- Reversible migrations (up/down SQL)
- Foreign key constraints with cascade delete
- Proper indexes (primary keys, unique constraints)

## Task Information

**Task ID**: 1
**Task Name**: Database Schema Setup
**Status**: Complete
**Dependencies**: None (Level 0 task)

This implementation provides the foundational data layer for the e-commerce API, enabling downstream tasks to build authentication, product catalog, and shopping cart features.

## Next Steps

After this database schema is in place:
- Task 2: API Endpoints (depends on schema)
- Task 3: User Authentication (uses users table)
- Task 4: Product Catalog (uses products table)
- Task 5: Shopping Cart (uses carts and cart_items tables)
