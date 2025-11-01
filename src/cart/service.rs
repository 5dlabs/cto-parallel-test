//! Cart service providing thread-safe cart operations
//!
//! This module implements the cart service with in-memory storage,
//! thread-safe operations, and integration with the product catalog.

use crate::cart::models::{Cart, CartItem};
use crate::catalog::ProductService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Error types for cart operations
#[derive(Debug, Clone, PartialEq)]
pub enum CartError {
    /// Product not found in catalog
    ProductNotFound,
    /// Insufficient inventory for requested quantity
    InsufficientInventory {
        /// The available inventory count
        available: i32,
        /// The requested quantity
        requested: i32,
    },
    /// Cart not found for user
    CartNotFound,
    /// Invalid quantity (must be positive)
    InvalidQuantity,
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductNotFound => write!(f, "Product not found"),
            Self::InsufficientInventory {
                available,
                requested,
            } => {
                write!(
                    f,
                    "Insufficient inventory: available {available}, requested {requested}"
                )
            }
            Self::CartNotFound => write!(f, "Cart not found"),
            Self::InvalidQuantity => write!(f, "Quantity must be positive"),
        }
    }
}

impl std::error::Error for CartError {}

/// Thread-safe shopping cart service with in-memory storage
#[derive(Debug, Clone)]
pub struct CartService {
    /// Maps `user_id` to their Cart
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Auto-incrementing cart ID
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
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should never happen in normal operation)
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");

        if let Some(cart) = carts.get(&user_id) {
            cart.clone()
        } else {
            let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");
            let cart = Cart::new(*next_id, user_id);
            *next_id += 1;
            carts.insert(user_id, cart.clone());
            cart
        }
    }

    /// Adds an item to the user's cart, validating inventory availability
    ///
    /// If the item already exists in the cart, the quantity is incremented.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Product is not found in the catalog
    /// - Requested quantity exceeds available inventory
    /// - Quantity is not positive
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn add_item(
        &self,
        user_id: i32,
        product_service: &ProductService,
        product_id: i32,
        quantity: i32,
    ) -> Result<Cart, CartError> {
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity);
        }

        // Get product from catalog
        let product = product_service
            .get_by_id(product_id)
            .ok_or(CartError::ProductNotFound)?;

        // Get user's cart
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");
        let cart = carts.entry(user_id).or_insert_with(|| {
            let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");
            let new_cart = Cart::new(*next_id, user_id);
            *next_id += 1;
            new_cart
        });

        // Calculate total quantity (existing + new)
        let existing_quantity = cart
            .items
            .iter()
            .find(|item| item.product_id == product_id)
            .map_or(0, |item| item.quantity);

        let total_quantity = existing_quantity + quantity;

        // Check inventory
        if product.inventory_count < total_quantity {
            return Err(CartError::InsufficientInventory {
                available: product.inventory_count,
                requested: total_quantity,
            });
        }

        // Add or update item in cart
        if let Some(existing_item) = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
        {
            existing_item.quantity = total_quantity;
        } else {
            cart.items.push(CartItem::new(
                product.id,
                quantity,
                product.name.clone(),
                product.price,
            ));
        }

        Ok(cart.clone())
    }

    /// Updates the quantity of an item in the cart
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Cart not found for user
    /// - Product not found in cart or catalog
    /// - Requested quantity exceeds available inventory
    /// - Quantity is not positive
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn update_quantity(
        &self,
        user_id: i32,
        product_service: &ProductService,
        product_id: i32,
        quantity: i32,
    ) -> Result<Cart, CartError> {
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity);
        }

        // Get product from catalog
        let product = product_service
            .get_by_id(product_id)
            .ok_or(CartError::ProductNotFound)?;

        // Check inventory
        if product.inventory_count < quantity {
            return Err(CartError::InsufficientInventory {
                available: product.inventory_count,
                requested: quantity,
            });
        }

        let mut carts = self.carts.lock().expect("Carts mutex poisoned");
        let cart = carts.get_mut(&user_id).ok_or(CartError::CartNotFound)?;

        // Find and update the item
        if let Some(item) = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
        {
            item.quantity = quantity;
            Ok(cart.clone())
        } else {
            Err(CartError::ProductNotFound)
        }
    }

    /// Removes an item from the user's cart
    ///
    /// # Errors
    ///
    /// Returns an error if cart is not found for the user.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Result<Cart, CartError> {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");
        let cart = carts.get_mut(&user_id).ok_or(CartError::CartNotFound)?;

        cart.items.retain(|item| item.product_id != product_id);
        Ok(cart.clone())
    }

    /// Retrieves the user's cart
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
    /// # Errors
    ///
    /// Returns an error if cart is not found for the user.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn clear_cart(&self, user_id: i32) -> Result<Cart, CartError> {
        let mut carts = self.carts.lock().expect("Carts mutex poisoned");
        let cart = carts.get_mut(&user_id).ok_or(CartError::CartNotFound)?;

        cart.items.clear();
        Ok(cart.clone())
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
    use crate::catalog::NewProduct;
    use rust_decimal_macros::dec;
    use std::thread;

    fn create_test_product_service() -> ProductService {
        let service = ProductService::new();

        // Add some test products
        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });

        let _ = service.create(NewProduct {
            name: "Keyboard".to_string(),
            description: "Mechanical keyboard".to_string(),
            price: dec!(79.99),
            inventory_count: 0, // Out of stock
        });

        service
    }

    #[test]
    fn test_get_or_create_cart() {
        let service = CartService::new();
        let user_id = 1;

        let cart = service.get_or_create_cart(user_id);
        assert_eq!(cart.user_id, user_id);
        assert_eq!(cart.items.len(), 0);

        // Getting again should return the same cart
        let cart2 = service.get_or_create_cart(user_id);
        assert_eq!(cart.id, cart2.id);
    }

    #[test]
    fn test_add_item_success() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        let result = cart_service.add_item(user_id, &product_service, 1, 2);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].product_name, "Laptop");
    }

    #[test]
    fn test_add_item_increments_quantity() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Add item first time
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();

        // Add same item again
        let result = cart_service.add_item(user_id, &product_service, 1, 3);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5); // 2 + 3
    }

    #[test]
    fn test_add_item_product_not_found() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        let result = cart_service.add_item(user_id, &product_service, 999, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CartError::ProductNotFound);
    }

    #[test]
    fn test_add_item_insufficient_inventory() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        let result = cart_service.add_item(user_id, &product_service, 1, 100);
        assert!(result.is_err());

        if let Err(CartError::InsufficientInventory {
            available,
            requested,
        }) = result
        {
            assert_eq!(available, 10);
            assert_eq!(requested, 100);
        } else {
            panic!("Expected InsufficientInventory error");
        }
    }

    #[test]
    fn test_add_item_out_of_stock() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        let result = cart_service.add_item(user_id, &product_service, 3, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_item_invalid_quantity() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        let result = cart_service.add_item(user_id, &product_service, 1, 0);
        assert_eq!(result.unwrap_err(), CartError::InvalidQuantity);

        let result = cart_service.add_item(user_id, &product_service, 1, -5);
        assert_eq!(result.unwrap_err(), CartError::InvalidQuantity);
    }

    #[test]
    fn test_update_quantity_success() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Add item first
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();

        // Update quantity
        let result = cart_service.update_quantity(user_id, &product_service, 1, 5);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_update_quantity_insufficient_inventory() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();

        let result = cart_service.update_quantity(user_id, &product_service, 1, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_quantity_cart_not_found() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();

        let result = cart_service.update_quantity(999, &product_service, 1, 5);
        assert_eq!(result.unwrap_err(), CartError::CartNotFound);
    }

    #[test]
    fn test_remove_item_success() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Add two items
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();
        cart_service
            .add_item(user_id, &product_service, 2, 3)
            .unwrap();

        // Remove one item
        let result = cart_service.remove_item(user_id, 1);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_remove_item_cart_not_found() {
        let cart_service = CartService::new();

        let result = cart_service.remove_item(999, 1);
        assert_eq!(result.unwrap_err(), CartError::CartNotFound);
    }

    #[test]
    fn test_get_cart() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Initially no cart
        assert!(cart_service.get_cart(user_id).is_none());

        // Add item (creates cart)
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();

        // Now cart exists
        let cart = cart_service.get_cart(user_id);
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().items.len(), 1);
    }

    #[test]
    fn test_clear_cart() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Add items
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();
        cart_service
            .add_item(user_id, &product_service, 2, 3)
            .unwrap();

        // Clear cart
        let result = cart_service.clear_cart(user_id);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_clear_cart_not_found() {
        let cart_service = CartService::new();

        let result = cart_service.clear_cart(999);
        assert_eq!(result.unwrap_err(), CartError::CartNotFound);
    }

    #[test]
    fn test_user_isolation() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();

        // User 1 adds items
        cart_service.add_item(1, &product_service, 1, 2).unwrap();

        // User 2 adds different items
        cart_service.add_item(2, &product_service, 2, 3).unwrap();

        // Check user 1's cart
        let cart1 = cart_service.get_cart(1).unwrap();
        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart1.items[0].product_id, 1);

        // Check user 2's cart
        let cart2 = cart_service.get_cart(2).unwrap();
        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart2.items[0].product_id, 2);
    }

    #[test]
    fn test_concurrent_access() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();

        let cart_service_clone1 = cart_service.clone();
        let product_service_clone1 = product_service.clone();
        let cart_service_clone2 = cart_service.clone();
        let product_service_clone2 = product_service.clone();

        let handle1 = thread::spawn(move || {
            for i in 1..=10 {
                let _ = cart_service_clone1.add_item(i, &product_service_clone1, 1, 1);
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 11..=20 {
                let _ = cart_service_clone2.add_item(i, &product_service_clone2, 2, 1);
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");

        // Verify that all carts were created correctly
        for i in 1..=10 {
            let cart = cart_service.get_cart(i);
            assert!(cart.is_some());
        }
    }

    #[test]
    fn test_multiple_items_in_cart() {
        let cart_service = CartService::new();
        let product_service = create_test_product_service();
        let user_id = 1;

        // Add multiple different items
        cart_service
            .add_item(user_id, &product_service, 1, 2)
            .unwrap();
        cart_service
            .add_item(user_id, &product_service, 2, 3)
            .unwrap();

        let cart = cart_service.get_cart(user_id).unwrap();
        assert_eq!(cart.items.len(), 2);

        // Verify total price calculation
        let expected_total = dec!(999.99) * dec!(2) + dec!(29.99) * dec!(3);
        assert_eq!(cart.total_price(), expected_total);
    }
}
