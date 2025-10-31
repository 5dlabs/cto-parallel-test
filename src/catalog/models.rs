//! Product catalog data models
//!
//! This module defines the data structures for products, including the full
//! product model with ID, a DTO for product creation without ID, and a filter
//! for querying products.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A product in the catalog with unique identifier
///
/// This represents a fully-formed product with an assigned ID. All products
/// returned from the service will use this structure.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Product {
    /// Unique product identifier
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price using Decimal for precise monetary calculations
    pub price: Decimal,
    /// Current inventory count
    pub inventory_count: i32,
}

/// A new product for creation (without ID)
///
/// This structure is used when creating a new product. The ID will be assigned
/// by the service upon creation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NewProduct {
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price using Decimal for precise monetary calculations
    pub price: Decimal,
    /// Initial inventory count
    pub inventory_count: i32,
}

/// Filter criteria for querying products
///
/// All fields are optional. When a field is `None`, no filtering is applied
/// for that criterion. Multiple filters are combined with AND logic.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
