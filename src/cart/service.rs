//! Shopping cart service implementation with database operations
//!
//! This service provides thread-safe cart management with:
//! - User-specific cart isolation
//! - Real database operations (no mocks)
//! - Inventory validation
//! - Concurrent access support

use crate::models::{Cart, CartItem, NewCart, NewCartItem, Product};
use crate::schema::{cart_items, carts, products};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Error types for cart operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartError {
    /// Database operation failed
    DatabaseError(String),
    /// Product not found
    ProductNotFound(i32),
    /// Insufficient inventory
    InsufficientInventory {
        product_id: i32,
        requested: i32,
        available: i32,
    },
    /// Cart item not found
    CartItemNotFound(i32),
    /// Invalid quantity
    InvalidQuantity(i32),
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {msg}"),
            Self::ProductNotFound(id) => write!(f, "Product not found: {id}"),
            Self::InsufficientInventory {
                product_id,
                requested,
                available,
            } => {
                write!(f, "Insufficient inventory for product {product_id}: requested {requested}, available {available}")
            }
            Self::CartItemNotFound(id) => write!(f, "Cart item not found: {id}"),
            Self::InvalidQuantity(qty) => write!(f, "Invalid quantity: {qty}"),
        }
    }
}

impl std::error::Error for CartError {}

/// Request to add an item to the cart
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AddItemRequest {
    /// Product ID to add
    pub product_id: i32,
    /// Quantity to add
    pub quantity: i32,
}

/// Cart with populated item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartWithItems {
    /// Cart information
    pub cart: Cart,
    /// Cart items with product details
    pub items: Vec<CartItemWithProduct>,
}

/// Cart item with product details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemWithProduct {
    /// Cart item information
    #[serde(flatten)]
    pub cart_item: CartItem,
    /// Product information
    pub product: Product,
}

/// Thread-safe shopping cart service with database operations
#[derive(Clone)]
pub struct CartService {
    pool: Arc<crate::config::Pool>,
}

impl CartService {
    /// Creates a new `CartService` instance
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    #[must_use]
    pub fn new(pool: Arc<crate::config::Pool>) -> Self {
        Self { pool }
    }

