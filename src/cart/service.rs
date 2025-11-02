//! Shopping cart service implementation providing business logic for cart management.

use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in a shopping cart.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    /// Product identifier
    pub product_id: i32,
    /// Quantity of this product in the cart
    pub quantity: i32,
    /// Product name (cached for convenience)
    pub product_name: String,
    /// Unit price of the product (cached for convenience)
    pub unit_price: Decimal,
}

/// Represents a user's shopping cart.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    /// Cart identifier
    pub id: i32,
    /// User who owns this cart
    pub user_id: i32,
    /// Items in the cart
    pub items: Vec<CartItem>,
}

impl Cart {
    /// Calculates the total price of all items in the cart.
    ///
    /// # Returns
    ///
    /// The total price as a `Decimal`
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::{Cart, CartItem};
    /// use rust_decimal_macros::dec;
    ///
    /// let cart = Cart {
    ///     id: 1,
    ///     user_id: 1,
    ///     items: vec![
    ///         CartItem {
    ///             product_id: 1,
    ///             quantity: 2,
    ///             product_name: "Item".to_string(),
    ///             unit_price: dec!(10.00),
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(cart.total(), dec!(20.00));
    /// ```
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.items
            .iter()
            .map(|item| item.unit_price * Decimal::from(item.quantity))
            .sum()
    }
}

/// Thread-safe shopping cart service for managing user carts.
///
/// This service provides CRUD operations for shopping carts with thread-safe
/// concurrent access using `Arc<Mutex>`.
#[derive(Clone)]
pub struct CartService {
    /// Thread-safe storage for carts indexed by user ID
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Thread-safe counter for auto-incrementing cart IDs
    next_id: Arc<Mutex<i32>>,
}

