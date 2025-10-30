use crate::cart::models::{Cart, CartItem};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory shopping cart service
///
/// Manages shopping carts for users with concurrent access support.
pub struct CartService {
    carts: Arc<Mutex<HashMap<String, Cart>>>,
}

impl CartService {
    /// Creates a new `CartService` with no carts.
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Gets or creates a cart for the specified user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get/create cart for
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: &str) -> Cart {
        let mut carts = self.carts.lock().expect("carts mutex poisoned");

        carts
            .entry(user_id.to_string())
            .or_insert_with(|| Cart {
                user_id: user_id.to_string(),
                items: Vec::new(),
            })
            .clone()
    }

    /// Adds an item to a user's cart or updates quantity if item exists
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `product_id` - The product ID to add
    /// * `quantity` - The quantity to add
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    #[must_use]
    pub fn add_item(&self, user_id: &str, product_id: i32, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().expect("carts mutex poisoned");

        let cart = carts.entry(user_id.to_string()).or_insert_with(|| Cart {
            user_id: user_id.to_string(),
            items: Vec::new(),
        });

        // Check if item already exists in cart
        if let Some(existing_item) = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
        {
            existing_item.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                product_id,
                quantity,
            });
        }

        cart.clone()
    }

    /// Gets a user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// * `Some(Cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Option<Cart> {
        let carts = self.carts.lock().expect("carts mutex poisoned");
        carts.get(user_id).cloned()
    }

    /// Removes an item from a user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `product_id` - The product ID to remove
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    #[must_use]
    pub fn remove_item(&self, user_id: &str, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("carts mutex poisoned");

        if let Some(cart) = carts.get_mut(user_id) {
            cart.items.retain(|item| item.product_id != product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    /// Clears all items from a user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    pub fn clear_cart(&self, user_id: &str) {
        let mut carts = self.carts.lock().expect("carts mutex poisoned");
        carts.remove(user_id);
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
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_add_item() {
        let service = CartService::new();

        let cart = service.add_item("user1", 1, 2);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);

        // Add same item again
        let cart = service.add_item("user1", 1, 3);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);

        // Add different item
        let cart = service.add_item("user1", 2, 1);
        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();

        // Non-existent cart
        assert!(service.get_cart("user1").is_none());

        // Create cart and retrieve
        let _ = service.add_item("user1", 1, 2);
        let cart = service.get_cart("user1");
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().items.len(), 1);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();

        let _ = service.add_item("user1", 1, 2);
        let _ = service.add_item("user1", 2, 3);

        let cart = service.remove_item("user1", 1);
        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();

        let _ = service.add_item("user1", 1, 2);
        let _ = service.add_item("user1", 2, 3);

        service.clear_cart("user1");
        assert!(service.get_cart("user1").is_none());
    }
}
