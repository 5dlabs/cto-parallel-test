# E-Commerce API - Database Schema

A Rust-based e-commerce API with PostgreSQL database backend using Diesel ORM.

## Features

- **User Management**: User registration and authentication with secure password hashing
- **Product Catalog**: Product management with inventory tracking
- **Shopping Cart**: Multi-item shopping cart with quantity management
- **Database Migrations**: Version-controlled schema changes with Diesel

## Prerequisites

- Rust 1.70 or later
- PostgreSQL 12 or later
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Database Setup

### 1. Environment Configuration

Create a `.env` file in the project root with your PostgreSQL connection string.

See the existing `.env` file for the correct format.

### 2. Initialize Database

```bash
# Create the database
diesel setup

# Run migrations
diesel migration run
```

### 3. Verify Schema

```bash
# Check that all tables were created
psql $DATABASE_URL -c "\dt"

# Expected output:
# - users
# - products
# - carts
# - cart_items
```

## Database Schema

### Users Table

Stores user account information.

```sql
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Products Table

Stores product catalog information.

```sql
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  price NUMERIC NOT NULL CHECK (price >= 0),
  inventory_count INTEGER NOT NULL DEFAULT 0 CHECK (inventory_count >= 0)
);
```

### Carts Table

Stores shopping carts associated with users.

```sql
CREATE TABLE carts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Cart Items Table

Stores individual items in shopping carts.

```sql
CREATE TABLE cart_items (
  id SERIAL PRIMARY KEY,
  cart_id INTEGER NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
  product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  quantity INTEGER NOT NULL CHECK (quantity > 0)
);
```

## Entity Relationships

```
users (1) ──────< (N) carts
                       │
                       │ (1)
                       │
                       ▼
                    (N) cart_items (N) ──────> (1) products
```

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Format

```bash
cargo fmt --all
```

### Lint

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```

## Project Structure

```
.
├── Cargo.toml              # Project dependencies
├── diesel.toml             # Diesel configuration
├── migrations/             # Database migration files
│   ├── 00000000000001_create_users/
│   ├── 00000000000002_create_products/
│   ├── 00000000000003_create_carts/
│   └── 00000000000004_create_cart_items/
└── src/
    ├── lib.rs              # Library root
    ├── main.rs             # Application entry point
    ├── schema.rs           # Generated database schema
    ├── models.rs           # ORM model structs
    └── config/
        ├── mod.rs          # Configuration module
        └── db.rs           # Database connection pooling
```

## Database Connection Pooling

The application uses R2D2 connection pooling for efficient database access:

- **Max Pool Size**: 10 connections
- **Min Idle Connections**: 2 connections

Configuration can be adjusted in `src/config/db.rs`.

## Migration Management

### Create a New Migration

```bash
diesel migration generate migration_name
```

### Apply Migrations

```bash
diesel migration run
```

### Rollback Last Migration

```bash
diesel migration revert
```

### Test Migrations (Up and Down)

```bash
diesel migration redo
```

## Security Considerations

- **Password Storage**: Never store plaintext passwords. Use Argon2 for hashing.
- **SQL Injection**: Diesel ORM prevents SQL injection through parameterized queries.
- **Environment Variables**: Store sensitive credentials in `.env` (never commit this file).
- **Foreign Keys**: All foreign keys use `ON DELETE CASCADE` for referential integrity.

## Future Tasks

This database schema is the foundation for:

- **Task 2**: API Endpoints - REST API implementation
- **Task 3**: User Authentication - JWT and password hashing
- **Task 4**: Product Catalog - Product management service
- **Task 5**: Shopping Cart - Cart management service
- **Task 6**: Frontend Components - React UI
- **Task 7**: Integration Tests - End-to-end testing

## License

Copyright © 2024 5DLabs

## Documentation

For more details, see:
- [Task Documentation](./task/task.md)
- [Acceptance Criteria](./task/acceptance-criteria.md)
- [Architecture Documentation](./docs/.taskmaster/docs/architecture.md)
