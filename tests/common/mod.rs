//! Common test utilities and helpers
//!
//! This module provides shared test utilities, mock services, and helper functions
//! used across integration tests.

use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Creates a new `ProductService` instance with sample test data.
///
/// # Returns
///
/// A `ProductService` pre-populated with test products
pub fn get_test_product_service() -> ProductService {
    let service = ProductService::new();

    // Create sample products
    let _ = service.create(NewProduct {
        name: "Laptop Pro".to_string(),
        description: "High-end laptop for professionals".to_string(),
        price: dec!(1299.99),
        inventory_count: 10,
    });

    let _ = service.create(NewProduct {
        name: "Wireless Mouse".to_string(),
        description: "Ergonomic wireless mouse".to_string(),
        price: dec!(29.99),
        inventory_count: 50,
    });

    let _ = service.create(NewProduct {
        name: "USB-C Hub".to_string(),
        description: "7-in-1 USB-C hub".to_string(),
        price: dec!(49.99),
        inventory_count: 30,
    });

    let _ = service.create(NewProduct {
        name: "Mechanical Keyboard".to_string(),
        description: "RGB mechanical keyboard".to_string(),
        price: dec!(149.99),
        inventory_count: 20,
    });

    let _ = service.create(NewProduct {
        name: "Monitor 27\"".to_string(),
        description: "4K UHD monitor".to_string(),
        price: dec!(399.99),
        inventory_count: 15,
    });

    let _ = service.create(NewProduct {
        name: "Webcam HD".to_string(),
        description: "1080p webcam with microphone".to_string(),
        price: dec!(79.99),
        inventory_count: 0, // Out of stock
    });

    service
}

/// Creates a sample new product for testing.
///
/// # Arguments
///
/// * `name` - Product name
/// * `price` - Product price
/// * `inventory` - Initial inventory count
///
/// # Returns
///
/// A `NewProduct` instance ready for creation
pub fn create_test_product(name: &str, price: Decimal, inventory: i32) -> NewProduct {
    NewProduct {
        name: name.to_string(),
        description: format!("Test product: {name}"),
        price,
        inventory_count: inventory,
    }
}

/// Mock authentication service for testing
///
/// This module provides simple JWT token creation and validation
/// for testing purposes without requiring full auth implementation.
pub mod auth {
    use serde::{Deserialize, Serialize};

    /// Simple JWT claims for testing
    #[derive(Debug, Serialize, Deserialize)]
    #[allow(dead_code)]
    pub struct Claims {
        /// Subject (user ID)
        pub sub: String,
        /// Expiration time
        pub exp: u64,
        /// Issued at
        pub iat: u64,
    }

    /// Creates a mock JWT token for testing
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID to encode in token
    ///
    /// # Returns
    ///
    /// A mock JWT token string
    pub fn create_test_token(user_id: i32) -> String {
        // Using a static expiration for testing (real implementation would use Clock abstraction)
        let static_expiration = 1_234_567_890_u64;

        format!("mock_token_user_{user_id}_exp_{static_expiration}")
    }

    /// Validates a mock JWT token for testing
    ///
    /// # Arguments
    ///
    /// * `token` - Token string to validate
    ///
    /// # Returns
    ///
    /// `Ok(user_id)` if valid, `Err` otherwise
    pub fn validate_test_token(token: &str) -> Result<i32, String> {
        if let Some(user_part) = token.strip_prefix("mock_token_user_") {
            if let Some(user_id_str) = user_part.split('_').next() {
                user_id_str
                    .parse::<i32>()
                    .map_err(|_| "Invalid user ID".to_string())
            } else {
                Err("Invalid token format".to_string())
            }
        } else {
            Err("Invalid token".to_string())
        }
    }

    /// Mock password hashing for testing
    ///
    /// # Arguments
    ///
    /// * `password` - Plain text password
    ///
    /// # Returns
    ///
    /// A mock hashed password string
    pub fn hash_test_password(password: &str) -> String {
        format!("hashed_{password}")
    }

    /// Mock password verification for testing
    ///
    /// # Arguments
    ///
    /// * `password` - Plain text password
    /// * `hash` - Hashed password
    ///
    /// # Returns
    ///
    /// `true` if password matches hash
    pub fn verify_test_password(password: &str, hash: &str) -> bool {
        hash == format!("hashed_{password}")
    }
}

/// Mock cart service for testing
///
/// This module provides a simple in-memory cart service for testing
/// without requiring full cart implementation.
pub mod cart {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    /// Represents a cart item
    #[derive(Debug, Clone)]
    pub struct CartItem {
        /// Product ID
        pub product_id: i32,
        /// Quantity
        pub quantity: i32,
    }

    /// Simple cart service for testing
    #[derive(Clone)]
    pub struct TestCartService {
        /// In-memory cart storage: `user_id` -> `Vec<CartItem>`
        carts: Arc<Mutex<HashMap<i32, Vec<CartItem>>>>,
    }

