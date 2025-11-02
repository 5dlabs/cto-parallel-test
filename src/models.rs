use crate::schema::{cart_items, carts, products, users};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// User models

/// Represents a user in the database.
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// Data required to create a new user.
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Product models

/// Represents a product in the database.
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

/// Data required to create a new product.
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

// Cart models

/// Represents a shopping cart in the database.
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

/// Data required to create a new cart.
#[derive(Insertable, Debug)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

// Cart Item models

/// Represents an item in a shopping cart.
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

/// Data required to create a new cart item.
#[derive(Insertable, Debug)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
