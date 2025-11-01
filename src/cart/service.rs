//! Shopping cart service with thread-safe in-memory storage.

use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    /// Product ID
    pub product_id: i32,
    /// Product name (cached for display)
    pub product_name: String,
    /// Quantity of this product in cart
    pub quantity: i32,
    /// Unit price (cached from product at time of addition)
    pub unit_price: Decimal,
}

/// Represents a user's shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    /// Cart ID
    pub id: i32,
    /// User ID who owns this cart
    pub user_id: i32,
    /// Items in the cart
    pub items: Vec<CartItem>,
}

impl Cart {
    /// Calculate the total price of all items in the cart
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.items
            .iter()
            .map(|item| item.unit_price * Decimal::from(item.quantity))
            .sum()
    }
}

/// Thread-safe shopping cart service
///
/// Provides cart management with:
/// - User-specific cart isolation
/// - Inventory validation before adding items
/// - Thread-safe concurrent access
/// - In-memory storage with Arc<Mutex>
#[derive(Clone)]
pub struct CartService {
    /// Thread-safe storage for carts (keyed by `user_id`)
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Auto-incrementing cart ID counter
    next_id: Arc<Mutex<i32>>,
}

impl CartService {
    /// Creates a new empty cart service.
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Gets or creates a cart for the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// The user's cart, creating a new empty cart if one doesn't exist
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        if let Some(cart) = carts.get(&user_id) {
            return cart.clone();
        }

        // Create new cart
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

    /// Adds an item to the user's cart or updates quantity if already present.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    /// * `product` - The product to add
    /// * `quantity` - The quantity to add (must be positive)
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with the updated cart if successful, `None` if quantity is invalid or insufficient inventory
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Option<Cart> {
        // Validate quantity
        if quantity <= 0 {
            return None;
        }

        // Validate inventory
        if product.inventory_count < quantity {
            return None;
        }

        let mut carts = self.carts.lock().unwrap();

        // Get or create cart
        let mut next_id = self.next_id.lock().unwrap();
        let cart = carts.entry(user_id).or_insert_with(|| {
            let new_cart = Cart {
                id: *next_id,
                user_id,
                items: Vec::new(),
            };
            *next_id += 1;
            new_cart
        });

        // Check if product already in cart
        if let Some(existing_item) = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product.id)
        {
            // Check if adding more would exceed inventory
            let new_quantity = existing_item.quantity + quantity;
            if new_quantity > product.inventory_count {
                return None;
            }
            existing_item.quantity = new_quantity;
        } else {
            // Add new item
            cart.items.push(CartItem {
                product_id: product.id,
                product_name: product.name.clone(),
                quantity,
                unit_price: product.price,
            });
        }

