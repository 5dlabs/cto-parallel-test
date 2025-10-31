# Task 1: Database Schema Setup

## Overview
Set up the foundational database schema and configuration for the e-commerce Rust API project using Diesel ORM and PostgreSQL.

## Context
This is a **Level 0 task** with no dependencies, designed to establish the data persistence layer that other tasks will build upon. The schema defines four core tables: users, products, carts, and cart_items.

## Objectives
1. Configure Diesel ORM with PostgreSQL support in Cargo.toml
2. Create schema.rs with table definitions
3. Implement database connection pooling with r2d2
4. Set up migration files for version-controlled schema changes
5. Create model structs for database entities

## Dependencies
- **Upstream**: None (Level 0 task)
- **Downstream**: Task 2 (API Endpoints) depends on this schema

## Technical Specifications

### Database Schema Design

#### Users Table
Stores user authentication and profile data:
```sql
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

#### Products Table
Catalog of available products:
```sql
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  price NUMERIC NOT NULL,
  inventory_count INTEGER NOT NULL
);
```

#### Carts Table
User shopping carts (one per user):
```sql
CREATE TABLE carts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

#### Cart Items Table
Items within shopping carts:
```sql
CREATE TABLE cart_items (
  id SERIAL PRIMARY KEY,
  cart_id INTEGER NOT NULL REFERENCES carts(id),
  product_id INTEGER NOT NULL REFERENCES products(id),
  quantity INTEGER NOT NULL
);
```

## Implementation Plan

### Step 1: Add Database Dependencies
Update `Cargo.toml` to include:
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
Create `src/config/db.rs`:
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

### Step 3: Define Schema in schema.rs
Create `src/schema.rs`:
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

### Step 4: Create Migration Files
Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

Initialize and create migrations:
```bash
diesel setup
diesel migration generate create_users
diesel migration generate create_products
diesel migration generate create_carts
diesel migration generate create_cart_items
```

### Step 5: Implement Model Structs
Create `src/models.rs`:
```rust
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub inventory_count: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub inventory_count: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "carts"]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "carts"]
pub struct NewCart {
    pub user_id: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Cart)]
#[belongs_to(Product)]
#[table_name = "cart_items"]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[table_name = "cart_items"]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
```

## Architecture Considerations

### Database Connection Pooling
- Uses r2d2 for efficient connection management
- Pools connections to avoid overhead of creating new connections per request
- Configurable pool size for production scalability

### Schema Relationships
- **One-to-Many**: User → Carts (one user can have multiple carts over time)
- **One-to-Many**: Cart → CartItems (one cart contains multiple items)
- **Many-to-One**: CartItem → Product (many cart items reference the same product)

### Migration Strategy
- Each table has its own migration for granular version control
- `up.sql` creates tables, `down.sql` drops them
- Migrations are idempotent and reversible

## Risks and Mitigation

### Risk: Database Connection Failures
**Mitigation**: Connection pooling with retry logic and proper error handling

### Risk: Migration Conflicts
**Mitigation**: Sequential migration numbering, test migrations in development first

### Risk: Schema Changes Breaking Existing Code
**Mitigation**: Version control migrations, comprehensive testing before deployment

## Testing Strategy

### Unit Tests
Test model struct creation and serialization:
```rust
#[test]
fn test_new_user_creation() {
    let user = NewUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };
    assert_eq!(user.username, "testuser");
}
```

### Integration Tests
1. Verify Cargo.toml dependencies resolve correctly (`cargo check`)
2. Test migration application (`diesel migration run`)
3. Test migration rollback (`diesel migration redo`)
4. Verify schema definitions compile without errors

### Validation Checklist
- [ ] All files created: `src/schema.rs`, `src/models.rs`, `src/config/db.rs`, migration files
- [ ] Cargo.toml updated with database dependencies
- [ ] Schema compiles without errors (`cargo check`)
- [ ] Migrations can be applied successfully
- [ ] Database connection pool can be established
- [ ] Model structs properly implement required traits

## Environment Setup

Create `.env` file in project root:
```
DATABASE_URL=postgres://username:password@localhost/ecommerce_db
```

## References
- [Diesel ORM Documentation](https://diesel.rs/)
- [PostgreSQL Data Types](https://www.postgresql.org/docs/current/datatype.html)
- [r2d2 Connection Pooling](https://docs.rs/r2d2/)

## Completion Criteria
✅ Database dependencies added to Cargo.toml
✅ Schema.rs created with all four tables
✅ Migration files created and tested
✅ Database connection module implemented
✅ Model structs created with proper traits
✅ Project builds successfully with `cargo check`
