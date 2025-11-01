use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub unit_price: Decimal,
}

/// Represents a shopping cart for a user
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

    /// Returns the number of items in the cart
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Returns the total quantity of all items
    #[must_use]
    pub fn total_quantity(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

/// Thread-safe shopping cart service with in-memory storage
#[derive(Clone)]
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

    /// Gets an existing cart for a user or creates a new one
    ///
    /// # Arguments
    /// * `user_id` - The user ID to get or create a cart for
    ///
    /// # Returns
    /// The user's cart (existing or newly created)
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Carts lock poisoned");

        // Find existing cart for user
        if let Some(cart) = carts.values().find(|c| c.user_id == user_id) {
            return cart.clone();
        }

        // Create new cart
        let mut next_id = self.next_id.lock().expect("Next ID lock poisoned");
        let cart = Cart {
            id: *next_id,
            user_id,
            items: Vec::new(),
        };
        *next_id += 1;
        carts.insert(cart.id, cart.clone());
        cart
    }

    /// Adds an item to the user's cart or increments quantity if already present
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    /// * `product` - The product to add
    /// * `quantity` - The quantity to add
    ///
    /// # Returns
    /// The updated cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Carts lock poisoned");

        // Get or create cart for user
        let cart_id = if let Some(cart) = carts.values().find(|c| c.user_id == user_id) {
            cart.id
        } else {
            // Create new cart
            let mut next_id = self.next_id.lock().expect("Next ID lock poisoned");
            let new_id = *next_id;
            *next_id += 1;
            let new_cart = Cart {
                id: new_id,
                user_id,
                items: Vec::new(),
            };
            carts.insert(new_id, new_cart);
            new_id
        };

        // Get mutable reference to cart
        let cart = carts.get_mut(&cart_id).expect("Cart should exist");

        // Check if product already in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            // Increment quantity
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
    /// * `user_id` - The user ID
    /// * `product_id` - The product ID to remove
    ///
    /// # Returns
    /// `Some(Cart)` if the cart exists, `None` if the user has no cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Carts lock poisoned");

        // Find cart for user
        let cart_id = carts.values().find(|c| c.user_id == user_id)?.id;

        // Remove item from cart
        let cart = carts.get_mut(&cart_id)?;
        cart.items.retain(|item| item.product_id != product_id);

        Some(cart.clone())
    }

    /// Gets the user's cart
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    ///
    /// # Returns
    /// `Some(Cart)` if the cart exists, `None` if the user has no cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().expect("Carts lock poisoned");
        carts.values().find(|c| c.user_id == user_id).cloned()
    }

    /// Clears all items from the user's cart
    ///
    /// # Arguments
    /// * `user_id` - The user ID
    ///
    /// # Returns
    /// `Some(Cart)` with empty items if the cart exists, `None` if the user has no cart
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Carts lock poisoned");

        // Find cart for user
        let cart_id = carts.values().find(|c| c.user_id == user_id)?.id;

        // Clear items
        let cart = carts.get_mut(&cart_id)?;
        cart.items.clear();

        Some(cart.clone())
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

    fn create_test_product(id: i32, name: &str, price: Decimal) -> Product {
        Product {
            id,
            name: name.to_string(),
            description: "Test product".to_string(),
            price,
            inventory_count: 10,
        }
    }

    #[test]
    fn test_cart_total() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![
                CartItem {
                    product_id: 1,
                    quantity: 2,
                    product_name: "Product 1".to_string(),
                    unit_price: dec!(10.00),
                },
                CartItem {
                    product_id: 2,
                    quantity: 1,
                    product_name: "Product 2".to_string(),
                    unit_price: dec!(25.00),
                },
            ],
        };

        assert_eq!(cart.total(), dec!(45.00));
    }

    #[test]
    fn test_cart_item_count() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![
                CartItem {
                    product_id: 1,
                    quantity: 2,
                    product_name: "Product 1".to_string(),
                    unit_price: dec!(10.00),
                },
                CartItem {
                    product_id: 2,
                    quantity: 3,
                    product_name: "Product 2".to_string(),
                    unit_price: dec!(25.00),
                },
            ],
        };

        assert_eq!(cart.item_count(), 2);
        assert_eq!(cart.total_quantity(), 5);
    }

    #[test]
    fn test_get_or_create_cart_new() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);

        assert_eq!(cart.user_id, 1);
        assert_eq!(cart.items.len(), 0);
        assert_eq!(cart.id, 1);
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
    fn test_add_item_new() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99));

        let cart = service.add_item(1, &product, 2);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].unit_price, dec!(19.99));
    }

    #[test]
    fn test_add_item_increment_quantity() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99));

        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_multiple_products() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", dec!(10.00));
        let product2 = create_test_product(2, "Product 2", dec!(20.00));

        let _ = service.add_item(1, &product1, 1);
        let cart = service.add_item(1, &product2, 2);

        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.total(), dec!(50.00));
    }

    #[test]
    fn test_remove_item() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", dec!(10.00));
        let product2 = create_test_product(2, "Product 2", dec!(20.00));

        let _ = service.add_item(1, &product1, 1);
        let _ = service.add_item(1, &product2, 2);

        let cart = service.remove_item(1, 1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_remove_item_nonexistent() {
        let service = CartService::new();
        let product = create_test_product(1, "Product 1", dec!(10.00));

        let _ = service.add_item(1, &product, 1);
        let cart = service.remove_item(1, 999);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 1); // Item not removed
    }

    #[test]
    fn test_remove_item_no_cart() {
        let service = CartService::new();
        let result = service.remove_item(999, 1);

        assert!(result.is_none());
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99));

        let _ = service.add_item(1, &product, 2);
        let cart = service.get_cart(1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.user_id, 1);
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
        let product1 = create_test_product(1, "Product 1", dec!(10.00));
        let product2 = create_test_product(2, "Product 2", dec!(20.00));

        let _ = service.add_item(1, &product1, 1);
        let _ = service.add_item(1, &product2, 2);

        let cart = service.clear_cart(1);

        assert!(cart.is_some());
        let cart = cart.unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_clear_cart_nonexistent() {
        let service = CartService::new();
        let result = service.clear_cart(999);

        assert!(result.is_none());
    }

    #[test]
    fn test_cart_isolation_per_user() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", dec!(10.00));
        let product2 = create_test_product(2, "Product 2", dec!(20.00));

        // User 1's cart
        let _ = service.add_item(1, &product1, 1);

        // User 2's cart
        let _ = service.add_item(2, &product2, 2);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_ne!(cart1.id, cart2.id);
        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart1.items[0].product_id, 1);
        assert_eq!(cart2.items[0].product_id, 2);
    }

    #[test]
    fn test_concurrent_cart_operations() {
        use std::thread;

        let service = CartService::new();
        let mut handles = vec![];

        // Create carts for multiple users concurrently
        for user_id in 1..=5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                let product =
                    create_test_product(user_id, &format!("Product {user_id}"), dec!(10.00));
                service_clone.add_item(user_id, &product, 1)
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify each user has their own cart
        for user_id in 1..=5 {
            let cart = service.get_cart(user_id);
            assert!(cart.is_some());
            let cart = cart.unwrap();
            assert_eq!(cart.user_id, user_id);
            assert_eq!(cart.items.len(), 1);
        }
    }

    #[test]
    fn test_cart_serialization() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![CartItem {
                product_id: 1,
                quantity: 2,
                product_name: "Test Product".to_string(),
                unit_price: dec!(19.99),
            }],
        };

        let json = serde_json::to_string(&cart).unwrap();
        let deserialized: Cart = serde_json::from_str(&json).unwrap();

        assert_eq!(cart, deserialized);
    }
}
