use serde::{Deserialize, Serialize};

/// Shopping cart item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

/// Shopping cart for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
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

    /// Adds a product to the cart or updates quantity if already present
    pub fn add_item(&mut self, product_id: i32, quantity: i32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            self.items.push(CartItem {
                product_id,
                quantity,
            });
        }
    }

    /// Removes a product from the cart
    pub fn remove_item(&mut self, product_id: i32) {
        self.items.retain(|item| item.product_id != product_id);
    }

    /// Clears all items from the cart
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_creation() {
        let cart = Cart::new("user_123".to_string());
        assert_eq!(cart.user_id, "user_123");
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut cart = Cart::new("user_123".to_string());

        cart.add_item(1, 2);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);

        // Adding same product should update quantity
        cart.add_item(1, 3);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_remove_item() {
        let mut cart = Cart::new("user_123".to_string());

        cart.add_item(1, 2);
        cart.add_item(2, 3);
        assert_eq!(cart.items.len(), 2);

        cart.remove_item(1);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_clear_cart() {
        let mut cart = Cart::new("user_123".to_string());

        cart.add_item(1, 2);
        cart.add_item(2, 3);

        cart.clear();
        assert_eq!(cart.items.len(), 0);
    }
}