    /// Gets or creates a cart for the specified user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// The user's cart, creating a new one if it doesn't exist
    ///
    /// # Errors
    ///
    /// Returns `CartError::DatabaseError` if database operation fails
    pub fn get_or_create_cart(&self, user_id: i32) -> Result<Cart, CartError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| CartError::DatabaseError(format!("Failed to get connection: {e}")))?;

        // Try to find existing cart
        let existing_cart = carts::table
            .filter(carts::user_id.eq(user_id))
            .first::<Cart>(&mut conn)
            .optional()
            .map_err(|e| CartError::DatabaseError(format!("Failed to query cart: {e}")))?;

        if let Some(cart) = existing_cart {
            return Ok(cart);
        }

        // Create new cart if none exists
        let new_cart = NewCart { user_id };
        diesel::insert_into(carts::table)
            .values(&new_cart)
            .get_result(&mut conn)
            .map_err(|e| CartError::DatabaseError(format!("Failed to create cart: {e}")))
    }

    /// Adds an item to the user's cart with inventory validation
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    /// * `request` - Add item request with `product_id` and quantity
    ///
    /// # Returns
    ///
    /// The cart with all items and product details
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Product not found
    /// - Insufficient inventory
    /// - Invalid quantity (must be positive)
    /// - Database operation fails
    pub fn add_item(
        &self,
        user_id: i32,
        request: &AddItemRequest,
    ) -> Result<CartWithItems, CartError> {
        if request.quantity <= 0 {
            return Err(CartError::InvalidQuantity(request.quantity));
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| CartError::DatabaseError(format!("Failed to get connection: {e}")))?;

        // Get or create cart
        let cart = self.get_or_create_cart(user_id)?;

        // Validate product exists and has sufficient inventory
        let product = products::table
            .find(request.product_id)
            .first::<Product>(&mut conn)
            .optional()
            .map_err(|e| CartError::DatabaseError(format!("Failed to query product: {e}")))?
            .ok_or(CartError::ProductNotFound(request.product_id))?;

        if product.inventory_count < request.quantity {
            return Err(CartError::InsufficientInventory {
                product_id: request.product_id,
                requested: request.quantity,
                available: product.inventory_count,
            });
        }

        // Check if item already exists in cart
        let existing_item = cart_items::table
            .filter(cart_items::cart_id.eq(cart.id))
            .filter(cart_items::product_id.eq(request.product_id))
            .first::<CartItem>(&mut conn)
            .optional()
            .map_err(|e| CartError::DatabaseError(format!("Failed to query cart item: {e}")))?;

        if let Some(item) = existing_item {
            // Update existing item quantity
            let new_quantity = item.quantity + request.quantity;

            // Validate total quantity doesn't exceed inventory
            if product.inventory_count < new_quantity {
                return Err(CartError::InsufficientInventory {
                    product_id: request.product_id,
                    requested: new_quantity,
                    available: product.inventory_count,
                });
            }

            diesel::update(cart_items::table.find(item.id))
                .set(cart_items::quantity.eq(new_quantity))
                .execute(&mut conn)
                .map_err(|e| {
                    CartError::DatabaseError(format!("Failed to update cart item: {e}"))
                })?;
        } else {
            // Add new item to cart
            let new_item = NewCartItem {
                cart_id: cart.id,
                product_id: request.product_id,
                quantity: request.quantity,
            };

            diesel::insert_into(cart_items::table)
                .values(&new_item)
                .execute(&mut conn)
                .map_err(|e| CartError::DatabaseError(format!("Failed to add cart item: {e}")))?;
        }

        // Return updated cart with items
        self.get_cart(user_id)
    }

    /// Removes an item from the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    /// * `product_id` - The ID of the product to remove
    ///
    /// # Returns
    ///
    /// The cart with all remaining items and product details
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Cart item not found
    /// - Database operation fails
    pub fn remove_item(&self, user_id: i32, product_id: i32) -> Result<CartWithItems, CartError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| CartError::DatabaseError(format!("Failed to get connection: {e}")))?;

        // Get cart
        let cart = self.get_or_create_cart(user_id)?;

        // Find and delete the cart item
        let deleted = diesel::delete(
            cart_items::table
                .filter(cart_items::cart_id.eq(cart.id))
                .filter(cart_items::product_id.eq(product_id)),
        )
        .execute(&mut conn)
        .map_err(|e| CartError::DatabaseError(format!("Failed to remove cart item: {e}")))?;

        if deleted == 0 {
            return Err(CartError::CartItemNotFound(product_id));
        }

        // Return updated cart with items
        self.get_cart(user_id)
    }

    /// Gets the user's cart with all items and product details
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// The cart with all items and product details
    ///
    /// # Errors
    ///
    /// Returns `CartError::DatabaseError` if database operation fails
    pub fn get_cart(&self, user_id: i32) -> Result<CartWithItems, CartError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| CartError::DatabaseError(format!("Failed to get connection: {e}")))?;

        // Get or create cart
        let cart = self.get_or_create_cart(user_id)?;

        // Get all cart items with product details
        let items = cart_items::table
            .filter(cart_items::cart_id.eq(cart.id))
            .inner_join(products::table)
            .select((CartItem::as_select(), Product::as_select()))
            .load::<(CartItem, Product)>(&mut conn)
            .map_err(|e| CartError::DatabaseError(format!("Failed to query cart items: {e}")))?
            .into_iter()
            .map(|(cart_item, product)| CartItemWithProduct { cart_item, product })
            .collect();

        Ok(CartWithItems { cart, items })
    }

    /// Clears all items from the user's cart
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    ///
    /// The empty cart
    ///
    /// # Errors
    ///
    /// Returns `CartError::DatabaseError` if database operation fails
    pub fn clear_cart(&self, user_id: i32) -> Result<CartWithItems, CartError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| CartError::DatabaseError(format!("Failed to get connection: {e}")))?;

        // Get or create cart
        let cart = self.get_or_create_cart(user_id)?;

        // Delete all items from cart
        diesel::delete(cart_items::table.filter(cart_items::cart_id.eq(cart.id)))
            .execute(&mut conn)
            .map_err(|e| CartError::DatabaseError(format!("Failed to clear cart: {e}")))?;

        // Return empty cart
        Ok(CartWithItems {
            cart,
            items: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::establish_connection_pool;
    use crate::models::{NewProduct, NewUser};
    use crate::schema::users;
    use rust_decimal_macros::dec;

    fn setup_test_db() -> Arc<crate::config::Pool> {
        dotenv::dotenv().ok();
        Arc::new(establish_connection_pool())
    }

    fn create_test_user(pool: &crate::config::Pool, username: &str) -> i32 {
        let mut conn = pool.get().expect("Failed to get connection");
        let new_user = NewUser {
            username: username.to_string(),
            email: format!("{username}@test.com"),
            password_hash: "test_hash".to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(users::id)
            .get_result(&mut conn)
            .expect("Failed to create test user")
    }

    fn create_test_product(pool: &crate::config::Pool, name: &str, inventory: i32) -> i32 {
        let mut conn = pool.get().expect("Failed to get connection");
        let new_product = NewProduct {
            name: name.to_string(),
            description: Some(format!("Test product: {name}")),
            price: dec!(99.99),
            inventory_count: inventory,
        };

        diesel::insert_into(products::table)
            .values(&new_product)
            .returning(products::id)
            .get_result(&mut conn)
            .expect("Failed to create test product")
    }

    #[test]
    fn test_get_or_create_cart_creates_new() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_1");

        let cart = service
            .get_or_create_cart(user_id)
            .expect("Failed to create cart");

        assert_eq!(cart.user_id, user_id);
    }

    #[test]
    fn test_get_or_create_cart_returns_existing() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_2");

        let cart1 = service
            .get_or_create_cart(user_id)
            .expect("Failed to create cart");
        let cart2 = service
            .get_or_create_cart(user_id)
            .expect("Failed to get cart");

        assert_eq!(cart1.id, cart2.id);
        assert_eq!(cart1.user_id, cart2.user_id);
    }

    #[test]
    fn test_add_item_success() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_3");
        let product_id = create_test_product(&pool, "Test Product", 10);

        let request = AddItemRequest {
            product_id,
            quantity: 2,
        };

        let cart = service
            .add_item(user_id, &request)
            .expect("Failed to add item");

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].cart_item.product_id, product_id);
        assert_eq!(cart.items[0].cart_item.quantity, 2);
    }

    #[test]
    fn test_add_item_updates_quantity() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_4");
        let product_id = create_test_product(&pool, "Test Product", 10);

        let request1 = AddItemRequest {
            product_id,
            quantity: 2,
        };
        service
            .add_item(user_id, &request1)
            .expect("Failed to add item");

        let request2 = AddItemRequest {
            product_id,
            quantity: 3,
        };
        let cart = service
            .add_item(user_id, &request2)
            .expect("Failed to add item");

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].cart_item.quantity, 5);
    }

    #[test]
    fn test_add_item_insufficient_inventory() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_5");
        let product_id = create_test_product(&pool, "Test Product", 5);

        let request = AddItemRequest {
            product_id,
            quantity: 10,
        };

        let result = service.add_item(user_id, &request);

        assert!(result.is_err());
        match result.unwrap_err() {
            CartError::InsufficientInventory {
                requested,
                available,
                ..
            } => {
                assert_eq!(requested, 10);
                assert_eq!(available, 5);
            }
            _ => panic!("Expected InsufficientInventory error"),
        }
    }

    #[test]
    fn test_add_item_invalid_quantity() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_6");
        let product_id = create_test_product(&pool, "Test Product", 10);

        let request = AddItemRequest {
            product_id,
            quantity: 0,
        };

        let result = service.add_item(user_id, &request);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CartError::InvalidQuantity(0)));
    }

    #[test]
    fn test_add_item_product_not_found() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_7");

        let request = AddItemRequest {
            product_id: 999_999,
            quantity: 1,
        };

        let result = service.add_item(user_id, &request);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CartError::ProductNotFound(999_999)
        ));
    }

    #[test]
    fn test_remove_item_success() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_8");
        let product_id = create_test_product(&pool, "Test Product", 10);

        let request = AddItemRequest {
            product_id,
            quantity: 2,
        };
        service
            .add_item(user_id, &request)
            .expect("Failed to add item");

        let cart = service
            .remove_item(user_id, product_id)
            .expect("Failed to remove item");

        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_remove_item_not_found() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_9");

        let result = service.remove_item(user_id, 999_999);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CartError::CartItemNotFound(999_999)
        ));
    }

    #[test]
    fn test_get_cart_empty() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_10");

        let cart = service.get_cart(user_id).expect("Failed to get cart");

        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_get_cart_with_items() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_11");
        let product1 = create_test_product(&pool, "Product 1", 10);
        let product2 = create_test_product(&pool, "Product 2", 10);

        service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id: product1,
                    quantity: 2,
                },
            )
            .expect("Failed to add item");
        service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id: product2,
                    quantity: 3,
                },
            )
            .expect("Failed to add item");

        let cart = service.get_cart(user_id).expect("Failed to get cart");

        assert_eq!(cart.items.len(), 2);
    }

    #[test]
    fn test_clear_cart_success() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user_id = create_test_user(&pool, "cart_test_user_12");
        let product1 = create_test_product(&pool, "Product 1", 10);
        let product2 = create_test_product(&pool, "Product 2", 10);

        service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id: product1,
                    quantity: 2,
                },
            )
            .expect("Failed to add item");
        service
            .add_item(
                user_id,
                &AddItemRequest {
                    product_id: product2,
                    quantity: 3,
                },
            )
            .expect("Failed to add item");

        let cart = service.clear_cart(user_id).expect("Failed to clear cart");

        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_cart_isolation_between_users() {
        let pool = setup_test_db();
        let service = CartService::new(pool.clone());
        let user1 = create_test_user(&pool, "cart_test_user_13");
        let user2 = create_test_user(&pool, "cart_test_user_14");
        let product = create_test_product(&pool, "Test Product", 10);

        service
            .add_item(
                user1,
                &AddItemRequest {
                    product_id: product,
                    quantity: 2,
                },
            )
            .expect("Failed to add item");

        let cart1 = service.get_cart(user1).expect("Failed to get cart");
        let cart2 = service.get_cart(user2).expect("Failed to get cart");

        assert_eq!(cart1.items.len(), 1);
        assert_eq!(cart2.items.len(), 0);
    }
}
