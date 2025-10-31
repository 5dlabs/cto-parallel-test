use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

/// Thread-safe in-memory shopping cart service
#[derive(Debug, Clone)]
pub struct CartService {
    carts: Arc<Mutex<Vec<Cart>>>,
}

impl CartService {
    /// Creates a new empty cart service
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Gets or creates a cart for the specified user
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: &str) -> Cart {
        let mut carts = self.carts.lock().expect("Mutex poisoned");

        if let Some(cart) = carts.iter().find(|c| c.user_id == user_id) {
            cart.clone()
        } else {
            let cart = Cart {
                user_id: user_id.to_string(),
                items: Vec::new(),
            };
            carts.push(cart.clone());
            cart
        }
    }

    /// Adds an item to the user's cart or updates quantity if it already exists
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn add_item(&self, user_id: &str, product_id: i32, quantity: i32) {
        let mut carts = self.carts.lock().expect("Mutex poisoned");

        // Find existing cart or create a new one
        let cart_exists = carts.iter().any(|c| c.user_id == user_id);

        if !cart_exists {
            carts.push(Cart {
                user_id: user_id.to_string(),
                items: Vec::new(),
            });
        }

        // Now safely get mutable reference to the cart
        let cart = carts
            .iter_mut()
            .find(|c| c.user_id == user_id)
            .expect("Cart should exist");

        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                product_id,
                quantity,
            });
        }
    }

    /// Removes an item from the user's cart
    ///
    /// Returns `true` if the item was found and removed, `false` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn remove_item(&self, user_id: &str, product_id: i32) -> bool {
        let mut carts = self.carts.lock().expect("Mutex poisoned");

        if let Some(cart) = carts.iter_mut().find(|c| c.user_id == user_id) {
            let initial_len = cart.items.len();
            cart.items.retain(|item| item.product_id != product_id);
            cart.items.len() < initial_len
        } else {
            false
        }
    }

    /// Gets the cart for a specific user
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Option<Cart> {
        let carts = self.carts.lock().expect("Mutex poisoned");
        carts.iter().find(|c| c.user_id == user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn clear_cart(&self, user_id: &str) {
        let mut carts = self.carts.lock().expect("Mutex poisoned");
        if let Some(cart) = carts.iter_mut().find(|c| c.user_id == user_id) {
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
    fn test_get_or_create_cart() {
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
    fn test_add_existing_item_increases_quantity() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 1, 3);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_different_items() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 1);

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 1);

        assert!(service.remove_item("user1", 1));

        let cart = service.get_cart("user1").unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);

        assert!(!service.remove_item("user1", 999));
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        service.add_item("user1", 1, 2);
        service.add_item("user1", 2, 1);

        service.clear_cart("user1");

        let cart = service.get_cart("user1").unwrap();
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_separate_user_carts() {
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
