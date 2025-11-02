//! Product data models for the catalog module.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a complete product entity with an ID.
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

/// Represents a new product to be created (without an ID).
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

/// Represents filtering criteria for product searches.
#[derive(Debug, Serialize, Deserialize, Default)]
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
    /// Creates a new empty product filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductFilter;
    ///
    /// let filter = ProductFilter::new();
    /// assert!(filter.name_contains.is_none());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }

    /// Creates a filter for products matching a name substring.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductFilter;
    ///
    /// let filter = ProductFilter::with_name("laptop");
    /// assert_eq!(filter.name_contains, Some("laptop".to_string()));
    /// ```
    #[must_use]
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name_contains: Some(name.into()),
            ..Self::default()
        }
    }

    /// Creates a filter for products within a price range.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductFilter;
    /// use rust_decimal_macros::dec;
    ///
    /// let filter = ProductFilter::with_price_range(Some(dec!(10.00)), Some(dec!(100.00)));
    /// assert_eq!(filter.min_price, Some(dec!(10.00)));
    /// assert_eq!(filter.max_price, Some(dec!(100.00)));
    /// ```
    #[must_use]
    pub const fn with_price_range(min: Option<Decimal>, max: Option<Decimal>) -> Self {
        Self {
            name_contains: None,
            min_price: min,
            max_price: max,
            in_stock: None,
        }
    }

    /// Creates a filter for products by stock status.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductFilter;
    ///
    /// let filter = ProductFilter::with_stock_status(true);
    /// assert_eq!(filter.in_stock, Some(true));
    /// ```
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
        assert_eq!(new_product.inventory_count, 5);
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
        let filter = ProductFilter::with_price_range(Some(dec!(10.00)), Some(dec!(100.00)));
        assert_eq!(filter.min_price, Some(dec!(10.00)));
        assert_eq!(filter.max_price, Some(dec!(100.00)));
    }

    #[test]
    fn test_product_filter_with_stock_status() {
        let filter = ProductFilter::with_stock_status(true);
        assert_eq!(filter.in_stock, Some(true));
    }

    #[test]
    fn test_product_clone() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: dec!(9.99),
            inventory_count: 5,
        };

        let cloned = product.clone();
        assert_eq!(product, cloned);
    }
}
