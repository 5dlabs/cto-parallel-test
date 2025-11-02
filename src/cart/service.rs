//! Cart service implementation providing business logic for shopping cart management.

use crate::catalog::models::Product;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an item in a shopping cart.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    /// Product ID
    pub product_id: i32,
    /// Quantity of this product in the cart
    pub quantity: i32,
    /// Product name (cached from product catalog)
    pub product_name: String,
    /// Unit price of the product (cached from product catalog)
    pub unit_price: Decimal,
}

impl CartItem {
    /// Creates a new cart item from a product and quantity.
    ///
    /// # Arguments
    ///
    /// * `product` - The product to add to the cart
    /// * `quantity` - The quantity to add
    ///
    /// # Returns
    ///
    /// A new `CartItem` instance
    #[must_use]
    pub fn from_product(product: &Product, quantity: i32) -> Self {
        Self {
            product_id: product.id,
            quantity,
            product_name: product.name.clone(),
            unit_price: product.price,
        }
    }

    /// Calculates the total price for this cart item (`unit_price` * `quantity`).
    ///
    /// # Returns
    ///
    /// The total price as a `Decimal`
    #[must_use]
    pub fn total_price(&self) -> Decimal {
        self.unit_price * Decimal::from(self.quantity)
    }
}

/// Represents a user's shopping cart.
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
    /// Calculates the total value of all items in the cart.
    ///
    /// # Returns
    ///
    /// The total cart value as a `Decimal`
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.items.iter().map(CartItem::total_price).sum()
    }

    /// Checks if the cart is empty.
    ///
    /// # Returns
    ///
    /// `true` if the cart has no items
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets the number of items in the cart.
    ///
    /// # Returns
    ///
    /// The number of cart items
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

/// Thread-safe cart service for managing shopping carts in-memory.
///
/// This service provides cart operations with user isolation, ensuring each user
/// has their own cart. Cart operations are thread-safe using `Arc<Mutex>`.
#[derive(Clone)]
pub struct CartService {
    /// Maps `user_id` to their cart
    carts: Arc<Mutex<HashMap<i32, Cart>>>,
    /// Counter for auto-incrementing cart IDs
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

    /// Gets or creates a cart for the specified user.
    ///
    /// If the user doesn't have a cart, a new empty cart is created.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique user identifier
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
    /// assert!(cart.is_empty());
    /// ```
    #[must_use]
    pub fn get_or_create_cart(&self, user_id: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        if let Some(cart) = carts.get(&user_id) {
            cart.clone()
        } else {
            let mut next_id = self.next_id.lock().expect("Failed to lock next_id");
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

    /// Adds an item to the user's cart or increments quantity if already present.
    ///
    /// If the product is already in the cart, its quantity is incremented by the
    /// specified amount. Otherwise, a new cart item is added.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique user identifier
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
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let cart = service.add_item(1, &product, 2);
    /// assert_eq!(cart.items.len(), 1);
    /// assert_eq!(cart.items[0].quantity, 2);
    ///
    /// // Adding same product again increments quantity
    /// let cart = service.add_item(1, &product, 1);
    /// assert_eq!(cart.items.len(), 1);
    /// assert_eq!(cart.items[0].quantity, 3);
    /// ```
    #[must_use]
    pub fn add_item(&self, user_id: i32, product: &Product, quantity: i32) -> Cart {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        let cart = carts.entry(user_id).or_insert_with(|| {
            let mut next_id = self.next_id.lock().expect("Failed to lock next_id");
            let new_cart = Cart {
                id: *next_id,
                user_id,
                items: Vec::new(),
            };
            *next_id += 1;
            new_cart
        });

        // Check if product already in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product.id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItem::from_product(product, quantity));
        }

        cart.clone()
    }

    /// Removes an item from the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique user identifier
    /// * `product_id` - The ID of the product to remove
    ///
    /// # Returns
    ///
    /// `Some(Cart)` if the user has a cart, `None` if the user has no cart
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
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let _ = service.add_item(1, &product, 2);
    /// let cart = service.remove_item(1, 1).expect("Cart should exist");
    /// assert!(cart.is_empty());
    /// ```
    #[must_use]
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        carts.get_mut(&user_id).map(|cart| {
            cart.items.retain(|item| item.product_id != product_id);
            cart.clone()
        })
    }

    /// Retrieves the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique user identifier
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
    /// assert!(cart.is_none()); // No cart exists yet
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

    /// Clears all items from the user's cart.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique user identifier
    ///
    /// # Returns
    ///
    /// `Some(Cart)` with empty items if the user has a cart, `None` otherwise
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
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let _ = service.add_item(1, &product, 2);
    /// let cart = service.clear_cart(1).expect("Cart should exist");
    /// assert!(cart.is_empty());
    /// ```
    #[must_use]
    pub fn clear_cart(&self, user_id: i32) -> Option<Cart> {
        let mut carts = self.carts.lock().expect("Failed to lock carts");

        carts.get_mut(&user_id).map(|cart| {
            cart.items.clear();
            cart.clone()
        })
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
    fn test_cart_item_from_product() {
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);
        let item = CartItem::from_product(&product, 2);

        assert_eq!(item.product_id, 1);
        assert_eq!(item.quantity, 2);
        assert_eq!(item.product_name, "Laptop");
        assert_eq!(item.unit_price, dec!(999.99));
    }

