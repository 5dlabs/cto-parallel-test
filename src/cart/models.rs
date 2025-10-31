use serde::{Deserialize, Serialize};

/// Represents a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

/// Represents an item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

impl Cart {
    /// Creates a new empty cart for a user
    #[must_use]
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            items: Vec::new(),
        }
    }
}
