use crate::schema::{cart_items, carts, products, users};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// User models
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Product models
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

// Cart models
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

// CartItem models
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
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
        use std::str::FromStr;
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: BigDecimal::from_str("19.99").unwrap(), // $19.99
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
            quantity: 5,
        };
        assert_eq!(new_cart_item.cart_id, 1);
        assert_eq!(new_cart_item.product_id, 1);
        assert_eq!(new_cart_item.quantity, 5);
    }
}
