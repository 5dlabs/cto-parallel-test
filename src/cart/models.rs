use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a single item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
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

    /// Calculate the total price for this cart item
    #[must_use]
    pub fn total_price(&self) -> Decimal {
        self.unit_price * Decimal::from(self.quantity)
    }
}

/// Represents a shopping cart for a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
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

    /// Calculate the total price of all items in the cart
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.items.iter().map(CartItem::total_price).sum()
    }

    /// Get the total number of items in the cart
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get the total quantity of all items
    #[must_use]
    pub fn total_quantity(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_cart_item_new() {
        let item = CartItem::new(
            1,
            2,
            "Test Product".to_string(),
            Decimal::from_str("10.50").unwrap(),
        );

        assert_eq!(item.product_id, 1);
        assert_eq!(item.quantity, 2);
        assert_eq!(item.product_name, "Test Product");
        assert_eq!(item.unit_price, Decimal::from_str("10.50").unwrap());
    }

    #[test]
    fn test_cart_item_total_price() {
        let item = CartItem::new(
            1,
            3,
            "Widget".to_string(),
            Decimal::from_str("5.99").unwrap(),
        );

        assert_eq!(item.total_price(), Decimal::from_str("17.97").unwrap());
    }

    #[test]
    fn test_cart_new() {
        let cart = Cart::new(1, 100);

        assert_eq!(cart.id, 1);
        assert_eq!(cart.user_id, 100);
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_cart_total_empty() {
        let cart = Cart::new(1, 100);
        assert_eq!(cart.total(), Decimal::from_str("0").unwrap());
    }

    #[test]
    fn test_cart_total_with_items() {
        let mut cart = Cart::new(1, 100);

        cart.items.push(CartItem::new(
            1,
            2,
            "Item 1".to_string(),
            Decimal::from_str("10.00").unwrap(),
        ));

        cart.items.push(CartItem::new(
            2,
            1,
            "Item 2".to_string(),
            Decimal::from_str("25.50").unwrap(),
        ));

        assert_eq!(cart.total(), Decimal::from_str("45.50").unwrap());
    }

    #[test]
    fn test_cart_item_count() {
        let mut cart = Cart::new(1, 100);
        assert_eq!(cart.item_count(), 0);

        cart.items.push(CartItem::new(
            1,
            2,
            "Item 1".to_string(),
            Decimal::from_str("10.00").unwrap(),
        ));

        assert_eq!(cart.item_count(), 1);

        cart.items.push(CartItem::new(
            2,
            1,
            "Item 2".to_string(),
            Decimal::from_str("25.50").unwrap(),
        ));

        assert_eq!(cart.item_count(), 2);
    }

    #[test]
    fn test_cart_total_quantity() {
        let mut cart = Cart::new(1, 100);
        assert_eq!(cart.total_quantity(), 0);

        cart.items.push(CartItem::new(
            1,
            3,
            "Item 1".to_string(),
            Decimal::from_str("10.00").unwrap(),
        ));

        cart.items.push(CartItem::new(
            2,
            5,
            "Item 2".to_string(),
            Decimal::from_str("25.50").unwrap(),
        ));

        assert_eq!(cart.total_quantity(), 8);
    }
}
