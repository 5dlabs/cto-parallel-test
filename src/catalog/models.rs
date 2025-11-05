use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Full product entity with all fields including ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Product creation DTO without ID (auto-generated)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Filter criteria for product search
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    /// Create a new empty filter (no filtering applied)
    #[must_use]
    pub fn new() -> Self {
        ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        }
    }
}

impl Default for ProductFilter {
    fn default() -> Self {
        Self::new()
    }
}
