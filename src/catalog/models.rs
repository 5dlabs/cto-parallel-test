//! Data models for the product catalog.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with a unique ID.
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

/// Represents a new product to be created (without ID).
#[derive(Debug, Serialize, Deserialize)]
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

/// Filter criteria for searching products.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductFilter {
    /// Filter by name substring (case-insensitive)
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter (returns all products).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
            description: "Test Description".to_string(),
            price: dec!(19.99),
            inventory_count: 100,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 100);
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "New Product".to_string(),
            description: "New Description".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        };

        assert_eq!(new_product.name, "New Product");
        assert_eq!(new_product.price, dec!(29.99));
    }

    #[test]
    fn test_product_filter_default() {
        let filter = ProductFilter::new();
        assert!(filter.name_contains.is_none());
        assert!(filter.min_price.is_none());
        assert!(filter.max_price.is_none());
        assert!(filter.in_stock.is_none());
    }

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: dec!(10.50),
            inventory_count: 5,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }
}
