use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with full information including ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product to be created (without ID)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for searching products
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
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
            inventory_count: 100,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "New Product".to_string(),
            description: "A new product".to_string(),
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
            inventory_count: 10,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(product, deserialized);
    }
}
