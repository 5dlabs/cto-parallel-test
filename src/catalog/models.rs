use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with all its details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product to be created (without an ID)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for searching products
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
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
    pub fn with_stock_status(mut self, in_stock: bool) -> Self {
        self.in_stock = Some(in_stock);
        self
    }
}
