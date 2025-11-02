//! Shopping cart service implementation with thread-safe storage
//!
//! This module provides:
//! - Thread-safe cart operations using `Arc<Mutex<HashMap>>`
//! - Per-user cart isolation
//! - Product validation and inventory checking
//! - CRUD operations: add, remove, clear, get cart

use crate::catalog::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents a single item in a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    /// Product ID
    pub product_id: i32,
    /// Product name (cached from catalog)
    pub product_name: String,
    /// Unit price at time of adding to cart
    pub unit_price: Decimal,
    /// Quantity of this product in cart
    pub quantity: i32,
}

/// Represents a user's shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    /// Cart ID (auto-generated)
    pub id: i32,
    /// User ID who owns this cart
    pub user_id: i32,
    /// Items in the cart
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

    /// Returns the total number of items (sum of all quantities) in the cart
    #[must_use]
    pub fn item_count(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

/// Thread-safe shopping cart service with in-memory storage
#[derive(Debug, Clone)]
pub struct CartService {
    /// Carts indexed by `user_id`
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Next cart ID for new carts
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

    /// Gets or creates a cart for the specified user
    ///
    /// If the user doesn't have a cart yet, creates a new empty one.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get/create cart for
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should never happen in normal operation)
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");

        if let Some(cart) = carts.get(&user_id) {
            return cart.clone();
        }

        // Create new cart for user
        let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");
        let cart = Cart {
            id: *next_id,
            user_id,
            items: Vec::new(),
        };
        *next_id += 1;

        carts.insert(user_id, cart.clone());
        cart
    }

    /// Adds an item to the user's cart or increments quantity if already present
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID whose cart to modify
    /// * `product` - The product to add (used for validation and price info)
    /// * `quantity` - The quantity to add (must be positive)
    ///
    /// # Returns
    ///
    /// Returns the updated cart, or `None` if quantity is invalid or insufficient inventory
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Option<Cart> {
        if quantity <= 0 {
            return None;
        }

        // Check inventory
        if product.inventory_count < quantity {
            return None;
        }

        let mut carts = self.carts.lock().expect("Carts mutex poisoned");

        // Get or create cart
        let cart = if let Some(existing_cart) = carts.get_mut(&user_id) {
            existing_cart
        } else {
            let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");
            let new_cart = Cart {
                id: *next_id,
                user_id,
                items: Vec::new(),
            };
            *next_id += 1;
            carts.insert(user_id, new_cart);
            carts.get_mut(&user_id).expect("Just inserted cart")
        };

        // Check if product already in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            // Check if adding this quantity would exceed inventory
            let new_quantity = item.quantity + quantity;
            if new_quantity > product.inventory_count {
                return None;
            }
            item.quantity = new_quantity;
        } else {
            // Add new item
            cart.items.push(CartItem {
                product_id: product.id,
                product_name: product.name.clone(),
                unit_price: product.price,
                quantity,
            });
        }

        Some(cart.clone())
    }

    /// Removes an item completely from the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID whose cart to modify
    /// * `product_id` - The product ID to remove
    ///
    /// # Returns
    ///
    /// Returns the updated cart if found, or `None` if user has no cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");

        if let Some(cart) = carts.get_mut(&user_id) {
            cart.items.retain(|item| item.product_id != product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    /// Gets the user's cart without creating one if it doesn't exist
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID whose cart to retrieve
    ///
    /// # Returns
    ///
    /// Returns the cart if found, or `None` if user has no cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().expect("Carts mutex poisoned");
        carts.get(&user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// The cart itself remains but becomes empty.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID whose cart to clear
    ///
    /// # Returns
    ///
    /// Returns the emptied cart if found, or `None` if user has no cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");

        if let Some(cart) = carts.get_mut(&user_id) {
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
    use rust_decimal_macros::dec;
    use std::thread;

    fn create_test_product(id: i32, name: &str, price: Decimal, inventory: i32) -> Product {
        Product {
            id,
            name: name.to_string(),
            description: "Test product".to_string(),
            price,
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_get_or_create_cart() {
        let service = CartService::new();
        let user_id = 1;

        let cart = service.get_or_create_cart(user_id);
        assert_eq!(cart.user_id, user_id);
        assert_eq!(cart.id, 1);
        assert!(cart.items.is_empty());

        // Getting again should return same cart
        let cart2 = service.get_or_create_cart(user_id);
        assert_eq!(cart.id, cart2.id);
    }

    #[test]
    fn test_add_item_new_product() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 5);

        let cart = service.add_item(user_id, &product, 2);
        assert!(cart.is_some());

        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].unit_price, dec!(10.99));
    }

    #[test]
    fn test_add_item_increment_quantity() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let cart = service.add_item(user_id, &product, 3);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_item_insufficient_inventory() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 3);

        let cart = service.add_item(user_id, &product, 5);
        assert!(cart.is_none());
    }

    #[test]
    fn test_add_item_zero_quantity() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let cart = service.add_item(user_id, &product, 0);
        assert!(cart.is_none());
    }

    #[test]
    fn test_add_item_negative_quantity() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let cart = service.add_item(user_id, &product, -1);
        assert!(cart.is_none());
    }

    #[test]
    fn test_add_item_inventory_check_on_increment() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 5);

        let _ = service.add_item(user_id, &product, 3);
        let cart = service.add_item(user_id, &product, 3); // Would exceed inventory

        assert!(cart.is_none());
    }

    #[test]
    fn test_add_multiple_different_products() {
        let service = CartService::new();
        let user_id = 1;
        let product1 = create_test_product(1, "Product 1", dec!(10.99), 10);
        let product2 = create_test_product(2, "Product 2", dec!(20.99), 5);

        let _ = service.add_item(user_id, &product1, 2);
        let cart = service.add_item(user_id, &product2, 1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let cart = service.remove_item(user_id, product.id);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_remove_item_nonexistent_cart() {
        let service = CartService::new();
        let cart = service.remove_item(999, 1);
        assert!(cart.is_none());
    }

    #[test]
    fn test_remove_item_nonexistent_product() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let cart = service.remove_item(user_id, 999);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1); // Original item still there
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let cart = service.get_cart(user_id);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.user_id, user_id);
        assert_eq!(cart.items.len(), 1);
    }

    #[test]
    fn test_get_cart_nonexistent() {
        let service = CartService::new();
        let cart = service.get_cart(999);
        assert!(cart.is_none());
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product1 = create_test_product(1, "Product 1", dec!(10.99), 10);
        let product2 = create_test_product(2, "Product 2", dec!(20.99), 5);

        let _ = service.add_item(user_id, &product1, 2);
        let _ = service.add_item(user_id, &product2, 1);

        let cart = service.clear_cart(user_id);
        assert!(cart.is_some());

        let cart = cart.unwrap();
        assert!(cart.items.is_empty());
    }

    #[test]
    fn test_clear_cart_nonexistent() {
        let service = CartService::new();
        let cart = service.clear_cart(999);
        assert!(cart.is_none());
    }

    #[test]
    fn test_cart_isolation_per_user() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(10.99), 20);

        let _ = service.add_item(1, &product, 2);
        let _ = service.add_item(2, &product, 3);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.items[0].quantity, 2);
        assert_eq!(cart2.items[0].quantity, 3);
        assert_ne!(cart1.id, cart2.id);
    }

    #[test]
    fn test_cart_total_calculation() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![
                CartItem {
                    product_id: 1,
                    product_name: "Product 1".to_string(),
                    unit_price: dec!(10.00),
                    quantity: 2,
                },
                CartItem {
                    product_id: 2,
                    product_name: "Product 2".to_string(),
                    unit_price: dec!(5.50),
                    quantity: 3,
                },
            ],
        };

        let total = cart.total();
        assert_eq!(total, dec!(36.50)); // (10.00 * 2) + (5.50 * 3)
    }

    #[test]
    fn test_cart_item_count() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![
                CartItem {
                    product_id: 1,
                    product_name: "Product 1".to_string(),
                    unit_price: dec!(10.00),
                    quantity: 2,
                },
                CartItem {
                    product_id: 2,
                    product_name: "Product 2".to_string(),
                    unit_price: dec!(5.50),
                    quantity: 3,
                },
            ],
        };

        let count = cart.item_count();
        assert_eq!(count, 5); // 2 + 3
    }

    #[test]
    fn test_empty_cart_total() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![],
        };

        assert_eq!(cart.total(), dec!(0));
        assert_eq!(cart.item_count(), 0);
    }

    #[test]
    fn test_concurrent_cart_operations() {
        let service = CartService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let product1 = create_test_product(1, "Product 1", dec!(10.00), 100);
        let product2 = create_test_product(2, "Product 2", dec!(20.00), 100);

        let handle1 = thread::spawn(move || {
            for _ in 0..10 {
                let _ = service_clone1.add_item(1, &product1, 1);
            }
        });

        let handle2 = thread::spawn(move || {
            for _ in 0..10 {
                let _ = service_clone2.add_item(2, &product2, 1);
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.items[0].quantity, 10);
        assert_eq!(cart2.items[0].quantity, 10);
    }

    #[test]
    fn test_cart_auto_incrementing_ids() {
        let service = CartService::new();
        let product = create_test_product(1, "Test", dec!(10.00), 10);

        let _ = service.add_item(1, &product, 1);
        let _ = service.add_item(2, &product, 1);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.id, 1);
        assert_eq!(cart2.id, 2);
    }

    #[test]
    fn test_decimal_precision_in_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Precise Product", dec!(19.995), 10);

        let cart = service.add_item(user_id, &product, 2);
        assert!(cart.is_some());

        let cart = cart.unwrap();
        assert_eq!(cart.items[0].unit_price, dec!(19.995));
        assert_eq!(cart.total(), dec!(39.990));
    }
}
