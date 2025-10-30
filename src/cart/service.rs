use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

pub struct CartService {
    carts: Arc<Mutex<HashMap<String, Cart>>>,
}

impl CartService {
    /// Creates a new `CartService` with empty cart storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Gets or creates a cart for the given user.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: &str) -> Cart {
        let mut carts = self.carts.lock().unwrap();
        carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart {
                user_id: user_id.to_string(),
                items: Vec::new(),
            })
            .clone()
    }

    /// Adds an item to the user's cart.
    ///
    /// If the item already exists, increases the quantity.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn add_item(&self, user_id: &str, product_id: i32, quantity: i32) {
        let mut carts = self.carts.lock().unwrap();
        let cart = carts.entry(user_id.to_string()).or_insert_with(|| Cart {
            user_id: user_id.to_string(),
            items: Vec::new(),
        });

        if let Some(item) = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
        {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                product_id,
                quantity,
            });
        }
    }

    /// Removes an item from the user's cart.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn remove_item(&self, user_id: &str, product_id: i32) {
        let mut carts = self.carts.lock().unwrap();
        if let Some(cart) = carts.get_mut(user_id) {
            cart.items.retain(|item| item.product_id != product_id);
        }
    }

    /// Gets the cart for a user.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();
        carts.get(user_id).cloned()
    }

    /// Clears all items from a user's cart.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn clear_cart(&self, user_id: &str) {
        let mut carts = self.carts.lock().unwrap();
        if let Some(cart) = carts.get_mut(user_id) {
            cart.items.clear();
        }
    }
}

impl Default for CartService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_creation() {
        let service = CartService::new();
        let cart = service.get_or_create_cart("user1");
        assert_eq!(cart.user_id, "user1");
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_add_item_to_cart() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
    }

    #[test]
    fn test_add_multiple_items() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 3);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_add_same_item_increases_quantity() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 1, 3);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_remove_item_from_cart() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 3);
        service.remove_item("user1", 1);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 3);
        service.clear_cart("user1");

        let cart = service.get_cart("user1").unwrap();
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_different_user_carts_are_isolated() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user2", 2, 3);

        let cart1 = service.get_cart("user1").unwrap();
        let cart2 = service.get_cart("user2").unwrap();

        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart1.items[0].product_id, 1);

        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart2.items[0].product_id, 2);
    }
}
