//! Product catalog service with thread-safe operations

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service
///
/// Provides CRUD operations for products with in-memory storage.
/// Uses `Arc<Mutex>` for safe concurrent access across threads.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe storage for products
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe counter for auto-incrementing IDs
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product with an auto-generated ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned (which only happens if a thread
    /// panicked while holding the lock)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");

        let product = Product {
            id: *next_id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        *next_id += 1;
        products.push(product.clone());
        product
    }

    /// Retrieves all products
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        products.iter_mut().find(|p| p.id == id).map(|product| {
            product.inventory_count = new_count;
            product.clone()
        })
    }

    /// Filters products based on specified criteria
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products
            .iter()
            .filter(|p| {
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
    }

    /// Deletes a product by ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        let initial_len = products.len();
        products.retain(|p| p.id != id);
        products.len() < initial_len
    }
}

impl Default for ProductService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let found = service.get_by_id(product.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Product");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 5);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "High-performance laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });

        let filter = ProductFilter::new().with_name_contains("laptop".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "High-performance laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_name_contains("LAPTOP".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: dec!(5.00),
            inventory_count: 100,
        });

        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 50,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_min_price(dec!(20.00))
            .with_max_price(dec!(100.00));

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");

        let filter_out = ProductFilter::new().with_in_stock(false);
        let results_out = service.filter(&filter_out);
        assert_eq!(results_out.len(), 1);
        assert_eq!(results_out[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-performance gaming laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business laptop".to_string(),
            price: dec!(800.00),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Entry-level laptop".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_name_contains("laptop".to_string())
            .with_min_price(dec!(600.00))
            .with_max_price(dec!(2000.00))
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "To be deleted".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Already deleted
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Test decimal precision".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        });

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, dec!(19.999));
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone = service.clone();

        let handle = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Thread 1".to_string(),
                    price: dec!(10.00),
                    inventory_count: 1,
                });
            }
        });

        for i in 0..10 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: "Main thread".to_string(),
                price: dec!(20.00),
                inventory_count: 1,
            });
        }

        handle.join().unwrap();

        let products = service.get_all();
        assert_eq!(products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }

    #[test]
    fn test_service_is_clonable() {
        let service = ProductService::new();
        let cloned = service.clone();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        // Cloned service shares the same data
        assert_eq!(cloned.get_all().len(), 1);
    }
}
