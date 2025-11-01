use crate::schema::{cart_items, carts, products, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// User model representing a registered user in the system
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

/// `NewUser` model for creating a new user
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Product model representing a product in the catalog
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: bigdecimal::BigDecimal,
    pub inventory_count: i32,
}

/// `NewProduct` model for creating a new product
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: bigdecimal::BigDecimal,
    pub inventory_count: i32,
}

/// Cart model representing a user's shopping cart
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

/// `NewCart` model for creating a new cart
#[derive(Insertable, Debug)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

/// `CartItem` model representing an item in a shopping cart
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

/// `NewCartItem` model for adding an item to a cart
#[derive(Insertable, Debug)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
