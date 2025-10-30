use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog with all its details including ID.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Represents a new product to be created (without ID).
/// The ID will be assigned by the service upon creation.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for querying products.
/// All fields are optional - None means no filter on that criterion.
/// Multiple filters are combined with AND logic.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    /// Filter by product name (case-insensitive substring match).
    pub name_contains: Option<String>,
    /// Filter by minimum price (inclusive).
    pub min_price: Option<Decimal>,
    /// Filter by maximum price (inclusive).
    pub max_price: Option<Decimal>,
    /// Filter by stock availability (true = in stock, false = out of stock).
    pub in_stock: Option<bool>,
}