    #[test]
    fn test_cart_item_total_price() {
        let product = create_test_product(1, "Mouse", dec!(29.99), 50);
        let item = CartItem::from_product(&product, 3);

        assert_eq!(item.total_price(), dec!(89.97));
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
                    product_name: "Laptop".to_string(),
                    unit_price: dec!(1000.00),
                },
                CartItem {
                    product_id: 2,
                    quantity: 1,
                    product_name: "Mouse".to_string(),
                    unit_price: dec!(29.99),
                },
            ],
        };

        assert_eq!(cart.total(), dec!(2029.99));
    }

    #[test]
    fn test_cart_is_empty() {
        let empty_cart = Cart {
            id: 1,
            user_id: 1,
            items: Vec::new(),
        };
        assert!(empty_cart.is_empty());

        let non_empty_cart = Cart {
            id: 2,
            user_id: 2,
            items: vec![CartItem {
                product_id: 1,
                quantity: 1,
                product_name: "Item".to_string(),
                unit_price: dec!(10.00),
            }],
        };
        assert!(!non_empty_cart.is_empty());
    }

    #[test]
    fn test_get_or_create_cart_new() {
        let service = CartService::new();
        let cart = service.get_or_create_cart(1);

        assert_eq!(cart.user_id, 1);
        assert!(cart.is_empty());
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
    fn test_add_item_new_product() {
        let service = CartService::new();
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let cart = service.add_item(1, &product, 2);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[0].quantity, 2);
        assert_eq!(cart.items[0].product_name, "Laptop");
    }

    #[test]
    fn test_add_item_existing_product() {
        let service = CartService::new();
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.add_item(1, &product, 1);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 3);
    }

    #[test]
    fn test_add_multiple_products() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Laptop", dec!(999.99), 10);
        let product2 = create_test_product(2, "Mouse", dec!(29.99), 50);

        let _ = service.add_item(1, &product1, 1);
        let cart = service.add_item(1, &product2, 2);

        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.items[0].product_id, 1);
        assert_eq!(cart.items[1].product_id, 2);
    }

    #[test]
    fn test_remove_item_existing() {
        let service = CartService::new();
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.remove_item(1, 1).expect("Cart should exist");

        assert!(cart.is_empty());
    }

    #[test]
    fn test_remove_item_nonexistent_product() {
        let service = CartService::new();
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.remove_item(1, 999).expect("Cart should exist");

        assert_eq!(cart.items.len(), 1); // Item not removed
    }

    #[test]
    fn test_remove_item_no_cart() {
        let service = CartService::new();
        let result = service.remove_item(1, 1);

        assert!(result.is_none());
    }

    #[test]
    fn test_get_cart_existing() {
        let service = CartService::new();
        let _ = service.get_or_create_cart(1);

        let cart = service.get_cart(1);
        assert!(cart.is_some());
        assert_eq!(cart.unwrap().user_id, 1);
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
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let _ = service.add_item(1, &product, 2);
        let cart = service.clear_cart(1).expect("Cart should exist");

        assert!(cart.is_empty());
    }

    #[test]
    fn test_clear_cart_nonexistent() {
        let service = CartService::new();
        let result = service.clear_cart(999);

        assert!(result.is_none());
    }

    #[test]
    fn test_user_isolation() {
        let service = CartService::new();
        let product = create_test_product(1, "Laptop", dec!(999.99), 10);

        let cart1 = service.add_item(1, &product, 1);
        let cart2 = service.add_item(2, &product, 2);

        assert_eq!(cart1.user_id, 1);
        assert_eq!(cart2.user_id, 2);
        assert_eq!(cart1.items[0].quantity, 1);
        assert_eq!(cart2.items[0].quantity, 2);
    }

    #[test]
    fn test_cart_auto_increment_ids() {
        let service = CartService::new();

        let cart1 = service.get_or_create_cart(1);
        let cart2 = service.get_or_create_cart(2);
        let cart3 = service.get_or_create_cart(3);

        assert_eq!(cart1.id, 1);
        assert_eq!(cart2.id, 2);
        assert_eq!(cart3.id, 3);
    }

    #[test]
    fn test_cart_item_count() {
        let service = CartService::new();
        let product1 = create_test_product(1, "Laptop", dec!(999.99), 10);
        let product2 = create_test_product(2, "Mouse", dec!(29.99), 50);

        let _ = service.add_item(1, &product1, 1);
        let cart = service.add_item(1, &product2, 2);

        assert_eq!(cart.item_count(), 2);
    }
}
