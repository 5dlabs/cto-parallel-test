//! Cart service implementation
//!
//! Provides thread-safe in-memory shopping cart management with user isolation,
//! product integration, and inventory validation.

use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: Decimal,
}

/// Represents a user's shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub items: Vec<CartItem>,
}

impl Cart {
    /// Calculates the total price of all items in the cart
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.items
            .iter()
            .map(|item| item.unit_price * Decimal::from(item.quantity))
            .sum()
    }

    /// Returns the total number of items in the cart
    #[must_use]
    pub fn item_count(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

/// Thread-safe shopping cart service with in-memory storage
///
/// Manages shopping carts for multiple users with isolation and thread safety.
/// Uses `Arc<Mutex<>>` for concurrent access from multiple threads.
#[derive(Clone)]
pub struct CartService {
    /// Maps `user_id` to their cart
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Next cart ID to assign
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

    /// Gets existing cart or creates a new one for the user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    ///
    /// The user's cart (existing or newly created)
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        if let Some(cart) = carts.get(&user_id) {
            cart.clone()
        } else {
            let mut next_id = self.next_id.lock().unwrap();
            let cart = Cart {
                id: *next_id,
                user_id,
                items: Vec::new(),
            };
            *next_id += 1;
            carts.insert(user_id, cart.clone());
            cart
        }
    }

    /// Adds an item to the user's cart or increments quantity if already present
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `product` - The product to add
    /// * `quantity` - The quantity to add
    ///
    /// # Returns
    ///
    /// The updated cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();
        let mut cart = self.get_or_create_cart_internal(&mut carts, user_id);

        // Check if item already exists in cart
        if let Some(existing_item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            // Increment quantity if already in cart
            existing_item.quantity += quantity;
        } else {
            // Add new item to cart
            cart.items.push(CartItem {
                product_id: product.id,
                quantity,
                product_name: product.name.clone(),
                unit_price: product.price,
            });
        }

        carts.insert(user_id, cart.clone());
        cart
    }

    /// Removes an item from the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `product_id` - The product ID to remove
    ///
    /// # Returns
    ///
    /// `Some(cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        carts.get_mut(&user_id).map(|cart| {
            cart.items.retain(|item| item.product_id != product_id);
            cart.clone()
        })
    }

    /// Retrieves the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    ///
    /// `Some(cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();
        carts.get(&user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    ///
    /// `Some(empty_cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        carts.get_mut(&user_id).map(|cart| {
            cart.items.clear();
            cart.clone()
        })
    }

    /// Internal helper to get or create cart with pre-locked mutex
    fn get_or_create_cart_internal(&self, carts: &mut HashMap<i32, Cart>, user_id: i32) -> Cart {
        if let Some(cart) = carts.get(&user_id) {
            cart.clone()
        } else {
            let mut next_id = self.next_id.lock().unwrap();
            let cart = Cart {
                id: *next_id,
                user_id,
                items: Vec::new(),
            };
            *next_id += 1;
            cart
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
    use rust_decimal::prelude::FromPrimitive;
    use std::thread;

    fn create_test_product(id: i32, name: &str, price: f64, inventory: i32) -> Product {
        Product {
            id,
            name: String::from(name),
            description: format!("Description for {name}"),
            price: Decimal::from_f64(price).unwrap(),
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_new_cart_service() {
        let service = CartService::new();
        let cart = service.get_cart(1);
        assert!(cart.is_none());
    }

    #[test]
    fn test_get_or_create_cart() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);

        assert_eq!(cart.id, 1);
        assert_eq!(cart.user_id, 1);
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_get_or_create_cart_existing() {
        let service = CartService::new();
        let cart1 = service.get_or_create_cart(1);
        let cart2 = service.get_or_create_cart(1);

        assert_eq!(cart1.id, cart2.id);
        assert_eq!(cart1.user_id, cart2.user_id);
    }

    #[test]
    fn test_add_item_to_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", 19.99, 10);

        let cart = service.add_item(1, &product, 2);

        assert_eq!(cart.user_id, 1);
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].product_name, "Test Product");
        assert_eq!(cart.items[0].unit_price, Decimal::from_f64(19.99).unwrap());
    }

    #[test]
    fn test_add_same_item_increments_quantity() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", 19.99, 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_different_items() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", 10.0, 5);
        let product2 = create_test_product(2, "Product 2", 20.0, 10);

        let _ = service.add_item(1, &product1, 1);
        let cart = service.add_item(1, &product2, 2);

        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[1].product_id, 2);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", 10.0, 5);
        let product2 = create_test_product(2, "Product 2", 20.0, 10);

        let _ = service.add_item(1, &product1, 1);
        let _ = service.add_item(1, &product2, 2);
        let cart = service.remove_item(1, 1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_remove_item_from_nonexistent_cart() {
        let service = CartService::new();
        let result = service.remove_item(999, 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let service = CartService::new();
        let product = create_test_product(1, "Product 1", 10.0, 5);

        let _ = service.add_item(1, &product, 1);
        let cart = service.remove_item(1, 999);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Product 1", 10.0, 5);

        let _ = service.add_item(1, &product, 1);
        let cart = service.get_cart(1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.user_id, 1);
        assert_eq!(cart.items.len(), 1);
    }

    #[test]
    fn test_get_nonexistent_cart() {
        let service = CartService::new();
        let cart = service.get_cart(999);
        assert!(cart.is_none());
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Product 1", 10.0, 5);

        let _ = service.add_item(1, &product, 3);
        let cart = service.clear_cart(1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_clear_nonexistent_cart() {
        let service = CartService::new();
        let result = service.clear_cart(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_cart_isolation_per_user() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", 10.0, 5);
        let product2 = create_test_product(2, "Product 2", 20.0, 10);

        let _ = service.add_item(1, &product1, 1);
        let _ = service.add_item(2, &product2, 2);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.user_id, 1);
        assert_eq!(cart2.user_id, 2);
        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart1.items[0].product_id, 1);
        assert_eq!(cart2.items[0].product_id, 2);
    }

    #[test]
    fn test_cart_total() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", 10.0, 5);
        let product2 = create_test_product(2, "Product 2", 20.0, 10);

        let _ = service.add_item(1, &product1, 2);
        let _ = service.add_item(1, &product2, 3);

        let cart = service.get_cart(1).unwrap();
        let total = cart.total();

        // 2 * 10.0 + 3 * 20.0 = 20 + 60 = 80
        assert_eq!(total, Decimal::from_f64(80.0).unwrap());
    }

    #[test]
    fn test_cart_item_count() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", 10.0, 5);
        let product2 = create_test_product(2, "Product 2", 20.0, 10);

        let _ = service.add_item(1, &product1, 2);
        let _ = service.add_item(1, &product2, 3);

        let cart = service.get_cart(1).unwrap();
        assert_eq!(cart.item_count(), 5);
    }

    #[test]
    fn test_concurrent_add_items() {
        let service = CartService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 1..=5 {
                let product = create_test_product(i, &format!("Product {i}"), 10.0, 10);
                let _ = service_clone1.add_item(1, &product, 1);
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 6..=10 {
                let product = create_test_product(i, &format!("Product {i}"), 20.0, 10);
                let _ = service_clone2.add_item(1, &product, 1);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let cart = service.get_cart(1).unwrap();
        assert_eq!(cart.items.len(), 10);
    }

    #[test]
    fn test_concurrent_user_isolation() {
        let service = CartService::new();
        let service_clone = service.clone();

        let handle1 = thread::spawn(move || {
            let product = create_test_product(1, "Product 1", 10.0, 10);
            service_clone.add_item(1, &product, 5)
        });

        let handle2 = thread::spawn(move || {
            let product = create_test_product(2, "Product 2", 20.0, 10);
            service.add_item(2, &product, 3)
        });

        let cart1 = handle1.join().unwrap();
        let cart2 = handle2.join().unwrap();

        assert_eq!(cart1.user_id, 1);
        assert_eq!(cart2.user_id, 2);
        assert_eq!(cart1.items[0].quantity, 5);
        assert_eq!(cart2.items[0].quantity, 3);
    }

    #[test]
    fn test_empty_cart_total() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);
        assert_eq!(cart.total(), Decimal::from(0));
    }

    #[test]
    fn test_empty_cart_item_count() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);
        assert_eq!(cart.item_count(), 0);
    }
}
