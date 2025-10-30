use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: Decimal,
}

/// Represents a shopping cart for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub items: Vec<CartItem>,
}

/// Thread-safe shopping cart service
pub struct CartService {
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    next_id: Arc<Mutex<i32>>,
}

impl CartService {
    /// Creates a new empty cart service
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Gets or creates a cart for the given user
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// The user's cart (existing or newly created)
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        // Search for existing cart by user_id
        if let Some(cart) = carts.values().find(|c| c.user_id == user_id) {
            return cart.clone();
        }

        // Create new cart if none exists
        let mut next_id = self.next_id.lock().unwrap();
        let cart = Cart {
            id: *next_id,
            user_id,
            items: Vec::new(),
        };
        *next_id += 1;

        carts.insert(cart.id, cart.clone());
        cart
    }

    /// Adds an item to the user's cart or updates quantity if already present
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `product` - The product to add
    /// * `quantity` - The quantity to add
    ///
    /// # Returns
    /// The updated cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        // Find the cart by user_id
        let cart = carts
            .values_mut()
            .find(|c| c.user_id == user_id)
            .expect("Cart should exist");

        // Check if product already in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            // Update quantity
            item.quantity += quantity;
        } else {
            // Add new item
            cart.items.push(CartItem {
                product_id: product.id,
                quantity,
                product_name: product.name.clone(),
                unit_price: product.price,
            });
        }

        cart.clone()
    }

    /// Removes an item from the user's cart
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `product_id` - The ID of the product to remove
    ///
    /// # Returns
    /// The updated cart, or None if the cart doesn't exist
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        // Find the cart by user_id
        if let Some(cart) = carts.values_mut().find(|c| c.user_id == user_id) {
            cart.items.retain(|item| item.product_id != product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    /// Gets the user's cart
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// The user's cart, or None if it doesn't exist
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();
        carts.values().find(|c| c.user_id == user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// The cleared cart, or None if the cart doesn't exist
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        // Find the cart by user_id
        if let Some(cart) = carts.values_mut().find(|c| c.user_id == user_id) {
            cart.items.clear();
            Some(cart.clone())
        } else {
            None
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
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn create_test_product(id: i32, name: &str, price: &str, inventory: i32) -> Product {
        Product {
            id,
            name: name.to_string(),
            description: "Test product".to_string(),
            price: Decimal::from_str(price).unwrap(),
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_get_or_create_cart() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);

        assert_eq!(cart.user_id, 1);
        assert_eq!(cart.id, 1);
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_get_or_create_cart_returns_existing() {
        let service = CartService::new();
        let cart1 = service.get_or_create_cart(1);
        let cart2 = service.get_or_create_cart(1);

        assert_eq!(cart1.id, cart2.id);
    }

    #[test]
    fn test_add_item() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let product = create_test_product(1, "Test Product", "10.00", 5);
        let cart = service.add_item(1, &product, 2);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].product_name, "Test Product");
        assert_eq!(
            cart.items[0].unit_price,
            Decimal::from_str("10.00").unwrap()
        );
    }

    #[test]
    fn test_add_item_increments_quantity() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let product = create_test_product(1, "Test Product", "10.00", 5);
        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let product = create_test_product(1, "Test Product", "10.00", 5);
        let _ = service.add_item(1, &product, 2);

        let cart = service.remove_item(1, 1);
        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let cart = service.get_cart(1);
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().user_id, 1);

        let no_cart = service.get_cart(999);
        assert!(no_cart.is_none());
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let product = create_test_product(1, "Test Product", "10.00", 5);
        let _ = service.add_item(1, &product, 2);

        let cart = service.clear_cart(1);
        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_multiple_users() {
        let service = CartService::new();
        let cart1 = service.get_or_create_cart(1);
        let cart2 = service.get_or_create_cart(2);

        assert_ne!(cart1.id, cart2.id);
        assert_eq!(cart1.user_id, 1);
        assert_eq!(cart2.user_id, 2);
    }
}
