use std::sync::{Arc, Mutex};

use super::models::{NewProduct, Product, ProductFilter};

/// Thread-safe in-memory product service.
#[derive(Clone, Debug)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<std::sync::atomic::AtomicI32>,
}

impl Default for ProductService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductService {
    /// Create a new, empty product service.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(std::sync::atomic::AtomicI32::new(1)),
        }
    }

    /// Create a product and return the created record.
    ///
    /// Auto-increments the ID in a thread-safe way.
    #[must_use]
    pub fn create(&self, new_product: &NewProduct) -> Product {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Basic input hygiene: trim name and clamp stock to non-negative.
        let name = new_product.name.trim().to_string();
        let stock = if new_product.stock < 0 {
            0
        } else {
            new_product.stock
        };

        let product = Product {
            id,
            name,
            price: new_product.price,
            stock,
        };

        let mut guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.push(product.clone());
        product
    }

    /// Return all products.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.clone()
    }

    /// Get a product by its ID.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.iter().find(|p| p.id == id).cloned()
    }

    /// Update the stock count for a product, returning `true` on success.
    ///
    /// Stock can be set to any non-negative value. Negative values are rejected.
    pub fn update_inventory(&self, id: i32, new_stock: i32) -> bool {
        if new_stock < 0 {
            return false;
        }

        let mut guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(p) = guard.iter_mut().find(|p| p.id == id) {
            p.stock = new_stock;
            true
        } else {
            false
        }
    }

    /// Filter products by the provided criteria.
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        guard
            .iter()
            .filter(|p| Self::matches(p, filter))
            .cloned()
            .collect()
    }

    /// Delete a product by ID, returning `true` if a product was removed.
    pub fn delete(&self, id: i32) -> bool {
        let mut guard = self
            .products
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if let Some(pos) = guard.iter().position(|p| p.id == id) {
            guard.remove(pos);
            true
        } else {
            false
        }
    }

    fn matches(p: &Product, f: &ProductFilter) -> bool {
        if let Some(name_sub) = &f.name_contains {
            let needle = name_sub.to_lowercase();
            let hay = p.name.to_lowercase();
            if !hay.contains(&needle) {
                return false;
            }
        }
        if let Some(min) = f.min_price {
            if p.price < min {
                return false;
            }
        }
        if let Some(max) = f.max_price {
            if p.price > max {
                return false;
            }
        }
        if let Some(min) = f.min_stock {
            if p.stock < min {
                return false;
            }
        }
        if let Some(max) = f.max_stock {
            if p.stock > max {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromStr;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use std::thread;

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).expect("valid decimal")
    }

    #[test]
    fn create_and_get() {
        let svc = ProductService::new();
        let p1 = svc.create(&NewProduct {
            name: "Widget".to_string(),
            price: dec("19.99"),
            stock: 10,
        });
        assert_eq!(p1.id, 1);
        assert_eq!(p1.price, dec("19.99"));

        let p2 = svc.create(&NewProduct {
            name: "Gadget".to_string(),
            price: dec("5.00"),
            stock: 0,
        });
        assert_eq!(p2.id, 2);

        let all = svc.get_all();
        assert_eq!(all.len(), 2);

        let got = svc.get_by_id(1).unwrap();
        assert_eq!(got.name, "Widget");
    }

    #[test]
    fn update_and_delete() {
        let svc = ProductService::new();
        let p = svc.create(&NewProduct {
            name: "Thing".to_string(),
            price: dec("1.23"),
            stock: 2,
        });

        assert!(svc.update_inventory(p.id, 5));
        assert_eq!(svc.get_by_id(p.id).unwrap().stock, 5);

        // negative not allowed
        assert!(!svc.update_inventory(p.id, -1));

        assert!(svc.delete(p.id));
        assert!(svc.get_by_id(p.id).is_none());
        assert!(!svc.delete(p.id));
    }

    #[test]
    fn filtering_works() {
        let svc = ProductService::new();
        let _ = svc.create(&NewProduct {
            name: "Apple iPhone".to_string(),
            price: dec("999.99"),
            stock: 3,
        });
        let _ = svc.create(&NewProduct {
            name: "Apple Watch".to_string(),
            price: dec("399.00"),
            stock: 10,
        });
        let _ = svc.create(&NewProduct {
            name: "Android Phone".to_string(),
            price: dec("299.50"),
            stock: 0,
        });

        // name filter
        let named = svc.filter(&ProductFilter {
            name_contains: Some("apple".to_string()),
            ..ProductFilter::default()
        });
        assert_eq!(named.len(), 2);

        // price range
        let mid_price = svc.filter(&ProductFilter {
            min_price: Some(dec("300")),
            max_price: Some(dec("600")),
            ..ProductFilter::default()
        });
        assert_eq!(mid_price.len(), 1);
        assert_eq!(mid_price[0].name, "Apple Watch");

        // stock filter
        let in_stock = svc.filter(&ProductFilter {
            min_stock: Some(1),
            ..ProductFilter::default()
        });
        assert_eq!(in_stock.len(), 2);
    }

    #[test]
    fn concurrency_and_id_uniqueness() {
        let svc = Arc::new(ProductService::new());
        let threads: usize = 8;
        let per_thread: usize = 25;
        let mut handles = Vec::with_capacity(threads);
        for t in 0..threads {
            let svc_cloned = Arc::clone(&svc);
            handles.push(thread::spawn(move || {
                for i in 0..per_thread {
                    let _ = svc_cloned.create(&NewProduct {
                        name: format!("Item-{t}-{i}"),
                        price: dec("10.00"),
                        stock: 1,
                    });
                }
            }));
        }
        for h in handles {
            h.join().expect("thread join");
        }

        let all = svc.get_all();
        assert_eq!(all.len(), threads * per_thread);

        // IDs should be unique and start at 1
        let mut ids: Vec<i32> = all.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), all.len());
        assert_eq!(ids[0], 1);
    }
}
