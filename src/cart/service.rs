use crate::cart::models::{Cart, CartItem};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory cart service
pub struct CartService {
    carts: Arc<Mutex<HashMap<String, Cart>>>,
}

impl CartService {
    /// Creates a new empty cart service
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Gets or creates a cart for a user
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: &str) -> Cart {
        let mut carts = self.carts.lock().expect("Mutex poisoned");
        carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart::new(user_id.to_string()))
            .clone()
    }

    /// Adds an item to the cart or updates quantity if it already exists
    ///
    /// # Errors
    ///
    /// Returns an error if quantity is invalid (<=0)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    pub fn add_item(&self, user_id: &str, product_id: i32, quantity: i32) -> Result<Cart, String> {
        if quantity <= 0 {
            return Err("Quantity must be positive".to_string());
        }

        let mut carts = self.carts.lock().expect("Mutex poisoned");
        let cart = carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart::new(user_id.to_string()));

        // Check if item already exists in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                product_id,
                quantity,
            });
        }

        Ok(cart.clone())
    }

    /// Removes an item from the cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn remove_item(&self, user_id: &str, product_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Mutex poisoned");
        if let Some(cart) = carts.get_mut(user_id) {
            cart.items.retain(|item| item.product_id != product_id);
            cart.clone()
        } else {
            Cart::new(user_id.to_string())
        }
    }

    /// Gets the user's cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Cart {
        let carts = self.carts.lock().expect("Mutex poisoned");
        carts
            .get(user_id)
            .cloned()
            .unwrap_or_else(|| Cart::new(user_id.to_string()))
    }

    /// Clears all items from the user's cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    pub fn clear_cart(&self, user_id: &str) {
        let mut carts = self.carts.lock().expect("Mutex poisoned");
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
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_add_item() {
        let service = CartService::new();

        let result = service.add_item("user1", 1, 2);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
    }

    #[test]
    fn test_add_same_item_increases_quantity() {
        let service = CartService::new();

        service.add_item("user1", 1, 2).unwrap();
        let cart = service.add_item("user1", 1, 3).unwrap();

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();

        service.add_item("user1", 1, 2).unwrap();
        service.add_item("user1", 2, 1).unwrap();

        let cart = service.remove_item("user1", 1);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();

        service.add_item("user1", 1, 2).unwrap();
        service.add_item("user1", 2, 1).unwrap();

        service.clear_cart("user1");
        let cart = service.get_cart("user1");

        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_invalid_quantity() {
        let service = CartService::new();

        let result = service.add_item("user1", 1, 0);
        assert!(result.is_err());

        let result = service.add_item("user1", 1, -1);
        assert!(result.is_err());
    }
}
