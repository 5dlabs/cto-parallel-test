use crate::cart::models::{Cart, CartItem};
use crate::catalog::ProductService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Thread-safe shopping cart service with in-memory storage
#[derive(Clone)]
pub struct CartService {
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    next_id: Arc<Mutex<i32>>,
    product_service: ProductService,
}

/// Error types for cart operations
#[derive(Debug, PartialEq)]
pub enum CartError {
    ProductNotFound,
    InsufficientInventory { available: i32, requested: i32 },
    InvalidQuantity,
    CartNotFound,
    ItemNotFound,
}

impl CartService {
    /// Creates a new cart service with the given product service
    #[must_use]
    pub fn new(product_service: ProductService) -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            product_service,
        }
    }

    /// Gets or creates a cart for a user
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().unwrap();

        // Find existing cart for this user
        if let Some(cart) = carts.values().find(|c| c.user_id == user_id).cloned() {
            return cart;
        }

        // Create new cart
        let mut next_id = self.next_id.lock().unwrap();
        let cart = Cart::new(*next_id, user_id);
        *next_id += 1;

        carts.insert(cart.id, cart.clone());
        cart
    }

    /// Gets a cart for a user
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_cart(&self, user_id: i32) -> Option<Cart> {
        let carts = self.carts.lock().unwrap();
        carts.values().find(|c| c.user_id == user_id).cloned()
    }

    /// Adds an item to a user's cart with inventory validation
    ///
    /// # Errors
    /// Returns an error if:
    /// - The product doesn't exist
    /// - There's insufficient inventory
    /// - The quantity is invalid (<=0)
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    pub fn add_item(
        &self,
        user_id: i32,
        product_id: i32,
        quantity: i32,
    ) -> Result<Cart, CartError> {
        // Validate quantity
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity);
        }

        // Get product and validate it exists
        let product = self
            .product_service
            .get_by_id(product_id)
            .ok_or(CartError::ProductNotFound)?;

        // Validate inventory
        if product.inventory_count < quantity {
            return Err(CartError::InsufficientInventory {
                available: product.inventory_count,
                requested: quantity,
            });
        }

        // Get or create cart
        let cart = self.get_or_create_cart(user_id);
        let mut carts = self.carts.lock().unwrap();

        // Get mutable reference to the cart
        let cart_mut = carts.get_mut(&cart.id).unwrap();

        // Check if item already exists in cart
        if let Some(existing_item) = cart_mut
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
        {
            // Update quantity and validate total inventory
            let new_quantity = existing_item.quantity + quantity;
            if product.inventory_count < new_quantity {
                return Err(CartError::InsufficientInventory {
                    available: product.inventory_count,
                    requested: new_quantity,
                });
            }
            existing_item.quantity = new_quantity;
        } else {
            // Add new item
            cart_mut.items.push(CartItem::new(
                product_id,
                quantity,
                product.name,
                product.price,
            ));
        }

        Ok(cart_mut.clone())
    }

    /// Removes an item from a user's cart
    ///
    /// # Errors
    /// Returns an error if:
    /// - The cart doesn't exist
    /// - The item is not in the cart
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Result<Cart, CartError> {
        let mut carts = self.carts.lock().unwrap();

        // Find the cart for this user
        let cart = carts
            .values_mut()
            .find(|c| c.user_id == user_id)
            .ok_or(CartError::CartNotFound)?;

        // Find and remove the item
        let initial_len = cart.items.len();
        cart.items.retain(|item| item.product_id != product_id);

        if cart.items.len() == initial_len {
            return Err(CartError::ItemNotFound);
        }

        Ok(cart.clone())
    }

    /// Updates the quantity of an item in the cart
    ///
    /// # Errors
    /// Returns an error if:
    /// - The cart doesn't exist
    /// - The item is not in the cart
    /// - The quantity is invalid
    /// - There's insufficient inventory
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    pub fn update_quantity(
        &self,
        user_id: i32,
        product_id: i32,
        quantity: i32,
    ) -> Result<Cart, CartError> {
        // Validate quantity
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity);
        }

        // Get product and validate inventory
        let product = self
            .product_service
            .get_by_id(product_id)
            .ok_or(CartError::ProductNotFound)?;

        if product.inventory_count < quantity {
            return Err(CartError::InsufficientInventory {
                available: product.inventory_count,
                requested: quantity,
            });
        }

        let mut carts = self.carts.lock().unwrap();

        // Find the cart for this user
        let cart = carts
            .values_mut()
            .find(|c| c.user_id == user_id)
            .ok_or(CartError::CartNotFound)?;

        // Find and update the item
        let item = cart
            .items
            .iter_mut()
            .find(|item| item.product_id == product_id)
            .ok_or(CartError::ItemNotFound)?;

        item.quantity = quantity;

        Ok(cart.clone())
    }

    /// Clears all items from a user's cart
    ///
    /// # Errors
    /// Returns an error if the cart doesn't exist
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    pub fn clear_cart(&self, user_id: i32) -> Result<Cart, CartError> {
        let mut carts = self.carts.lock().unwrap();

        // Find the cart for this user
        let cart = carts
            .values_mut()
            .find(|c| c.user_id == user_id)
            .ok_or(CartError::CartNotFound)?;

        cart.items.clear();

        Ok(cart.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::NewProduct;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::thread;

    fn create_test_product_service() -> ProductService {
        let service = ProductService::new();

        // Add test products
        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Test product 1".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 100,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test product 2".to_string(),
            price: Decimal::from_str("25.50").unwrap(),
            inventory_count: 50,
        });

        let _ = service.create(NewProduct {
            name: "Limited Stock".to_string(),
            description: "Low inventory product".to_string(),
            price: Decimal::from_str("99.99").unwrap(),
            inventory_count: 2,
        });

        service
    }

    #[test]
    fn test_get_or_create_cart() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let cart = cart_service.get_or_create_cart(1);
        assert_eq!(cart.user_id, 1);
        assert_eq!(cart.items.len(), 0);

        // Getting again should return the same cart
        let cart2 = cart_service.get_or_create_cart(1);
        assert_eq!(cart.id, cart2.id);
    }

    #[test]
    fn test_get_cart() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        // Non-existent cart
        assert!(cart_service.get_cart(999).is_none());

        // Create a cart
        let _cart = cart_service.get_or_create_cart(1);

        // Now it should exist
        let retrieved = cart_service.get_cart(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().user_id, 1);
    }

    #[test]
    fn test_add_item_success() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let result = cart_service.add_item(1, 1, 5);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 5);
        assert_eq!(cart.items[0].product_name, "Product 1");
    }

    #[test]
    fn test_add_item_increments_quantity() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        // Add item first time
        cart_service.add_item(1, 1, 3).unwrap();

        // Add same item again
        let result = cart_service.add_item(1, 1, 2);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn test_add_item_product_not_found() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let result = cart_service.add_item(1, 999, 1);
        assert_eq!(result, Err(CartError::ProductNotFound));
    }

    #[test]
    fn test_add_item_insufficient_inventory() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        // Product 3 has only 2 items in stock
        let result = cart_service.add_item(1, 3, 10);
        assert_eq!(
            result,
            Err(CartError::InsufficientInventory {
                available: 2,
                requested: 10
            })
        );
    }

    #[test]
    fn test_add_item_invalid_quantity() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let result = cart_service.add_item(1, 1, 0);
        assert_eq!(result, Err(CartError::InvalidQuantity));

        let result = cart_service.add_item(1, 1, -5);
        assert_eq!(result, Err(CartError::InvalidQuantity));
    }

    #[test]
    fn test_add_multiple_items() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();
        cart_service.add_item(1, 2, 3).unwrap();

        let cart = cart_service.get_cart(1).unwrap();
        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_remove_item_success() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();
        cart_service.add_item(1, 2, 3).unwrap();

        let result = cart_service.remove_item(1, 1);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 2);
    }

    #[test]
    fn test_remove_item_cart_not_found() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let result = cart_service.remove_item(999, 1);
        assert_eq!(result, Err(CartError::CartNotFound));
    }

    #[test]
    fn test_remove_item_not_found() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();

        let result = cart_service.remove_item(1, 999);
        assert_eq!(result, Err(CartError::ItemNotFound));
    }

    #[test]
    fn test_update_quantity_success() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();

        let result = cart_service.update_quantity(1, 1, 10);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items[0].quantity, 10);
    }

    #[test]
    fn test_update_quantity_insufficient_inventory() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 3, 1).unwrap();

        let result = cart_service.update_quantity(1, 3, 100);
        assert_eq!(
            result,
            Err(CartError::InsufficientInventory {
                available: 2,
                requested: 100
            })
        );
    }

    #[test]
    fn test_update_quantity_invalid() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();

        let result = cart_service.update_quantity(1, 1, 0);
        assert_eq!(result, Err(CartError::InvalidQuantity));
    }

    #[test]
    fn test_clear_cart_success() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        cart_service.add_item(1, 1, 2).unwrap();
        cart_service.add_item(1, 2, 3).unwrap();

        let result = cart_service.clear_cart(1);
        assert!(result.is_ok());

        let cart = result.unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_clear_cart_not_found() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let result = cart_service.clear_cart(999);
        assert_eq!(result, Err(CartError::CartNotFound));
    }

    #[test]
    fn test_cart_isolation_between_users() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        // User 1 adds items
        cart_service.add_item(1, 1, 2).unwrap();
        cart_service.add_item(1, 2, 3).unwrap();

        // User 2 adds different items
        cart_service.add_item(2, 1, 5).unwrap();

        // Verify user 1's cart
        let cart1 = cart_service.get_cart(1).unwrap();
        assert_eq!(cart1.items.len(), 2);
        assert_eq!(cart1.items[0].quantity, 2);

        // Verify user 2's cart
        let cart2 = cart_service.get_cart(2).unwrap();
        assert_eq!(cart2.items.len(), 1);
        assert_eq!(cart2.items[0].quantity, 5);
    }

    #[test]
    fn test_concurrent_cart_access() {
        let product_service = create_test_product_service();
        let cart_service = CartService::new(product_service);

        let service1 = cart_service.clone();
        let service2 = cart_service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service1.add_item(1, 1, 1);
                let _ = service1.add_item(100 + i, 2, 2);
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service2.add_item(2, 2, 1);
                let _ = service2.add_item(200 + i, 1, 3);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Verify some carts exist
        let cart1 = cart_service.get_cart(1);
        let cart2 = cart_service.get_cart(2);

        assert!(cart1.is_some());
        assert!(cart2.is_some());
    }
}
