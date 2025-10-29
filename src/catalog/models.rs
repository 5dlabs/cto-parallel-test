//! Product catalog data models
//!
//! Defines the data structures for products, product creation, and filtering.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with all fields including ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Detailed product description
    pub description: String,
    /// Product price using decimal for precision
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// Data transfer object for creating a new product (no ID field)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Detailed product description
    pub description: String,
    /// Product price using decimal for precision
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for querying products
///
/// All fields are optional. None means no filter is applied for that criterion.
/// Multiple filters are combined with AND logic.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    /// Filter by name containing this substring (case-insensitive)
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status (true = in stock, false = out of stock)
    pub in_stock: Option<bool>,
}
