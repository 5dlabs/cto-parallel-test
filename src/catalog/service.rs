use crate::catalog::models::{
    configured_max_name_len, configured_max_stock, NewProduct, Product, ProductFilter,
};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service.
#[derive(Debug, Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
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
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    fn lock_products(&self) -> Vec<Product> {
        self.products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn lock_products_mut(&self) -> std::sync::MutexGuard<'_, Vec<Product>> {
        self.products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn next_id(&self) -> i32 {
        let mut guard = self
            .next_id
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = *guard;
        *guard += 1;
        current
    }

    /// Create a product with auto-incrementing id.
    #[must_use]
    pub fn create(&self, input: NewProduct) -> Product {
        let id = self.next_id();
        // Sanitize inputs defensively
        let name: String = input.name.chars().take(configured_max_name_len()).collect();
        let inventory_count = input.inventory_count.clamp(0, configured_max_stock());
        let price = if input.price.is_sign_negative() {
            // Avoid negative prices; default to 0
            rust_decimal::Decimal::new(0, 0)
        } else {
            input.price
        };
        let product = Product {
            id,
            name,
            description: input.description,
            price,
            inventory_count,
        };

        let mut guard = self.lock_products_mut();
        guard.push(product.clone());
        product
    }

    /// Return a snapshot of all products.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        self.lock_products()
    }

    /// Return product with matching id if present.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        self.products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .find(|product| product.id == id)
            .cloned()
    }

    /// Update inventory count for product id.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        // Allow negative counts (backorders) but clamp to an upper bound
        // to avoid unrealistic values.
        let new_count = new_count.min(configured_max_stock());
        let mut guard = self.lock_products_mut();
        guard
            .iter_mut()
            .find(|product| product.id == id)
            .map(|product| {
                product.inventory_count = new_count;
                product.clone()
            })
    }

    /// Filter products using the provided criteria.
    #[must_use]
    pub fn filter(&self, filter: ProductFilter) -> Vec<Product> {
        let ProductFilter {
            name_contains,
            min_price,
            max_price,
            in_stock,
        } = filter;

        // Bound the substring length to avoid unbounded allocations from
        // untrusted input. Use the configured name length cap.
        let name_contains = name_contains.map(|name| {
            name.chars()
                .take(configured_max_name_len())
                .collect::<String>()
                .to_lowercase()
        });

        self.products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .filter(|product| {
                if let Some(ref name) = name_contains {
                    if !product.name.to_lowercase().contains(name) {
                        return false;
                    }
                }

                if let Some(ref min) = min_price {
                    if product.price < *min {
                        return false;
                    }
                }

                if let Some(ref max) = max_price {
                    if product.price > *max {
                        return false;
                    }
                }

                if let Some(flag) = in_stock {
                    let available = product.inventory_count > 0;
                    if flag != available {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Delete product by id; returns true if one was removed.
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut guard = self.lock_products_mut();
        let len_before = guard.len();
        guard.retain(|product| product.id != id);
        guard.len() != len_before
    }
}
