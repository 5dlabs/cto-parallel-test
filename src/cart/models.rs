use serde::{Deserialize, Serialize};

/// Represents a shopping cart item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

/// Represents a user's shopping cart
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

/// Request to add an item to cart
#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: i32,
    pub quantity: i32,
}
