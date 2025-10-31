//! Database entity models for the e-commerce application
//!
//! This module defines all the ORM models for database entities including
//! users, products, carts, and cart items. Each entity has both a queryable
//! struct for reading from the database and an insertable struct for creating
//! new records.

use crate::schema::{cart_items, carts, products, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ============================================================================
// User Models
// ============================================================================

/// Represents a user in the database
///
/// This struct is used for reading user records from the database.
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// Unique username
    pub username: String,
    /// Unique email address
    pub email: String,
    /// Hashed password (never send in API responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Account creation timestamp
    pub created_at: NaiveDateTime,
}

/// Represents data needed to create a new user
///
/// This struct is used for inserting new user records into the database.
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
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

/// Represents a product in the catalog
///
/// This struct is used for reading product records from the database.
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: Option<String>,
    /// Product price (using Decimal for financial precision)
    pub price: Decimal,
    /// Available inventory count
    pub inventory_count: i32,
}

/// Represents data needed to create a new product
///
/// This struct is used for inserting new product records into the database.
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    /// Name of the new product
    pub name: String,
    /// Description of the new product
    pub description: Option<String>,
    /// Price of the new product
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

// ============================================================================
// Cart Models
// ============================================================================

/// Represents a shopping cart in the database
///
/// This struct is used for reading cart records from the database.
#[derive(
    Debug, Clone, Queryable, Identifiable, Selectable, Associations, Serialize, Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    /// Unique cart identifier
    pub id: i32,
    /// ID of the user who owns this cart
    pub user_id: i32,
    /// Cart creation timestamp
    pub created_at: NaiveDateTime,
}

/// Represents data needed to create a new cart
///
/// This struct is used for inserting new cart records into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = carts)]
pub struct NewCart {
    /// ID of the user who will own this cart
    pub user_id: i32,
}

// ============================================================================
// Cart Item Models
// ============================================================================

/// Represents an item within a shopping cart
///
/// This struct is used for reading cart item records from the database.
#[derive(
    Debug, Clone, Queryable, Identifiable, Selectable, Associations, Serialize, Deserialize,
)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
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

/// Represents data needed to create a new cart item
///
/// This struct is used for inserting new cart item records into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    /// ID of the cart this item will belong to
    pub cart_id: i32,
    /// ID of the product for this cart item
    pub product_id: i32,
    /// Quantity of the product
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
            password_hash: "hashed_password".to_string(),
        };

        assert_eq!(new_user.username, "testuser");
        assert_eq!(new_user.email, "test@example.com");
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 100,
        };

        assert_eq!(new_product.name, "Test Product");
        assert_eq!(new_product.inventory_count, 100);
    }

    #[test]
    fn test_new_cart_creation() {
        let new_cart = NewCart { user_id: 1 };

        assert_eq!(new_cart.user_id, 1);
    }

    #[test]
    fn test_new_cart_item_creation() {
        let new_cart_item = NewCartItem {
            cart_id: 1,
            product_id: 1,
            quantity: 2,
        };

        assert_eq!(new_cart_item.cart_id, 1);
        assert_eq!(new_cart_item.product_id, 1);
        assert_eq!(new_cart_item.quantity, 2);
    }
}
