//! Cart data models
//!
//! This module defines the data structures for shopping cart items and carts.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a single item in a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    /// The product ID
    pub product_id: i32,
    /// The quantity of this product in the cart
    pub quantity: i32,
    /// The name of the product (cached for convenience)
    pub product_name: String,
    /// The unit price of the product (cached at time of adding to cart)
    pub unit_price: Decimal,
}

impl CartItem {
    /// Creates a new cart item
    #[must_use]
    pub fn new(product_id: i32, quantity: i32, product_name: String, unit_price: Decimal) -> Self {
        Self {
            product_id,
            quantity,
            product_name,
            unit_price,
        }
    }

    /// Calculates the total price for this cart item (`quantity * unit_price`)
    #[must_use]
    pub fn total_price(&self) -> Decimal {
        self.unit_price * Decimal::from(self.quantity)
    }
}

/// Represents a user's shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    /// The cart ID (auto-generated)
    pub id: i32,
    /// The user ID who owns this cart
    pub user_id: i32,
    /// The list of items in the cart
    pub items: Vec<CartItem>,
}

impl Cart {
    /// Creates a new empty cart for a user
    #[must_use]
    pub fn new(id: i32, user_id: i32) -> Self {
        Self {
            id,
            user_id,
            items: Vec::new(),
        }
    }

    /// Calculates the total price of all items in the cart
    #[must_use]
    pub fn total_price(&self) -> Decimal {
        self.items.iter().map(CartItem::total_price).sum()
    }

    /// Returns the number of items in the cart
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Returns the total quantity of all items in the cart
    #[must_use]
    pub fn total_quantity(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_cart_item_creation() {
        let item = CartItem::new(1, 2, "Test Product".to_string(), dec!(10.50));
        assert_eq!(item.product_id, 1);
        assert_eq!(item.quantity, 2);
        assert_eq!(item.product_name, "Test Product");
        assert_eq!(item.unit_price, dec!(10.50));
    }

    #[test]
    fn test_cart_item_total_price() {
        let item = CartItem::new(1, 3, "Test Product".to_string(), dec!(10.00));
        assert_eq!(item.total_price(), dec!(30.00));
    }

    #[test]
    fn test_cart_item_total_price_with_decimal() {
        let item = CartItem::new(1, 2, "Test Product".to_string(), dec!(19.99));
        assert_eq!(item.total_price(), dec!(39.98));
    }

    #[test]
    fn test_cart_creation() {
        let cart = Cart::new(1, 100);
        assert_eq!(cart.id, 1);
        assert_eq!(cart.user_id, 100);
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_empty_cart_total_price() {
        let cart = Cart::new(1, 100);
        assert_eq!(cart.total_price(), dec!(0));
    }

    #[test]
    fn test_cart_with_items_total_price() {
        let mut cart = Cart::new(1, 100);
        cart.items
            .push(CartItem::new(1, 2, "Product 1".to_string(), dec!(10.00)));
        cart.items
            .push(CartItem::new(2, 1, "Product 2".to_string(), dec!(5.00)));

        assert_eq!(cart.total_price(), dec!(25.00));
    }

    #[test]
    fn test_cart_item_count() {
        let mut cart = Cart::new(1, 100);
        assert_eq!(cart.item_count(), 0);

        cart.items
            .push(CartItem::new(1, 2, "Product 1".to_string(), dec!(10.00)));
        assert_eq!(cart.item_count(), 1);

        cart.items
            .push(CartItem::new(2, 1, "Product 2".to_string(), dec!(5.00)));
        assert_eq!(cart.item_count(), 2);
    }

    #[test]
    fn test_cart_total_quantity() {
        let mut cart = Cart::new(1, 100);
        assert_eq!(cart.total_quantity(), 0);

        cart.items
            .push(CartItem::new(1, 3, "Product 1".to_string(), dec!(10.00)));
        assert_eq!(cart.total_quantity(), 3);

        cart.items
            .push(CartItem::new(2, 2, "Product 2".to_string(), dec!(5.00)));
        assert_eq!(cart.total_quantity(), 5);
    }

    #[test]
    fn test_cart_item_equality() {
        let item1 = CartItem::new(1, 2, "Product".to_string(), dec!(10.00));
        let item2 = CartItem::new(1, 2, "Product".to_string(), dec!(10.00));
        let item3 = CartItem::new(1, 3, "Product".to_string(), dec!(10.00));

        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_cart_equality() {
        let cart1 = Cart::new(1, 100);
        let cart2 = Cart::new(1, 100);
        let cart3 = Cart::new(2, 100);

        assert_eq!(cart1, cart2);
        assert_ne!(cart1, cart3);
    }
}
