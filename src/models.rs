use crate::schema::{cart_items, carts, products, users};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a user in the system
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// Represents a new user to be inserted into the database
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Represents a product in the catalog
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

/// Represents a new product to be inserted into the database
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub inventory_count: i32,
}

/// Represents a shopping cart belonging to a user
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

/// Represents a new cart to be inserted into the database
#[derive(Insertable, Debug)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

/// Represents an item in a shopping cart
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

/// Represents a new cart item to be inserted into the database
#[derive(Insertable, Debug)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
