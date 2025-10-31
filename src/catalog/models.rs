//! Product data models

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price (using Decimal for exact decimal arithmetic)
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product (without ID)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price (using Decimal for exact decimal arithmetic)
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for querying products
///
/// All filter fields are optional. When a field is `None`, it doesn't restrict results.
/// Multiple filters combine with AND logic.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductFilter {
    /// Filter by name substring (case-insensitive)
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock availability (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}
