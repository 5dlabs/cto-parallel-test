use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// DTO for creating a new product
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
    /// Creates a new empty product filter
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name filter (case-insensitive substring match)
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name_contains = Some(name.into());
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
            inventory_count: 10,
        };

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name("laptop")
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
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }
}
