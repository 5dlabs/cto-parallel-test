use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with full details including ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product to be created (without ID).
/// Used as a DTO for product creation requests.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for querying products.
/// All fields are optional - None means no filter applied for that field.
/// Multiple filters are combined with AND logic.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductFilter {
    /// Case-insensitive substring match on product name
    pub name_contains: Option<String>,
    /// Minimum price (inclusive)
    pub min_price: Option<Decimal>,
    /// Maximum price (inclusive)
    pub max_price: Option<Decimal>,
    /// Filter by stock status: true = in stock (`inventory_count` > 0), false = out of stock
    pub in_stock: Option<bool>,
}
