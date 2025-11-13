// Database entity models
use crate::schema::{cart_items, carts, products, users};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// User Models
// ============================================================================

/// User entity representing a registered user account
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    /// Password hash (never expose this in API responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// Data transfer object for creating a new user
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    /// Create a new user DTO
    #[must_use]
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            username,
            email,
            password_hash,
        }
    }
}

// ============================================================================
// Product Models
// ============================================================================

/// Product entity representing an item in the catalog
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

impl NewProduct {
    /// Create a new product DTO
    #[must_use]
    pub fn new(
        name: String,
        description: Option<String>,
        price: BigDecimal,
        inventory_count: i32,
    ) -> Self {
        Self {
            name,
            description,
            price,
            inventory_count,
        }
    }
}

// ============================================================================
// Cart Models
// ============================================================================

/// Cart entity representing a user's shopping cart
#[derive(
    Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize, PartialEq,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

/// Data transfer object for creating a new cart
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

impl NewCart {
    /// Create a new cart DTO for a user
    #[must_use]
    pub const fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

// ============================================================================
// Cart Item Models
// ============================================================================

/// Cart item entity representing a product in a shopping cart
#[derive(
    Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize, PartialEq,
)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

/// Data transfer object for creating a new cart item
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

impl NewCartItem {
    /// Create a new cart item DTO
    #[must_use]
    pub const fn new(cart_id: i32, product_id: i32, quantity: i32) -> Self {
        Self {
            cart_id,
            product_id,
            quantity,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn test_new_user_creation() {
        let user = NewUser::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_new_product_creation() {
        let product = NewProduct::new(
            "Test Product".to_string(),
            Some("A test product".to_string()),
            BigDecimal::from_str("19.99").unwrap(), // $19.99
            10,
        );
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_new_cart_creation() {
        let cart = NewCart::new(1);
        assert_eq!(cart.user_id, 1);
    }

    #[test]
    fn test_new_cart_item_creation() {
        let cart_item = NewCartItem::new(1, 2, 3);
        assert_eq!(cart_item.cart_id, 1);
        assert_eq!(cart_item.product_id, 2);
        assert_eq!(cart_item.quantity, 3);
    }
}
