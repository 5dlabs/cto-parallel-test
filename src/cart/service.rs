use crate::cart::models::Cart;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Thread-safe shopping cart service
///
/// Manages shopping carts for multiple users with concurrent access support.
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
    /// # Arguments
    /// * `user_id` - The user ID to get the cart for
    ///
    /// # Returns
    /// The user's cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Cart {
        let mut carts = self.carts.lock().expect("Cart lock poisoned");
        carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart::new(user_id.to_string()))
            .clone()
    }

    /// Adds a product to a user's cart
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    /// * `product_id` - The product ID to add
    /// * `quantity` - The quantity to add
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    pub fn add_to_cart(&self, user_id: &str, product_id: i32, quantity: i32) {
        let mut carts = self.carts.lock().expect("Cart lock poisoned");
        let cart = carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart::new(user_id.to_string()));
        cart.add_item(product_id, quantity);
    }

    /// Removes a product from a user's cart
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    /// * `product_id` - The product ID to remove
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    pub fn remove_from_cart(&self, user_id: &str, product_id: i32) {
        let mut carts = self.carts.lock().expect("Cart lock poisoned");
        if let Some(cart) = carts.get_mut(user_id) {
            cart.remove_item(product_id);
        }
    }

    /// Clears a user's cart
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    pub fn clear_cart(&self, user_id: &str) {
        let mut carts = self.carts.lock().expect("Cart lock poisoned");
        if let Some(cart) = carts.get_mut(user_id) {
            cart.clear();
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
    fn test_cart_service_creation() {
        let service = CartService::new();
        let cart = service.get_cart("user_123");
        assert_eq!(cart.user_id, "user_123");
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_add_to_cart() {
        let service = CartService::new();

        service.add_to_cart("user_123", 1, 2);
        service.add_to_cart("user_123", 2, 3);

        let cart = service.get_cart("user_123");
        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[1].product_id, 2);
        assert_eq!(cart.items[1].quantity, 3);
    }

    #[test]
    fn test_remove_from_cart() {
        let service = CartService::new();

        service.add_to_cart("user_123", 1, 2);
        service.add_to_cart("user_123", 2, 3);
        service.remove_from_cart("user_123", 1);

        let cart = service.get_cart("user_123");
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();

        service.add_to_cart("user_123", 1, 2);
        service.add_to_cart("user_123", 2, 3);
        service.clear_cart("user_123");

        let cart = service.get_cart("user_123");
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_multiple_users() {
        let service = CartService::new();

        service.add_to_cart("user_1", 1, 2);
        service.add_to_cart("user_2", 2, 3);

        let cart1 = service.get_cart("user_1");
        let cart2 = service.get_cart("user_2");

        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart1.items[0].product_id, 1);

        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart2.items[0].product_id, 2);
    }
}
