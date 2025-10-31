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
    /// # Arguments
    /// * `new_product` - Product data without ID
    ///
    /// # Returns
    /// The created product with assigned ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
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
    /// # Returns
    /// A vector containing clones of all products
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
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
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
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count (can be negative for backorders)
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
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
    /// All filter criteria use AND logic - a product must match all specified filters.
    /// Empty/None criteria are ignored.
    ///
    /// # Arguments
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    /// A vector of products matching all filter criteria
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
    /// # Arguments
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    /// `true` if a product was deleted, `false` if not found
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

    /// Returns the number of products in the catalog
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn count(&self) -> usize {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.len()
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
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, dec!(999.99));
        assert_eq!(product.inventory_count, 5);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
        });

        let product3 = service.create(NewProduct {
            name: "Product 3".to_string(),
            description: "Third product".to_string(),
            price: dec!(30.00),
            inventory_count: 3,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Product");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let found = service.get_by_id(999);
        assert!(found.is_none());
    }

    #[test]
    fn test_update_inventory_success() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Test".to_string(),
            price: dec!(100.00),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 25);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        // Verify the change persists
        let fetched = service.get_by_id(product.id).unwrap();
        assert_eq!(fetched.inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let updated = service.update_inventory(999, 10);
        assert!(updated.is_none());
    }

    #[test]
    fn test_update_inventory_negative_count() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Test".to_string(),
            price: dec!(50.00),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "Professional laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Desktop PC".to_string(),
            description: "Desktop computer".to_string(),
            price: dec!(2000.00),
            inventory_count: 2,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Basic laptop".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_name_contains("laptop".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.name.to_lowercase().contains("laptop")));
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "LAPTOP".to_string(),
            description: "Upper case".to_string(),
            price: dec!(1000.00),
            inventory_count: 1,
        });

        let filter = ProductFilter::new().with_name_contains("laptop".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: dec!(150.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_min_price(dec!(100.00))
            .with_max_price(dec!(300.00));
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_stock_status_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(100.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(100.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }

    #[test]
    fn test_filter_by_stock_status_out_of_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(100.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(100.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business use".to_string(),
            price: dec!(800.00),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Entry level".to_string(),
            price: dec!(400.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Gaming Desktop".to_string(),
            description: "Desktop PC".to_string(),
            price: dec!(1200.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_name_contains("laptop".to_string())
            .with_min_price(dec!(500.00))
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
            price: dec!(100.00),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(200.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product_success() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Will be deleted".to_string(),
            price: dec!(50.00),
            inventory_count: 5,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_product_not_found() {
        let service = ProductService::new();
        let deleted = service.delete(999);
        assert!(!deleted);
    }

    #[test]
    fn test_count() {
        let service = ProductService::new();
        assert_eq!(service.count(), 0);

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(100.00),
            inventory_count: 1,
        });

        assert_eq!(service.count(), 1);

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(200.00),
            inventory_count: 2,
        });

        assert_eq!(service.count(), 2);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise Price".to_string(),
            description: "Test decimal precision".to_string(),
            price: dec!(19.99),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(19.99));

        let fetched = service.get_by_id(product.id).unwrap();
        assert_eq!(fetched.price, dec!(19.99));
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();
        let service_clone3 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Thread1-Product{i}"),
                    description: "From thread 1".to_string(),
                    price: dec!(100.00),
                    inventory_count: 1,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Thread2-Product{i}"),
                    description: "From thread 2".to_string(),
                    price: dec!(200.00),
                    inventory_count: 2,
                });
            }
        });

        let handle3 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone3.create(NewProduct {
                    name: format!("Thread3-Product{i}"),
                    description: "From thread 3".to_string(),
                    price: dec!(300.00),
                    inventory_count: 3,
                });
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");
        handle3.join().expect("Thread 3 panicked");

        // Should have 30 products total
        assert_eq!(service.count(), 30);

        // All IDs should be unique
        let products = service.get_all();
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: std::collections::HashSet<i32> = ids.iter().copied().collect();
        assert_eq!(unique_ids.len(), 30);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product{i}"),
                description: "Initial product".to_string(),
                price: dec!(100.00),
                inventory_count: 10,
            });
        }

        let service_read = service.clone();
        let service_write = service.clone();

        let read_handle = thread::spawn(move || {
            for _ in 0..100 {
                let _products = service_read.get_all();
                thread::sleep(std::time::Duration::from_micros(1));
            }
        });

        let write_handle = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_write.create(NewProduct {
                    name: format!("NewProduct{i}"),
                    description: "Added concurrently".to_string(),
                    price: dec!(200.00),
                    inventory_count: 5,
                });
                thread::sleep(std::time::Duration::from_micros(10));
            }
        });

        read_handle.join().expect("Read thread panicked");
        write_handle.join().expect("Write thread panicked");

        // Should have original 5 + 10 new = 15 total
        assert_eq!(service.count(), 15);
    }
}
