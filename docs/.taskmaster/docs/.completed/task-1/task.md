# Task 1: Database Schema Setup

## Overview
Set up the foundational database schema and configuration for the e-commerce API project using Diesel ORM with PostgreSQL. This task establishes the data layer that all other backend components will depend on.

## Objectives
- Define database table schemas for users, products, carts, and cart_items
- Configure Diesel ORM with PostgreSQL support
- Create database migration files for version control
- Set up database connection pooling
- Implement ORM model structs with appropriate traits

## Context
This is a **Level 0** task with no dependencies, making it suitable for parallel execution. It serves as a foundation for Task 2 (API Endpoints), which depends on this database schema being in place.

## Technical Specifications

### Database Technology
- **ORM**: Diesel 2.1.0
- **Database**: PostgreSQL
- **Connection Pooling**: r2d2 0.8.10
- **Environment Management**: dotenv 0.15.0
- **Date/Time**: chrono 0.4 with serde features

### Database Schema

#### Tables Structure
1. **users** - User account information
   - `id`: SERIAL PRIMARY KEY
   - `username`: VARCHAR NOT NULL UNIQUE
   - `email`: VARCHAR NOT NULL UNIQUE
   - `password_hash`: VARCHAR NOT NULL
   - `created_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

2. **products** - Product catalog
   - `id`: SERIAL PRIMARY KEY
   - `name`: VARCHAR NOT NULL
   - `description`: TEXT
   - `price`: NUMERIC NOT NULL
   - `inventory_count`: INTEGER NOT NULL

3. **carts** - User shopping carts
   - `id`: SERIAL PRIMARY KEY
   - `user_id`: INTEGER NOT NULL REFERENCES users(id)
   - `created_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

4. **cart_items** - Items within carts
   - `id`: SERIAL PRIMARY KEY
   - `cart_id`: INTEGER NOT NULL REFERENCES carts(id)
   - `product_id`: INTEGER NOT NULL REFERENCES products(id)
   - `quantity`: INTEGER NOT NULL

### Entity Relationships
```
users (1) ──────< (N) carts
                       │
                       │ (1)
                       │
                       ▼
                    (N) cart_items (N) ──────> (1) products
```

## Implementation Plan

### Step 1: Add Database Dependencies
**File**: `Cargo.toml`

Add the following dependencies:
```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
r2d2 = "0.8.10"
dotenv = "0.15.0"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Step 2: Create Database Configuration Module
**File**: `src/config/db.rs`

Implement connection pooling:
```rust
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
```

**File**: `.env` (create in project root)
```
DATABASE_URL=postgres://username:password@localhost/database_name
```

### Step 3: Define Database Schema
**File**: `src/schema.rs`

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

joinable!(cart_items -> carts (cart_id));
joinable!(cart_items -> products (product_id));
joinable!(carts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    products,
    carts,
    cart_items,
);
```

### Step 4: Create Database Migrations
**Setup**: Install Diesel CLI (if not already installed)
```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

**Create migrations**:
```bash
diesel migration generate create_users
diesel migration generate create_products
diesel migration generate create_carts
diesel migration generate create_cart_items
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_users/up.sql`
```sql
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_users/down.sql`
```sql
DROP TABLE users;
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_products/up.sql`
```sql
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  price NUMERIC NOT NULL,
  inventory_count INTEGER NOT NULL
);
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_products/down.sql`
```sql
DROP TABLE products;
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_carts/up.sql`
```sql
CREATE TABLE carts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_carts/down.sql`
```sql
DROP TABLE carts;
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_cart_items/up.sql`
```sql
CREATE TABLE cart_items (
  id SERIAL PRIMARY KEY,
  cart_id INTEGER NOT NULL REFERENCES carts(id),
  product_id INTEGER NOT NULL REFERENCES products(id),
  quantity INTEGER NOT NULL
);
```

**File**: `migrations/YYYY-MM-DD-HHMMSS_create_cart_items/down.sql`
```sql
DROP TABLE cart_items;
```

### Step 5: Create Model Structs
**File**: `src/models.rs`

```rust
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub inventory_count: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub inventory_count: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
```

### Step 6: Update Module Exports
**File**: `src/lib.rs` or `src/main.rs`

Add module declarations:
```rust
pub mod config;
pub mod models;
pub mod schema;
```

## Architectural Considerations

### Design Decisions
1. **Diesel ORM**: Chosen for compile-time query validation and type safety
2. **PostgreSQL**: Robust RDBMS with excellent support for concurrent operations
3. **Connection Pooling**: r2d2 ensures efficient database connection management
4. **Foreign Key Constraints**: Enforced at database level for data integrity
5. **Migrations**: Version-controlled schema changes for reproducible deployments

### Data Integrity
- Primary keys on all tables for unique identification
- Unique constraints on username and email to prevent duplicates
- Foreign key relationships ensure referential integrity
- NOT NULL constraints prevent incomplete records

### Performance Considerations
- Connection pooling reduces overhead of connection establishment
- Indexed primary keys for fast lookups
- Foreign key indexes automatically created by PostgreSQL
- Efficient query patterns through Diesel's compile-time checking

## Dependencies
**None** - This is a foundational task that can run in parallel with Tasks 3, 4, and 6.

## Dependent Tasks
- **Task 2: API Endpoints** - Requires this schema to implement database-backed endpoints

## Risks and Mitigation

### Risk: Database Connection Failures
**Mitigation**: Implement robust error handling and connection retry logic in the connection pool configuration.

### Risk: Migration Conflicts
**Mitigation**: Use Diesel's migration system to ensure migrations are applied in correct order with proper rollback support.

### Risk: Data Type Mismatches
**Mitigation**: Diesel's type system catches mismatches at compile time, preventing runtime errors.

## Testing Strategy
Detailed in `acceptance-criteria.md`. Key validation points:
- Cargo build succeeds with all dependencies
- Schema compiles without errors
- Migrations apply and rollback successfully
- Model structs serialize/deserialize correctly
- Connection pool initializes properly

## References
- [Diesel Documentation](https://diesel.rs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- Architecture Document: `.taskmaster/docs/architecture.md` (Database Schema section)
- PRD: `.taskmaster/docs/prd.txt` (Task 1 specification)

## Estimated Effort
30 minutes (as per PRD)
