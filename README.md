# E-Commerce API - Database Schema

This project implements the foundational database schema for an e-commerce API using Rust, Diesel ORM, and PostgreSQL.

## Overview

This is Task 1 of the e-commerce application project, establishing the data persistence layer with:
- User authentication and management
- Product catalog with inventory tracking
- Shopping cart functionality with multi-item support

## Architecture

### Database Schema

The application uses PostgreSQL with four core tables:

1. **users** - User accounts with authentication
   - `id` (SERIAL PRIMARY KEY)
   - `username` (VARCHAR UNIQUE)
   - `email` (VARCHAR UNIQUE)
   - `password_hash` (VARCHAR)
   - `created_at` (TIMESTAMP)

2. **products** - Product catalog
   - `id` (SERIAL PRIMARY KEY)
   - `name` (VARCHAR)
   - `description` (TEXT)
   - `price` (NUMERIC)
   - `inventory_count` (INTEGER)

3. **carts** - User shopping carts
   - `id` (SERIAL PRIMARY KEY)
   - `user_id` (INTEGER REFERENCES users)
   - `created_at` (TIMESTAMP)

4. **cart_items** - Items in shopping carts
   - `id` (SERIAL PRIMARY KEY)
   - `cart_id` (INTEGER REFERENCES carts)
   - `product_id` (INTEGER REFERENCES products)
   - `quantity` (INTEGER)

### Entity Relationships

```
users (1) ──────< (N) carts
                       │
                       │ (1)
                       │
                       ▼
                    (N) cart_items (N) ──────> (1) products
```

## Technologies

- **Rust** - Systems programming language
- **Diesel 2.1.0** - Type-safe ORM and query builder
- **PostgreSQL** - Relational database
- **r2d2** - Connection pooling
- **dotenvy** - Environment variable management
- **chrono** - Date and time handling
- **serde** - Serialization framework
- **bigdecimal** - Precise decimal arithmetic for prices

## Prerequisites

- Rust 1.70+ with Cargo
- PostgreSQL 12+
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

### 1. Environment Configuration

Create a `.env` file with database connection details. Use standard PostgreSQL URL format.

### 2. Database Initialization

```bash
# Create database and run migrations
diesel setup
diesel migration run

# Verify migrations
diesel migration redo
```

### 3. Build and Run

```bash
# Check compilation
cargo check

# Run tests
cargo test --workspace --all-features

# Run the application
cargo run
```

## Project Structure

```
.
├── Cargo.toml                 # Project dependencies
├── diesel.toml                # Diesel configuration
├── migrations/                # Database migration files
│   ├── 2025-11-01-*_create_users/
│   ├── 2025-11-01-*_create_products/
│   ├── 2025-11-01-*_create_carts/
│   └── 2025-11-01-*_create_cart_items/
├── src/
│   ├── main.rs               # Application entry point
│   ├── lib.rs                # Library exports
│   ├── schema.rs             # Auto-generated Diesel schema
│   ├── models.rs             # ORM model definitions
│   └── config/
│       ├── mod.rs
│       └── db.rs             # Database connection pooling
└── README.md
```

## Code Quality

This project follows strict Rust coding standards:

- ✅ `cargo fmt --all -- --check` - Code formatting
- ✅ `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic` - Linting
- ✅ `cargo test --workspace --all-features` - Testing
- ✅ No unsafe code
- ✅ Comprehensive documentation

## Database Operations

### Connection Pooling

The application uses r2d2 for efficient connection management:

```rust
use ecommerce_api::establish_connection_pool;

let pool = establish_connection_pool();
let conn = pool.get().expect("Failed to get connection");
```

### Models

All database entities have corresponding Rust structs with Diesel traits:

- **Queryable** - For reading from database
- **Insertable** - For inserting new records
- **Identifiable** - For primary key operations
- **Associations** - For foreign key relationships
- **Serialize/Deserialize** - For JSON conversion

## Migration Management

### Create New Migration

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

### Test Migration Rollback/Reapply

```bash
diesel migration redo
```

## Security Features

- Password hashing (field exists, implementation in Task 3)
- Prepared statements via Diesel (SQL injection prevention)
- Environment-based configuration
- Connection pooling with proper resource management

## Next Steps

This schema serves as the foundation for:
- **Task 2**: API Endpoints - REST API implementation
- **Task 3**: User Authentication - JWT and password hashing
- **Task 4**: Product Catalog - Product management API
- **Task 5**: Shopping Cart - Cart operations API

## Verification

To verify the setup:

1. **Database Connection**: `cargo run` should output success messages
2. **Schema Validation**: `psql $DATABASE_URL -c "\dt"` should show all 4 tables
3. **Migration Tests**: `diesel migration redo` should succeed
4. **Code Quality**: All quality gates should pass

## Troubleshooting

### PostgreSQL Connection Failed

Ensure PostgreSQL is running:
```bash
# Check PostgreSQL status
pg_isready -h localhost -p 5432

# Start PostgreSQL service or container as needed
```

### Migration Errors

```bash
# Reset migrations
diesel migration revert --all
diesel migration run
```

### Compilation Errors

```bash
# Clean build artifacts
cargo clean
cargo check
```

## License

This project is part of the cto-parallel-test repository.
