use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents a single item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: Decimal,
}

/// Represents a user's shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub items: Vec<CartItem>,
}

/// Thread-safe service for managing shopping carts
pub struct CartService {
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    next_id: Arc<Mutex<i32>>,
}

impl CartService {
    /// Creates a new `CartService` with empty cart storage
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Gets or creates a cart for the specified user
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Lock poisoned");

        // Find existing cart for this user
        for cart in carts.values() {
            if cart.user_id == user_id {
                return cart.clone();
            }
        }

        // Create new cart if none exists
        let mut next_id = self.next_id.lock().expect("Lock poisoned");
        let cart_id = *next_id;
        *next_id += 1;

        let new_cart = Cart {
            id: cart_id,
            user_id,
            items: Vec::new(),
        };

        carts.insert(cart_id, new_cart.clone());
        new_cart
    }

    /// Adds an item to the user's cart or increments quantity if already present
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let cart = self.get_or_create_cart(user_id);
        let mut carts = self.carts.lock().expect("Lock poisoned");

        if let Some(existing_cart) = carts.get_mut(&cart.id) {
            // Check if product already in cart
            if let Some(item) = existing_cart
                .items
                .iter_mut()
                .find(|item| item.product_id == product.id)
            {
                // Increment quantity
                item.quantity += quantity;
            } else {
                // Add new item
                existing_cart.items.push(CartItem {
                    product_id: product.id,
                    quantity,
                    product_name: product.name.clone(),
                    unit_price: product.price,
                });
            }

            existing_cart.clone()
        } else {
            cart
        }
    }

    /// Removes an item from the user's cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Lock poisoned");

        // Find user's cart
        let cart = carts.values().find(|c| c.user_id == user_id)?;
        let cart_id = cart.id;

        // Remove item from cart
        if let Some(cart) = carts.get_mut(&cart_id) {
            cart.items.retain(|item| item.product_id != product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    /// Gets the user's cart if it exists
    ///
    /// Used in tests and available as public API for future integration
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    #[allow(dead_code)] // Used in tests, available for future API endpoints
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().expect("Lock poisoned");
        carts.values().find(|c| c.user_id == user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Lock poisoned");

        // Find user's cart
        let cart = carts.values().find(|c| c.user_id == user_id)?;
        let cart_id = cart.id;

        // Clear items
        if let Some(cart) = carts.get_mut(&cart_id) {
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

    #[test]
    fn test_create_new_cart_service() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);
        assert_eq!(cart.user_id, 1);
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
    fn test_add_item_to_cart() {
        let service = CartService::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        };

        let cart = service.add_item(1, &product, 2);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
    }

    #[test]
    fn test_add_existing_item_increments_quantity() {
        let service = CartService::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        };

        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_remove_item_from_cart() {
        let service = CartService::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        };

        let _ = service.add_item(1, &product, 2);
        let cart = service.remove_item(1, 1);

        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        let product1 = Product {
            id: 1,
            name: "Product 1".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        };
        let product2 = Product {
            id: 2,
            name: "Product 2".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(2999, 2),
            inventory_count: 5,
        };

        let _ = service.add_item(1, &product1, 2);
        let _ = service.add_item(1, &product2, 1);

        let cart = service.clear_cart(1);
        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_multiple_users_have_separate_carts() {
        let service = CartService::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        };

        let _ = service.add_item(1, &product, 2);
        let _ = service.add_item(2, &product, 3);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.user_id, 1);
        assert_eq!(cart2.user_id, 2);
        assert_eq!(cart1.items[0].quantity, 2);
        assert_eq!(cart2.items[0].quantity, 3);
    }
}
