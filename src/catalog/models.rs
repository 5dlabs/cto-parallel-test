use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with a unique identifier.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product without an ID (used for product creation).
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for querying products. All fields are optional.
/// When multiple filters are specified, they are combined with AND logic.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    /// Case-insensitive substring match on product name
    pub name_contains: Option<String>,
    /// Minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}
