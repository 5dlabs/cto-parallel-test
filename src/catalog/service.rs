use crate::catalog::models::{NewProduct, Product, ProductFilter};
use rust_decimal::Decimal;
use std::error::Error as StdError;
use std::fmt;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

/// Errors that can occur when operating on the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    /// Provided input failed validation.
    InvalidInput(&'static str),
    /// No product exists for the requested id.
    NotFound(i32),
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::NotFound(id) => write!(f, "product not found: {id}"),
        }
    }
}

impl StdError for CatalogError {}

/// Thread-safe in-memory product catalog service.
#[derive(Debug, Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<AtomicI32>,
}

impl Default for ProductService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductService {
    /// Create a new empty service instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(AtomicI32::new(1)),
        }
    }

    /// Create a product with auto-incrementing id.
    ///
    /// # Errors
    /// Returns `CatalogError::InvalidInput` when the provided `NewProduct` fails validation.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    pub fn create(&self, input: NewProduct) -> Result<Product, CatalogError> {
        input.validate().map_err(CatalogError::InvalidInput)?;

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let product = Product {
            id,
            name: input.name,
            price: input.price,
            stock: input.stock,
        };

        let mut guard = self
            .products
            .lock()
            .expect("mutex poisoned: ProductService.products");
        guard.push(product.clone());
        Ok(product)
    }

    /// Return a snapshot of all products.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        self.products
            .lock()
            .expect("mutex poisoned: ProductService.products")
            .clone()
    }

    /// Return product with matching id if present.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        self.products
            .lock()
            .expect("mutex poisoned: ProductService.products")
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }

    /// Update stock to `new_stock` for product id.
    ///
    /// # Errors
    /// Returns `CatalogError::InvalidInput` when `new_stock` is negative.
    /// Returns `CatalogError::NotFound` when the `id` does not exist.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    pub fn update_inventory(&self, id: i32, new_stock: i32) -> Result<Product, CatalogError> {
        if new_stock < 0 {
            return Err(CatalogError::InvalidInput("stock must be non-negative"));
        }
        let mut guard = self
            .products
            .lock()
            .expect("mutex poisoned: ProductService.products");
        let product = guard
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(CatalogError::NotFound(id))?;
        product.stock = new_stock;
        Ok(product.clone())
    }

    /// Filter products using the provided criteria.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn filter(&self, f: &ProductFilter) -> Vec<Product> {
        let lower = f.name_contains.as_ref().map(|s| s.to_lowercase());

        let apply_name = |p: &Product| match &lower {
            Some(sub) => p.name.to_lowercase().contains(sub),
            None => true,
        };

        let apply_min_price = |p: &Product| match f.min_price {
            Some(min) => p.price >= min,
            None => true,
        };

        let apply_max_price = |p: &Product| match f.max_price {
            Some(max) => p.price <= max,
            None => true,
        };

        let apply_stock_flag = |p: &Product| match f.in_stock {
            Some(true) => p.stock > 0,
            Some(false) => p.stock == 0,
            None => true,
        };

        let apply_stock_min = |p: &Product| match f.min_stock {
            Some(min) => p.stock >= min,
            None => true,
        };

        let apply_stock_max = |p: &Product| match f.max_stock {
            Some(max) => p.stock <= max,
            None => true,
        };

        self.products
            .lock()
            .expect("mutex poisoned: ProductService.products")
            .iter()
            .filter(|&p| {
                apply_name(p)
                    && apply_min_price(p)
                    && apply_max_price(p)
                    && apply_stock_flag(p)
                    && apply_stock_min(p)
                    && apply_stock_max(p)
            })
            .cloned()
            .collect()
    }

    /// Delete product by id; returns true if one was removed.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut guard = self
            .products
            .lock()
            .expect("mutex poisoned: ProductService.products");
        let len_before = guard.len();
        guard.retain(|p| p.id != id);
        guard.len() != len_before
    }
}

// Helper to prevent unused imports warnings for Decimal in public API docs.
#[allow(dead_code)]
fn _ensure_decimal(_: Decimal) {}
