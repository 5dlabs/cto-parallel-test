use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with full details including ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product for creation (without ID).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for searching products.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Creates a new empty filter that matches all products.
    #[must_use]
    pub fn new() -> Self {
        Self {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }

    /// Sets the name filter to match products containing the given text (case-insensitive).
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name_contains = Some(name);
        self
    }

    /// Sets the minimum price filter.
    #[must_use]
    pub fn with_min_price(mut self, price: Decimal) -> Self {
        self.min_price = Some(price);
        self
    }

    /// Sets the maximum price filter.
    #[must_use]
    pub fn with_max_price(mut self, price: Decimal) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Sets the in-stock filter.
    #[must_use]
    pub fn with_in_stock(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
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
    fn test_product_clone() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        };

        let cloned = product.clone();
        assert_eq!(product, cloned);
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
    fn test_product_filter_default() {
        let filter = ProductFilter::default();
        assert!(filter.name_contains.is_none());
        assert!(filter.min_price.is_none());
        assert!(filter.max_price.is_none());
        assert!(filter.in_stock.is_none());
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(dec!(500.00))
            .with_max_price(dec!(2000.00))
            .with_in_stock(true);

        assert_eq!(filter.name_contains, Some("laptop".to_string()));
        assert_eq!(filter.min_price, Some(dec!(500.00)));
        assert_eq!(filter.max_price, Some(dec!(2000.00)));
        assert_eq!(filter.in_stock, Some(true));
    }

    #[test]
    fn test_decimal_precision() {
        let product = Product {
            id: 1,
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        };

        // Verify decimal precision is maintained
        assert_eq!(product.price.to_string(), "19.999");
    }

    #[test]
    fn test_serialization() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        };

        let json = serde_json::to_string(&product).expect("Serialization failed");
        let deserialized: Product = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(product, deserialized);
        assert_eq!(deserialized.price, dec!(19.99));
    }
}
