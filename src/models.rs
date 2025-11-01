use crate::schema::{cart_items, carts, products, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a user in the database.
///
/// This struct is used for querying existing users from the database.
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

/// Represents a new user to be inserted into the database.
///
/// This struct is used when creating a new user account.
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Represents a product in the database.
///
/// This struct is used for querying existing products from the database.
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product to be inserted into the database.
///
/// This struct is used when adding a new product to the catalog.
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a shopping cart in the database.
///
/// This struct is used for querying existing carts from the database.
/// Each cart belongs to a specific user.
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

/// Represents a new cart to be inserted into the database.
///
/// This struct is used when creating a new shopping cart for a user.
#[derive(Insertable, Debug)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

/// Represents a cart item in the database.
///
/// This struct is used for querying existing cart items from the database.
/// Each cart item belongs to a specific cart and references a product.
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

/// Represents a new cart item to be inserted into the database.
///
/// This struct is used when adding a product to a shopping cart.
#[derive(Insertable, Debug)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
