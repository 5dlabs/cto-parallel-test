//! Product data models
//!
//! This module defines the core product data structures used throughout the catalog system.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with full details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Detailed product description
    pub description: String,
    /// Product price with decimal precision
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Detailed product description
    pub description: String,
    /// Product price with decimal precision
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for searching products
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    /// Filter by products containing this string in their name (case-insensitive)
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status: true for in-stock only, false for out-of-stock only
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter that matches all products
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name filter (case-insensitive substring match)
    #[must_use]
    pub fn with_name_contains(mut self, name: String) -> Self {
        self.name_contains = Some(name);
        self
    }

    /// Sets the minimum price filter (inclusive)
    #[must_use]
    pub fn with_min_price(mut self, price: Decimal) -> Self {
        self.min_price = Some(price);
        self
    }

    /// Sets the maximum price filter (inclusive)
    #[must_use]
    pub fn with_max_price(mut self, price: Decimal) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Sets the stock status filter
    #[must_use]
    pub fn with_in_stock(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_product_creation() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_product_clone() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let cloned = product.clone();
        assert_eq!(product, cloned);
    }

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }

    #[test]
    fn test_new_product_serialization() {
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let json = serde_json::to_string(&new_product).unwrap();
        let deserialized: NewProduct = serde_json::from_str(&json).unwrap();

        assert_eq!(new_product.name, deserialized.name);
        assert_eq!(new_product.price, deserialized.price);
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name_contains("laptop".to_string())
            .with_min_price(Decimal::from_str("100.00").unwrap())
            .with_max_price(Decimal::from_str("1000.00").unwrap())
            .with_in_stock(true);

        assert_eq!(filter.name_contains, Some("laptop".to_string()));
        assert_eq!(filter.min_price, Some(Decimal::from_str("100.00").unwrap()));
        assert_eq!(
            filter.max_price,
            Some(Decimal::from_str("1000.00").unwrap())
        );
        assert_eq!(filter.in_stock, Some(true));
    }

    #[test]
    fn test_decimal_precision() {
        let price = Decimal::from_str("19.99").unwrap();
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Test".to_string(),
            price,
            inventory_count: 1,
        };

        // Verify decimal precision is maintained through serialization
        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.price.to_string(), "19.99");
    }
}