impl CartService {
    /// Creates a new empty cart service.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    ///
    /// let service = CartService::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Gets the cart for a user, or creates a new empty cart if none exists.
    ///
    /// This method is thread-safe and can be called concurrently from multiple threads.
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
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    ///
    /// let service = CartService::new();
    /// let cart = service.get_or_create_cart(1);
    /// assert_eq!(cart.user_id, 1);
    /// assert!(cart.items.is_empty());
    /// ```
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        carts
            .entry(user_id)
            .or_insert_with(|| {
                let mut next_id = self.next_id.lock().expect("Failed to lock next_id");
                let cart_id = *next_id;
                *next_id += 1;

                Cart {
                    id: cart_id,
                    user_id,
                    items: Vec::new(),
                }
            })
            .clone()
    }

    /// Adds an item to a user's cart or increments quantity if already present.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `product` - The product to add
    /// * `quantity` - The quantity to add (must be positive)
    ///
    /// # Returns
    ///
    /// The updated cart
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    /// use cto_parallel_test::catalog::models::Product;
    /// use rust_decimal_macros::dec;
    ///
    /// let service = CartService::new();
    /// let product = Product {
    ///     id: 1,
    ///     name: "Test Product".to_string(),
    ///     description: "Description".to_string(),
    ///     price: dec!(19.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let cart = service.add_item(1, &product, 2);
    /// assert_eq!(cart.items.len(), 1);
    /// assert_eq!(cart.items[0].quantity, 2);
    /// ```
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        let cart = carts.entry(user_id).or_insert_with(|| {
            let mut next_id = self.next_id.lock().expect("Failed to lock next_id");
            let cart_id = *next_id;
            *next_id += 1;

            Cart {
                id: cart_id,
                user_id,
                items: Vec::new(),
            }
        });

        // Check if product already exists in cart
        if let Some(existing_item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            existing_item.quantity += quantity;
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

    /// Removes an item from a user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `product_id` - The product identifier to remove
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with the updated cart if the cart exists, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    /// use cto_parallel_test::catalog::models::Product;
    /// use rust_decimal_macros::dec;
    ///
    /// let service = CartService::new();
    /// let product = Product {
    ///     id: 1,
    ///     name: "Test Product".to_string(),
    ///     description: "Description".to_string(),
    ///     price: dec!(19.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let _ = service.add_item(1, &product, 2);
    /// let cart = service.remove_item(1, 1);
    /// assert!(cart.is_some());
    /// assert!(cart.unwrap().items.is_empty());
    /// ```
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        if let Some(cart) = carts.get_mut(&user_id) {
            cart.items.retain(|item| item.product_id != product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    /// Gets a user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    ///
    /// `Some(Cart)` if the user has a cart, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    ///
    /// let service = CartService::new();
    /// let cart = service.get_cart(1);
    /// assert!(cart.is_none());
    ///
    /// let _ = service.get_or_create_cart(1);
    /// let cart = service.get_cart(1);
    /// assert!(cart.is_some());
    /// ```
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().expect("Failed to lock carts");
        carts.get(&user_id).cloned()
    }

    /// Clears all items from a user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with the empty cart if the cart exists, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::cart::CartService;
    /// use cto_parallel_test::catalog::models::Product;
    /// use rust_decimal_macros::dec;
    ///
    /// let service = CartService::new();
    /// let product = Product {
    ///     id: 1,
    ///     name: "Test Product".to_string(),
    ///     description: "Description".to_string(),
    ///     price: dec!(19.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let _ = service.add_item(1, &product, 2);
    /// let cart = service.clear_cart(1);
    /// assert!(cart.is_some());
    /// assert!(cart.unwrap().items.is_empty());
    /// ```
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

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
    fn test_new_cart_service() {
        let service = CartService::new();
        assert!(service.get_cart(1).is_none());
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
    fn test_get_or_create_returns_existing() {
        let service = CartService::new();
        let cart1 = service.get_or_create_cart(1);
        let cart2 = service.get_or_create_cart(1);

        assert_eq!(cart1.id, cart2.id);
        assert_eq!(cart1.user_id, cart2.user_id);
    }

    #[test]
    fn test_add_item_to_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let cart = service.add_item(1, &product, 2);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].product_name, "Test Product");
        assert_eq!(cart.items[0].unit_price, dec!(19.99));
    }

    #[test]
    fn test_add_same_item_increments_quantity() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_different_items() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Product 1", dec!(10.00), 10);
        let product2 = create_test_product(2, "Product 2", dec!(20.00), 5);

        let _ = service.add_item(1, &product1, 2);
        let cart = service.add_item(1, &product2, 1);

        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_remove_item_from_cart() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.remove_item(1, 1);

        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.remove_item(1, 999);

        assert!(cart.is_some());
        assert_eq!(cart.unwrap().items.len(), 1);
    }

    #[test]
    fn test_remove_from_nonexistent_cart() {
        let service = CartService::new();
        let cart = service.remove_item(999, 1);
        assert!(cart.is_none());
    }

    #[test]
    fn test_get_cart() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let cart = service.get_cart(1);
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().user_id, 1);
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
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.clear_cart(1);

        assert!(cart.is_some());
        assert!(cart.unwrap().items.is_empty());
    }

    #[test]
    fn test_clear_nonexistent_cart() {
        let service = CartService::new();
        let cart = service.clear_cart(999);
        assert!(cart.is_none());
    }

    #[test]
    fn test_cart_isolation_per_user() {
        let service = CartService::new();
        let product = create_test_product(1, "Test Product", dec!(19.99), 10);

        let _ = service.add_item(1, &product, 2);
        let _ = service.add_item(2, &product, 3);

        let cart1 = service.get_cart(1).unwrap();
        let cart2 = service.get_cart(2).unwrap();

        assert_eq!(cart1.items[0].quantity, 2);
        assert_eq!(cart2.items[0].quantity, 3);
        assert_ne!(cart1.id, cart2.id);
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
                    product_name: "Item 1".to_string(),
                    unit_price: dec!(10.00),
                },
                CartItem {
                    product_id: 2,
                    quantity: 3,
                    product_name: "Item 2".to_string(),
                    unit_price: dec!(15.00),
                },
            ],
        };

        assert_eq!(cart.total(), dec!(65.00));
    }

    #[test]
    fn test_cart_total_empty() {
        let cart = Cart {
            id: 1,
            user_id: 1,
            items: vec![],
        };

        assert_eq!(cart.total(), dec!(0.00));
    }

    #[test]
    fn test_concurrent_operations() {
        use std::thread;

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
}
