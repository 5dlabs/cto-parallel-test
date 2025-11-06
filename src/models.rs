/// Database ORM models for the e-commerce system
///
/// This module contains all the database models and their associated types.
/// Models are defined using Diesel's derive macros for seamless ORM integration.
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// User Models
// ============================================================================

/// User model representing a registered user in the system
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Hashed password
    pub password_hash: String,
    /// Account creation timestamp
    pub created_at: NaiveDateTime,
}

/// Insertable user model for creating new users
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    /// Username for the new user
    pub username: String,
    /// Email address for the new user
    pub email: String,
    /// Hashed password for the new user
    pub password_hash: String,
}

// ============================================================================
// Product Models
// ============================================================================

/// Product model representing an item in the catalog
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: Option<String>,
    /// Product price (using `bigdecimal::BigDecimal` for precision)
    pub price: bigdecimal::BigDecimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Insertable product model for creating new products
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct {
    /// Name of the new product
    pub name: String,
    /// Description of the new product
    pub description: Option<String>,
    /// Price of the new product
    pub price: bigdecimal::BigDecimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

// ============================================================================
// Cart Models
// ============================================================================

/// Cart model representing a user's shopping cart
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::carts)]
pub struct Cart {
    /// Unique cart identifier
    pub id: i32,
    /// ID of the user who owns this cart
    pub user_id: i32,
    /// Cart creation timestamp
    pub created_at: NaiveDateTime,
}

/// Insertable cart model for creating new carts
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::carts)]
pub struct NewCart {
    /// ID of the user who will own this cart
    pub user_id: i32,
}

// ============================================================================
// Cart Item Models
// ============================================================================

/// Cart item model representing a product in a shopping cart
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = crate::schema::cart_items)]
pub struct CartItem {
    /// Unique cart item identifier
    pub id: i32,
    /// ID of the cart this item belongs to
    pub cart_id: i32,
    /// ID of the product in this cart item
    pub product_id: i32,
    /// Quantity of the product
    pub quantity: i32,
}

/// Insertable cart item model for adding items to a cart
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::cart_items)]
pub struct NewCartItem {
    /// ID of the cart to add this item to
    pub cart_id: i32,
    /// ID of the product to add
    pub product_id: i32,
    /// Quantity of the product to add
    pub quantity: i32,
}
