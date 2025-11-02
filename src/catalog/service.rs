//! Product catalog service implementation
//!
//! This module provides thread-safe product management functionality with in-memory storage.
//! All operations are safe for concurrent access using Arc<Mutex> for synchronization.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service
///
/// Manages products in memory with auto-incrementing IDs and concurrent access safety.
/// All methods use internal locking to ensure thread safety.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe storage for products
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe counter for auto-incrementing IDs
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    ///
    /// let service = ProductService::new();
    /// assert_eq!(service.get_all().len(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and adds it to the catalog
    ///
    /// The product will be assigned an auto-incrementing ID starting from 1.
    /// This operation is thread-safe.
    ///
    /// # Arguments
    ///
    /// * `new_product` - Product data without an ID
    ///
    /// # Returns
    ///
    /// The created product with its assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// };
    ///
    /// let product = service.create(new_product);
    /// assert_eq!(product.id, 1);
    /// assert_eq!(product.name, "Laptop");
    /// ```
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

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

    /// Returns all products in the catalog
    ///
    /// This operation clones the product list to avoid holding the lock.
    ///
    /// # Returns
    ///
    /// A vector containing all products
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// };
    ///
    /// service.create(new_product);
    /// let products = service.get_all();
    /// assert_eq!(products.len(), 1);
    /// ```
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    ///
    /// `Some(Product)` if found, `None` if not found
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// };
    ///
    /// let created = service.create(new_product);
    /// let found = service.get_by_id(created.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().name, "Laptop");
    ///
    /// let not_found = service.get_by_id(999);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count (can be negative for accounting purposes)
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` if product doesn't exist
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// };
    ///
    /// let created = service.create(new_product);
    /// let updated = service.update_inventory(created.id, 10);
    /// assert!(updated.is_some());
    /// assert_eq!(updated.unwrap().inventory_count, 10);
    ///
    /// let not_found = service.update_inventory(999, 10);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().unwrap();
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// Multiple filter criteria are combined with AND logic.
    /// An empty filter returns all products.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter criteria for searching products
    ///
    /// # Returns
    ///
    /// A vector of products matching all filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct, ProductFilter};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    ///
    /// // Create test products
    /// service.create(NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// });
    ///
    /// service.create(NewProduct {
    ///     name: "Mouse".to_string(),
    ///     description: "Wireless mouse".to_string(),
    ///     price: Decimal::from_str("29.99").unwrap(),
    ///     inventory_count: 0,
    /// });
    ///
    /// // Filter by name
    /// let filter = ProductFilter::new().with_name_contains("lap".to_string());
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Laptop");
    ///
    /// // Filter by price range
    /// let filter = ProductFilter::new()
    ///     .with_min_price(Decimal::from_str("20.00").unwrap())
    ///     .with_max_price(Decimal::from_str("100.00").unwrap());
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Mouse");
    ///
    /// // Filter by stock status
    /// let filter = ProductFilter::new().with_in_stock(true);
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Laptop");
    /// ```
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();
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

    /// Deletes a product from the catalog
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    ///
    /// `true` if the product was deleted, `false` if it didn't exist
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned, which only occurs if a thread
    /// panicked while holding the lock. This is extremely rare in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "Gaming laptop".to_string(),
    ///     price: Decimal::from_str("999.99").unwrap(),
    ///     inventory_count: 5,
    /// };
    ///
    /// let created = service.create(new_product);
    /// assert_eq!(service.get_all().len(), 1);
    ///
    /// let deleted = service.delete(created.id);
    /// assert!(deleted);
    /// assert_eq!(service.get_all().len(), 0);
    ///
    /// let not_deleted = service.delete(999);
    /// assert!(!not_deleted);
    /// ```
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().unwrap();
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
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new_service() {
        let service = ProductService::new();
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, Decimal::from_str("999.99").unwrap());
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Laptop");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory_success() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let updated = service.update_inventory(created.id, 10);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 10);

        let retrieved = service.get_by_id(created.id);
        assert_eq!(retrieved.unwrap().inventory_count, 10);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let updated = service.update_inventory(created.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_name_contains("lap".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let filter = ProductFilter::new().with_name_contains("LAPTOP".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_min_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_min_price(Decimal::from_str("100.00").unwrap());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_max_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_max_price(Decimal::from_str("100.00").unwrap());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mouse");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Cheap product".to_string(),
            price: Decimal::from_str("5.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Medium product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "Expensive product".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("10.00").unwrap())
            .with_max_price(Decimal::from_str("100.00").unwrap());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }

    #[test]
    fn test_filter_by_out_of_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
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
            description: "High-end gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business laptop".to_string(),
            price: Decimal::from_str("599.99").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Gaming Mouse".to_string(),
            description: "High DPI mouse".to_string(),
            price: Decimal::from_str("79.99").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_name_contains("laptop".to_string())
            .with_min_price(Decimal::from_str("500.00").unwrap())
            .with_in_stock(true);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_empty() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_success() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        assert_eq!(service.get_all().len(), 1);

        let deleted = service.delete(created.id);
        assert!(deleted);
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_delete_not_found() {
        let service = ProductService::new();
        let deleted = service.delete(999);
        assert!(!deleted);
    }

    #[test]
    fn test_concurrent_create() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create 10 products concurrently
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let products = service.get_all();
        assert_eq!(products.len(), 10);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = Arc::new(ProductService::new());

        // Pre-populate with some products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: Decimal::from_str("10.00").unwrap(),
                inventory_count: i,
            });
        }

        let mut handles = vec![];

        // Spawn readers
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let products = service_clone.get_all();
                assert!(products.len() >= 5);
            });
            handles.push(handle);
        }

        // Spawn writers
        for i in 5..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(service.get_all().len(), 10);
    }

    #[test]
    fn test_decimal_precision_maintained() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Test decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 1,
        });

        assert_eq!(created.price.to_string(), "19.99");

        let retrieved = service.get_by_id(created.id).unwrap();
        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_service_clone() {
        let service1 = ProductService::new();
        let _ = service1.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let service2 = service1.clone();
        let products = service2.get_all();
        assert_eq!(products.len(), 1);

        // Both services should share the same underlying data
        let _ = service2.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
        });

        assert_eq!(service1.get_all().len(), 2);
        assert_eq!(service2.get_all().len(), 2);
    }
}
