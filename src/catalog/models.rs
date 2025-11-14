use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A product in the catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Product {
    /// Auto-incremented unique identifier.
    pub id: i32,
    /// Display name.
    pub name: String,
    /// Detailed description.
    pub description: String,
    /// Price with exact decimal precision.
    pub price: Decimal,
    /// Units available in stock.
    pub inventory_count: i32,
}

/// Input payload for creating a new product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria to query products.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ProductFilter {
    /// Case-insensitive substring check on the name.
    pub name_contains: Option<String>,
    /// Inclusive minimum price.
    pub min_price: Option<Decimal>,
    /// Inclusive maximum price.
    pub max_price: Option<Decimal>,
    /// If set, true only returns items with `inventory_count` > 0; false returns `inventory_count` == 0.
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
