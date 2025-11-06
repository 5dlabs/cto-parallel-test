use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with full details including ID
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter (matches all products)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }

    /// Creates a filter for products containing the given name
    #[must_use]
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name_contains: Some(name.into()),
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }

    /// Creates a filter for products within a price range
    #[must_use]
    pub const fn with_price_range(min: Option<Decimal>, max: Option<Decimal>) -> Self {
        Self {
            name_contains: None,
            min_price: min,
            max_price: max,
            in_stock: None,
        }
    }

    /// Creates a filter for in-stock or out-of-stock products
    #[must_use]
    pub const fn with_stock_status(in_stock: bool) -> Self {
        Self {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(in_stock),
        }
    }
}

impl Default for ProductFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::*;

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
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        };

        let cloned = product.clone();
        assert_eq!(product, cloned);
    }

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct {
            name: "New Product".to_string(),
            description: "A new product".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 20,
        };

        assert_eq!(new_product.name, "New Product");
        assert_eq!(new_product.price, Decimal::from_str("29.99").unwrap());
    }

    #[test]
    fn test_product_filter_new() {
        let filter = ProductFilter::new();
        assert!(filter.name_contains.is_none());
        assert!(filter.min_price.is_none());
        assert!(filter.max_price.is_none());
        assert!(filter.in_stock.is_none());
    }

    #[test]
    fn test_product_filter_with_name() {
        let filter = ProductFilter::with_name("laptop");
        assert_eq!(filter.name_contains, Some("laptop".to_string()));
    }

    #[test]
    fn test_product_filter_with_price_range() {
        let min = Decimal::from_str("10.00").unwrap();
        let max = Decimal::from_str("50.00").unwrap();
        let filter = ProductFilter::with_price_range(Some(min), Some(max));

        assert_eq!(filter.min_price, Some(min));
        assert_eq!(filter.max_price, Some(max));
    }

    #[test]
    fn test_product_filter_with_stock_status() {
        let filter = ProductFilter::with_stock_status(true);
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

        // Verify decimal precision is maintained
        assert_eq!(product.price.to_string(), "19.99");
    }

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }
}
