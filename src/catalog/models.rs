use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub inventory_count: i32,
}

impl NewProduct {
    /// Creates a new `NewProduct` instance
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the product
    /// * `description` - Optional description of the product
    /// * `price` - The price of the product
    /// * `inventory_count` - The initial inventory count
    #[must_use]
    pub fn new(
        name: String,
        description: Option<String>,
        price: Decimal,
        inventory_count: i32,
    ) -> Self {
        Self {
            name,
            description,
            price,
            inventory_count,
        }
    }
}

/// Filter criteria for searching products
#[derive(Debug, Clone, Default)]
pub struct ProductFilter {
    pub name: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock_only: bool,
}

impl ProductFilter {
    /// Creates a new empty `ProductFilter`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name filter (case-insensitive substring match)
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the minimum price filter
    #[must_use]
    pub fn with_min_price(mut self, min_price: Decimal) -> Self {
        self.min_price = Some(min_price);
        self
    }

    /// Sets the maximum price filter
    #[must_use]
    pub fn with_max_price(mut self, max_price: Decimal) -> Self {
        self.max_price = Some(max_price);
        self
    }

    /// Sets the in-stock-only filter
    #[must_use]
    pub fn with_in_stock_only(mut self, in_stock_only: bool) -> Self {
        self.in_stock_only = in_stock_only;
        self
    }

    /// Checks if a product matches the filter criteria
    ///
    /// # Arguments
    ///
    /// * `product` - The product to check against the filter
    ///
    /// # Returns
    ///
    /// `true` if the product matches all filter criteria, `false` otherwise
    #[must_use]
    pub fn matches(&self, product: &Product) -> bool {
        // Check name filter (case-insensitive substring match)
        if let Some(ref name) = self.name {
            if !product.name.to_lowercase().contains(&name.to_lowercase()) {
                return false;
            }
        }

        // Check minimum price filter
        if let Some(min_price) = self.min_price {
            if product.price < min_price {
                return false;
            }
        }

        // Check maximum price filter
        if let Some(max_price) = self.max_price {
            if product.price > max_price {
                return false;
            }
        }

        // Check in-stock filter
        if self.in_stock_only && product.inventory_count <= 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_product_creation() {
        let new_product = NewProduct::new(
            "Test Product".to_string(),
            Some("Test Description".to_string()),
            dec!(99.99),
            10,
        );

        assert_eq!(new_product.name, "Test Product");
        assert_eq!(
            new_product.description,
            Some("Test Description".to_string())
        );
        assert_eq!(new_product.price, dec!(99.99));
        assert_eq!(new_product.inventory_count, 10);
    }

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(dec!(500.00))
            .with_max_price(dec!(2000.00))
            .with_in_stock_only(true);

        assert_eq!(filter.name, Some("laptop".to_string()));
        assert_eq!(filter.min_price, Some(dec!(500.00)));
        assert_eq!(filter.max_price, Some(dec!(2000.00)));
        assert!(filter.in_stock_only);
    }

    #[test]
    fn test_filter_matches_name() {
        let product = Product {
            id: 1,
            name: "Gaming Laptop".to_string(),
            description: None,
            price: dec!(1500.00),
            inventory_count: 5,
        };

        let filter = ProductFilter::new().with_name("laptop".to_string());
        assert!(filter.matches(&product));

        let filter = ProductFilter::new().with_name("desktop".to_string());
        assert!(!filter.matches(&product));
    }

    #[test]
    fn test_filter_matches_name_case_insensitive() {
        let product = Product {
            id: 1,
            name: "Gaming Laptop".to_string(),
            description: None,
            price: dec!(1500.00),
            inventory_count: 5,
        };

        let filter = ProductFilter::new().with_name("LAPTOP".to_string());
        assert!(filter.matches(&product));
    }

    #[test]
    fn test_filter_matches_price_range() {
        let product = Product {
            id: 1,
            name: "Gaming Laptop".to_string(),
            description: None,
            price: dec!(1500.00),
            inventory_count: 5,
        };

        let filter = ProductFilter::new()
            .with_min_price(dec!(1000.00))
            .with_max_price(dec!(2000.00));
        assert!(filter.matches(&product));

        let filter = ProductFilter::new()
            .with_min_price(dec!(2000.00))
            .with_max_price(dec!(3000.00));
        assert!(!filter.matches(&product));
    }

    #[test]
    fn test_filter_matches_in_stock() {
        let in_stock_product = Product {
            id: 1,
            name: "In Stock Product".to_string(),
            description: None,
            price: dec!(100.00),
            inventory_count: 5,
        };

        let out_of_stock_product = Product {
            id: 2,
            name: "Out of Stock Product".to_string(),
            description: None,
            price: dec!(100.00),
            inventory_count: 0,
        };

        let filter = ProductFilter::new().with_in_stock_only(true);
        assert!(filter.matches(&in_stock_product));
        assert!(!filter.matches(&out_of_stock_product));

        let filter = ProductFilter::new().with_in_stock_only(false);
        assert!(filter.matches(&in_stock_product));
        assert!(filter.matches(&out_of_stock_product));
    }

    #[test]
    fn test_filter_matches_combined_criteria() {
        let product = Product {
            id: 1,
            name: "Gaming Laptop".to_string(),
            description: None,
            price: dec!(1500.00),
            inventory_count: 5,
        };

        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(dec!(1000.00))
            .with_max_price(dec!(2000.00))
            .with_in_stock_only(true);

        assert!(filter.matches(&product));
    }
}