    impl TestCartService {
        /// Creates a new test cart service
        #[must_use]
        pub fn new() -> Self {
            Self {
                carts: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Adds an item to a user's cart
        ///
        /// # Arguments
        ///
        /// * `user_id` - User ID
        /// * `product_id` - Product ID to add
        /// * `quantity` - Quantity to add
        pub fn add_item(&self, user_id: i32, product_id: i32, quantity: i32) {
            let mut carts = self.carts.lock().expect("Failed to lock carts");
            let cart = carts.entry(user_id).or_default();

            if let Some(item) = cart.iter_mut().find(|i| i.product_id == product_id) {
                item.quantity += quantity;
            } else {
                cart.push(CartItem {
                    product_id,
                    quantity,
                });
            }
        }

        /// Gets a user's cart
        ///
        /// # Arguments
        ///
        /// * `user_id` - User ID
        ///
        /// # Returns
        ///
        /// Vector of cart items
        pub fn get_cart(&self, user_id: i32) -> Vec<CartItem> {
            let carts = self.carts.lock().expect("Failed to lock carts");
            carts.get(&user_id).cloned().unwrap_or_default()
        }

        /// Removes an item from a user's cart
        ///
        /// # Arguments
        ///
        /// * `user_id` - User ID
        /// * `product_id` - Product ID to remove
        ///
        /// # Returns
        ///
        /// `true` if item was removed
        pub fn remove_item(&self, user_id: i32, product_id: i32) -> bool {
            let mut carts = self.carts.lock().expect("Failed to lock carts");
            if let Some(cart) = carts.get_mut(&user_id) {
                let initial_len = cart.len();
                cart.retain(|item| item.product_id != product_id);
                cart.len() < initial_len
            } else {
                false
            }
        }

        /// Clears a user's cart
        ///
        /// # Arguments
        ///
        /// * `user_id` - User ID
        pub fn clear_cart(&self, user_id: i32) {
            let mut carts = self.carts.lock().expect("Failed to lock carts");
            carts.remove(&user_id);
        }
    }

    impl Default for TestCartService {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_product_service() {
        let service = get_test_product_service();
        let products = service.get_all();
        assert_eq!(products.len(), 6);
        assert!(products.iter().any(|p| p.name == "Laptop Pro"));
    }

    #[test]
    fn test_create_test_product() {
        let product = create_test_product("Test", dec!(19.99), 10);
        assert_eq!(product.name, "Test");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auth_create_token() {
        let token = auth::create_test_token(123);
        assert!(token.contains("mock_token_user_123"));
    }

    #[test]
    fn test_auth_validate_token() {
        let token = auth::create_test_token(456);
        let user_id = auth::validate_test_token(&token);
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), 456);
    }

    #[test]
    fn test_auth_invalid_token() {
        let result = auth::validate_test_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_auth_hash_password() {
        let hash = auth::hash_test_password("secret");
        assert_eq!(hash, "hashed_secret");
    }

    #[test]
    fn test_auth_verify_password() {
        let hash = auth::hash_test_password("mypassword");
        assert!(auth::verify_test_password("mypassword", &hash));
        assert!(!auth::verify_test_password("wrongpassword", &hash));
    }

    #[test]
    fn test_cart_add_item() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 2);

        let cart = service.get_cart(1);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, 101);
        assert_eq!(cart[0].quantity, 2);
    }

    #[test]
    fn test_cart_add_multiple_items() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 1);
        service.add_item(1, 102, 3);

        let cart = service.get_cart(1);
        assert_eq!(cart.len(), 2);
    }

    #[test]
    fn test_cart_increment_quantity() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 2);
        service.add_item(1, 101, 3);

        let cart = service.get_cart(1);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].quantity, 5);
    }

    #[test]
    fn test_cart_remove_item() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 2);
        service.add_item(1, 102, 1);

        assert!(service.remove_item(1, 101));
        let cart = service.get_cart(1);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, 102);
    }

    #[test]
    fn test_cart_remove_nonexistent() {
        let service = cart::TestCartService::new();
        assert!(!service.remove_item(1, 999));
    }

    #[test]
    fn test_cart_clear() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 2);
        service.add_item(1, 102, 3);

        service.clear_cart(1);
        let cart = service.get_cart(1);
        assert_eq!(cart.len(), 0);
    }

    #[test]
    fn test_cart_multiple_users() {
        let service = cart::TestCartService::new();
        service.add_item(1, 101, 2);
        service.add_item(2, 102, 3);

        let cart1 = service.get_cart(1);
        let cart2 = service.get_cart(2);

        assert_eq!(cart1.len(), 1);
        assert_eq!(cart2.len(), 1);
        assert_eq!(cart1[0].product_id, 101);
        assert_eq!(cart2[0].product_id, 102);
    }
}
