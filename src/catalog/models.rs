//! Product data models

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a complete product entity with an assigned ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    /// Unique product identifier (auto-generated)
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

/// Data transfer object for creating a new product (without ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    /// Creates a new empty filter that matches all products
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name filter (builder pattern)
    #[must_use]
    pub fn with_name_contains(mut self, name: String) -> Self {
        self.name_contains = Some(name);
        self
    }

    /// Sets the minimum price filter (builder pattern)
    #[must_use]
    pub fn with_min_price(mut self, price: Decimal) -> Self {
        self.min_price = Some(price);
        self
    }

    /// Sets the maximum price filter (builder pattern)
    #[must_use]
    pub fn with_max_price(mut self, price: Decimal) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Sets the in-stock filter (builder pattern)
    #[must_use]
    pub fn with_in_stock(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_product_creation() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "New Product".to_string(),
            description: "A new product".to_string(),
            price: dec!(29.99),
            inventory_count: 5,
        };

        assert_eq!(new_product.name, "New Product");
        assert_eq!(new_product.price, dec!(29.99));
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name_contains("laptop".to_string())
            .with_min_price(dec!(100.00))
            .with_max_price(dec!(1000.00))
            .with_in_stock(true);

        assert_eq!(filter.name_contains, Some("laptop".to_string()));
        assert_eq!(filter.min_price, Some(dec!(100.00)));
        assert_eq!(filter.max_price, Some(dec!(1000.00)));
        assert_eq!(filter.in_stock, Some(true));
    }

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Description".to_string(),
            price: dec!(9.99),
            inventory_count: 5,
        };

        let json = serde_json::to_string(&product).expect("Failed to serialize");
        let deserialized: Product = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(product, deserialized);
    }
}
