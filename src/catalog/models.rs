use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

/// A product in the catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Product {
    /// Auto-incremented unique identifier.
    pub id: i32,
    /// Display name.
    pub name: String,
    /// Detailed description.
    pub description: String,
    /// Price with exact decimal precision.
    pub price: Decimal,
    /// Units available in stock.
    pub inventory_count: i32,
}

/// Input payload for creating a new product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// Default maximums used for safe bounds.
pub const MAX_NAME_LEN: usize = 100;
pub const MAX_STOCK: i32 = 1_000_000;

// Absolute caps to avoid unrealistic values when environment overrides are used.
const MAX_NAME_LEN_ABSOLUTE_CAP: usize = 10_000;
const MAX_STOCK_ABSOLUTE_CAP: i32 = 10_000_000;

static CONFIG_NAME_LEN: OnceLock<usize> = OnceLock::new();
static CONFIG_MAX_STOCK: OnceLock<i32> = OnceLock::new();

/// Effective maximum name length, optionally overridden via the
/// `CATALOG_MAX_NAME_LEN` environment variable. Values are clamped to
/// `1..=MAX_NAME_LEN_ABSOLUTE_CAP` and default to `MAX_NAME_LEN`.
#[must_use]
pub fn configured_max_name_len() -> usize {
    *CONFIG_NAME_LEN.get_or_init(|| {
        let raw = env::var("CATALOG_MAX_NAME_LEN").ok();
        match raw.and_then(|s| s.parse::<usize>().ok()) {
            Some(v) if (1..=MAX_NAME_LEN_ABSOLUTE_CAP).contains(&v) => v,
            _ => MAX_NAME_LEN,
        }
    })
}

/// Effective maximum stock, optionally overridden via the
/// `CATALOG_MAX_STOCK` environment variable. Values are clamped to
/// `0..=MAX_STOCK_ABSOLUTE_CAP` and default to `MAX_STOCK`.
#[must_use]
pub fn configured_max_stock() -> i32 {
    *CONFIG_MAX_STOCK.get_or_init(|| {
        let raw = env::var("CATALOG_MAX_STOCK").ok();
        match raw.and_then(|s| s.parse::<i32>().ok()) {
            Some(v) if (0..=MAX_STOCK_ABSOLUTE_CAP).contains(&v) => v,
            _ => MAX_STOCK,
        }
    })
}

/// Filter criteria to query products.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ProductFilter {
    /// Case-insensitive substring check on the name.
    pub name_contains: Option<String>,
    /// Inclusive minimum price.
    pub min_price: Option<Decimal>,
    /// Inclusive maximum price.
    pub max_price: Option<Decimal>,
    /// If set, true only returns items with `inventory_count` > 0; false returns `inventory_count` == 0.
    pub in_stock: Option<bool>,
}

impl ProductFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
