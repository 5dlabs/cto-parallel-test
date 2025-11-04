//! Product data models
//!
//! This module defines the data structures for products, including
//! the full product entity, creation DTO, and filter criteria.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price with decimal precision
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price with decimal precision
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for searching products
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ProductFilter {
    /// Filter by name (case-insensitive substring match)
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name filter
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name_contains = Some(name);
        self
    }

    /// Sets the minimum price filter
    #[must_use]
    pub fn with_min_price(mut self, price: Decimal) -> Self {
        self.min_price = Some(price);
        self
    }

    /// Sets the maximum price filter
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
            inventory_count: 100,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.inventory_count, 100);
    }

    #[test]
    fn test_new_product() {
        let new_product = NewProduct {
            name: "New Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 50,
        };

        assert_eq!(new_product.name, "New Product");
        assert_eq!(new_product.inventory_count, 50);
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(Decimal::from_str("100.00").unwrap())
            .with_max_price(Decimal::from_str("2000.00").unwrap())
            .with_in_stock(true);

        assert_eq!(filter.name_contains, Some("laptop".to_string()));
        assert_eq!(filter.min_price, Some(Decimal::from_str("100.00").unwrap()));
        assert_eq!(
            filter.max_price,
            Some(Decimal::from_str("2000.00").unwrap())
        );
        assert_eq!(filter.in_stock, Some(true));
    }

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::from_str("10.50").unwrap(),
            inventory_count: 5,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }

    #[test]
    fn test_decimal_precision() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        // Verify decimal precision is maintained
        assert_eq!(product.price.to_string(), "19.99");
    }
}
