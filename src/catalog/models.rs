use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a product in the catalog.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

/// Data required to create a new product.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewProduct {
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

/// Filter criteria for searching products.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductFilter {
    pub name_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub min_stock: Option<i32>,
    pub max_stock: Option<i32>,
}
