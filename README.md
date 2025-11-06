# E-Commerce API

A Rust-based e-commerce API using Diesel ORM and PostgreSQL.

## Prerequisites

- Rust (1.70+)
- PostgreSQL (16+)
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Configure the database:**
   
   Create a `.env` file in the project root with your PostgreSQL connection string.
   Use the standard PostgreSQL URL format with your credentials:
   ```
   DATABASE_URL=postgres://<username>:<password>@<host>/<database>
   ```

3. **Run migrations:**
   ```bash
   diesel migration run
   ```

## Database Schema

The application uses the following tables:

- **users**: User accounts with authentication
- **products**: Product catalog with inventory
- **carts**: Shopping carts associated with users
- **cart_items**: Items within shopping carts

### Entity Relationships

```
users (1) ──────< (N) carts
                       │
                       │ (1)
                       │
                       ▼
                    (N) cart_items (N) ──────> (1) products
```

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── schema.rs           # Diesel schema definitions (auto-generated)
├── models.rs           # ORM models and entities
└── config/
    ├── mod.rs          # Config module exports
    └── db.rs           # Database connection pooling

migrations/
├── 00000000000001_create_users/
├── 00000000000002_create_products/
├── 00000000000003_create_carts/
└── 00000000000004_create_cart_items/
```

## Development

### Run tests
```bash
cargo test --workspace --all-features
```

### Run linter
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```

### Format code
```bash
cargo fmt --all
```

## Database Migrations

### Create a new migration
```bash
diesel migration generate <migration_name>
```

### Apply migrations
```bash
diesel migration run
```

### Rollback migrations
```bash
diesel migration revert
```

### Test migrations (run + revert)
```bash
diesel migration redo
```

## Features

- **Diesel ORM**: Type-safe SQL query builder
- **Connection Pooling**: r2d2 for efficient database connections
- **Migrations**: Version-controlled schema changes
- **Type Safety**: Compile-time verification of queries

## Dependencies

- `diesel`: PostgreSQL ORM with r2d2 and chrono support
- `r2d2`: Connection pooling
- `dotenv`: Environment variable management
- `chrono`: Date and time handling
- `serde`: Serialization framework
- `bigdecimal`: Decimal number handling for prices

## License

[Add your license here]
