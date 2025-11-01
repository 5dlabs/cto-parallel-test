//! Database model definitions
//!
//! This module contains all the ORM models for database entities,
//! including both query and insert structs.

use crate::schema::{cart_items, carts, products, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// User Models
// ============================================================================

/// User entity representing a registered user in the system
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique identifier for the user
    pub id: i32,
    /// Username (must be unique)
    pub username: String,
    /// Email address (must be unique)
    pub email: String,
    /// Hashed password (using Argon2)
    pub password_hash: String,
    /// Timestamp when the user was created
    pub created_at: NaiveDateTime,
}

/// Struct for inserting a new user into the database
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    /// Username for the new user
    pub username: String,
    /// Email address for the new user
    pub email: String,
    /// Pre-hashed password
    pub password_hash: String,
}

// ============================================================================
// Product Models
// ============================================================================

/// Product entity representing an item in the catalog
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    /// Unique identifier for the product
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: Option<String>,
    /// Product price (stored as NUMERIC for precision)
    pub price: rust_decimal::Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Struct for inserting a new product into the database
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Product description
    pub description: Option<String>,
    /// Product price
    pub price: rust_decimal::Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

// ============================================================================
// Cart Models
// ============================================================================

/// Cart entity representing a user's shopping cart
#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    /// Unique identifier for the cart
    pub id: i32,
    /// User ID who owns this cart
    pub user_id: i32,
    /// Timestamp when the cart was created
    pub created_at: NaiveDateTime,
}

/// Struct for inserting a new cart into the database
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = carts)]
pub struct NewCart {
    /// User ID who will own this cart
    pub user_id: i32,
}

// ============================================================================
// Cart Item Models
// ============================================================================

/// Cart item entity representing a product in a user's cart
#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    /// Unique identifier for the cart item
    pub id: i32,
    /// Cart ID this item belongs to
    pub cart_id: i32,
    /// Product ID for the item
    pub product_id: i32,
    /// Quantity of this product in the cart
    pub quantity: i32,
}

/// Struct for inserting a new cart item into the database
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    /// Cart ID this item belongs to
    pub cart_id: i32,
    /// Product ID for the item
    pub product_id: i32,
    /// Quantity of this product
    pub quantity: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_creation() {
        let new_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$...".to_string(),
        };

        assert_eq!(new_user.username, "testuser");
        assert_eq!(new_user.email, "test@example.com");
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: rust_decimal::Decimal::new(1999, 2), // $19.99
            inventory_count: 100,
        };

        assert_eq!(new_product.name, "Test Product");
        assert_eq!(new_product.inventory_count, 100);
    }
}
