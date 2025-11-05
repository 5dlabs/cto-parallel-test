use crate::schema::{cart_items, carts, products, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// User models
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
    pub price: Decimal,
    pub inventory_count: i32,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
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

// Cart item models
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