        Some(cart.clone())
    }

    /// Removes an item from the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    /// * `product_id` - The ID of the product to remove
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with the updated cart if the item was removed, `None` if cart or item not found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        if let Some(cart) = carts.get_mut(&user_id) {
            let initial_len = cart.items.len();
            cart.items.retain(|item| item.product_id != product_id);

            if cart.items.len() < initial_len {
                return Some(cart.clone());
            }
        }

        None
    }

    /// Retrieves the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// `Some(Cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();
        carts.get(&user_id).cloned()
    }

    /// Clears all items from the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with the empty cart if the cart existed, `None` if cart not found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().unwrap();

        if let Some(cart) = carts.get_mut(&user_id) {
            cart.items.clear();
            return Some(cart.clone());
        }

        None
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
            description: format!("Description for {name}"),
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
        assert_eq!(cart.items.len(), 0);

        // Getting again should return same cart
        let cart2 = service.get_or_create_cart(user_id);
        assert_eq!(cart.id, cart2.id);
    }

    #[test]
    fn test_add_item() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let result = service.add_item(user_id, &product, 2);
        assert!(result.is_some());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].unit_price, dec!(999.99));
    }

    #[test]
    fn test_add_item_increments_quantity() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let result = service.add_item(user_id, &product, 3);

        assert!(result.is_some());
        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_item_insufficient_inventory() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 5);

        // Try to add more than available
        let result = service.add_item(user_id, &product, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_add_item_exceeds_inventory_on_increment() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 5);

        // Add 3 items (ok)
        let result = service.add_item(user_id, &product, 3);
        assert!(result.is_some());

        // Try to add 3 more (would total 6, exceeds 5 available)
        let result = service.add_item(user_id, &product, 3);
        assert!(result.is_none());

        // Verify quantity unchanged
        let cart = service.get_cart(user_id).unwrap();
        assert_eq!(cart.items[0].quantity, 3);
    }

    #[test]
    fn test_add_item_invalid_quantity() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        // Zero quantity
        let result = service.add_item(user_id, &product, 0);
        assert!(result.is_none());

        // Negative quantity
        let result = service.add_item(user_id, &product, -5);
        assert!(result.is_none());
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(user_id, &product, 2);
        let result = service.remove_item(user_id, 1);

        assert!(result.is_some());
        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_remove_item_not_found() {
        let service = CartService::new();
        let user_id = 1;

        // Create cart first
        let _ = service.get_or_create_cart(user_id);

        // Try to remove non-existent item
        let result = service.remove_item(user_id, 999);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        // No cart initially
        assert!(service.get_cart(user_id).is_none());

        // Add item (creates cart)
        let _ = service.add_item(user_id, &product, 2);

        // Now cart exists
        let cart = service.get_cart(user_id);
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().items.len(), 1);
    }

    #[test]
    fn test_clear_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product1 = create_test_product(1, "Laptop", dec!(999.99), 10);
        let product2 = create_test_product(2, "Mouse", dec!(29.99), 20);

        let _ = service.add_item(user_id, &product1, 2);
        let _ = service.add_item(user_id, &product2, 1);

        let result = service.clear_cart(user_id);
        assert!(result.is_some());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_clear_cart_not_found() {
        let service = CartService::new();
        let result = service.clear_cart(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_cart_isolation() {
        let service = CartService::new();
        let user1 = 1;
        let user2 = 2;
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        // User 1 adds item
        let _ = service.add_item(user1, &product, 2);

        // User 2 adds item
        let _ = service.add_item(user2, &product, 3);

        // Each user has their own cart
        let cart1 = service.get_cart(user1).unwrap();
        let cart2 = service.get_cart(user2).unwrap();

        assert_ne!(cart1.id, cart2.id);
        assert_eq!(cart1.items[0].quantity, 2);
        assert_eq!(cart2.items[0].quantity, 3);
    }

    #[test]
    fn test_cart_total() {
        let service = CartService::new();
        let user_id = 1;
        let product1 = create_test_product(1, "Laptop", dec!(999.99), 10);
        let product2 = create_test_product(2, "Mouse", dec!(29.99), 20);

        let _ = service.add_item(user_id, &product1, 2); // 2 * 999.99 = 1999.98
        let _ = service.add_item(user_id, &product2, 3); // 3 * 29.99 = 89.97

        let cart = service.get_cart(user_id).unwrap();
        let total = cart.total();

        assert_eq!(total, dec!(2089.95)); // 1999.98 + 89.97
    }

    #[test]
    fn test_concurrent_cart_creation() {
        let service = Arc::new(CartService::new());
        let mut handles = vec![];

        for user_id in 1..=10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || service_clone.get_or_create_cart(user_id));
            handles.push(handle);
        }

        let mut carts = vec![];
        for handle in handles {
            carts.push(handle.join().unwrap());
        }

        // All carts should have unique IDs
        let mut ids: Vec<i32> = carts.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn test_concurrent_add_remove() {
        let service = Arc::new(CartService::new());
        let user_id = 1;
        let product = create_test_product(1, "Test Product", dec!(10.00), 100);

        let mut handles = vec![];

        // Spawn multiple threads adding items
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let product_clone = product.clone();
            let handle = thread::spawn(move || service_clone.add_item(user_id, &product_clone, 1));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let cart = service.get_cart(user_id).unwrap();
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_multiple_products_in_cart() {
        let service = CartService::new();
        let user_id = 1;
        let product1 = create_test_product(1, "Laptop", dec!(999.99), 10);
        let product2 = create_test_product(2, "Mouse", dec!(29.99), 20);
        let product3 = create_test_product(3, "Keyboard", dec!(79.99), 15);

        let _ = service.add_item(user_id, &product1, 1);
        let _ = service.add_item(user_id, &product2, 2);
        let _ = service.add_item(user_id, &product3, 1);

        let cart = service.get_cart(user_id).unwrap();
        assert_eq!(cart.items.len(), 3);
    }
}
