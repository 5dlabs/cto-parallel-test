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

/// Data required to create a new product
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
    /// Create a new empty filter
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name filter (case-insensitive substring match)
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name_contains = Some(name);
        self
    }

    /// Set minimum price filter
    #[must_use]
    pub fn with_min_price(mut self, min: Decimal) -> Self {
        self.min_price = Some(min);
        self
    }

    /// Set maximum price filter
    #[must_use]
    pub fn with_max_price(mut self, max: Decimal) -> Self {
        self.max_price = Some(max);
        self
    }

    /// Set stock status filter
    #[must_use]
    pub fn with_in_stock(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
    }
}
