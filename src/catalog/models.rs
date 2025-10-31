//! Product data models
//!
//! Defines the data structures for products and filtering.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with all fields including ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price (using Decimal for precision)
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Represents a new product to be created (without ID)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price (using Decimal for precision)
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for querying products
///
/// All fields are optional. When None, the filter doesn't restrict results.
/// All filters are combined with AND logic.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    /// Case-insensitive substring match for product name
    pub name_contains: Option<String>,
    /// Minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}
