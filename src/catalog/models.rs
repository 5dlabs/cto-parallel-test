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
    /// Price with exact decimal precision.
    pub price: Decimal,
    /// Units available in stock.
    pub stock: i32,
}

/// Input payload for creating a new product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NewProduct {
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

impl NewProduct {
    /// Basic invariant validation.
    ///
    /// # Errors
    /// Returns an error if name is empty, price is negative, or stock is negative.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.name.trim().is_empty() {
            return Err("name must not be empty");
        }
        if self.price.is_sign_negative() {
            return Err("price must be non-negative");
        }
        if self.stock < 0 {
            return Err("stock must be non-negative");
        }
        Ok(())
    }
}

/// Filter criteria to query products.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductFilter {
    /// Case-insensitive substring check on the name.
    pub name_contains: Option<String>,
    /// Inclusive minimum price.
    pub min_price: Option<Decimal>,
    /// Inclusive maximum price.
    pub max_price: Option<Decimal>,
    /// If set, true only returns items with stock > 0; false returns stock == 0.
    pub in_stock: Option<bool>,
    /// Inclusive minimum stock.
    pub min_stock: Option<i32>,
    /// Inclusive maximum stock.
    pub max_stock: Option<i32>,
}
