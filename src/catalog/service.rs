//! Product service with thread-safe in-memory storage
//!
//! This module provides the `ProductService` for managing products with
//! concurrent access support using `Arc<Mutex<_>>`.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
#[derive(Debug, Clone)]
pub struct ProductService {
    /// Thread-safe product storage
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe ID counter for auto-incrementing product IDs
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

    /// Creates a new product and assigns it a unique ID
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product data to create
    ///
    /// # Returns
    ///
    /// The created product with an assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// * `new_count` - The new inventory count (can be negative)
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// # Arguments
    ///
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    ///
    /// A vector of products matching all specified criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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

    /// Deletes a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    ///
    /// `true` if a product was deleted, `false` if no product with that ID exists
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Laptop".to_string(),
            description: "High-performance laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, Decimal::from_str("999.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
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
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 3,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 8,
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
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Desc".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
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
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Desc".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end laptop".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Desktop Computer".to_string(),
            description: "Gaming desktop".to_string(),
            price: Decimal::from_str("2000.00").unwrap(),
            inventory_count: 3,
        });

        let filter = ProductFilter::new().with_name("laptop".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Pro");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 20,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: Decimal::from_str("200.00").unwrap(),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("20.00").unwrap())
            .with_max_price(Decimal::from_str("100.00").unwrap());

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::from_str("30.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("40.00").unwrap(),
            inventory_count: 0,
        });

        let in_stock_filter = ProductFilter::new().with_in_stock(true);
        let in_stock_results = service.filter(&in_stock_filter);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock");

        let out_of_stock_filter = ProductFilter::new().with_in_stock(false);
        let out_of_stock_results = service.filter(&out_of_stock_filter);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Entry-level laptop".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end laptop".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Gaming".to_string(),
            description: "Gaming laptop".to_string(),
            price: Decimal::from_str("1200.00").unwrap(),
            inventory_count: 3,
        });

        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(Decimal::from_str("1000.00").unwrap())
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Gaming");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
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
            name: "To Delete".to_string(),
            description: "Will be deleted".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 5,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());
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

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), 10);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = Arc::new(ProductService::new());

        // Create initial product
        let product = service.create(NewProduct {
            name: "Shared Product".to_string(),
            description: "Accessed concurrently".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 100,
        });

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let product_id = product.id;
            let handle = thread::spawn(move || {
                let found = service_clone.get_by_id(product_id);
                assert!(found.is_some());
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for i in 0..5 {
            let service_clone = Arc::clone(&service);
            let product_id = product.id;
            let handle = thread::spawn(move || {
                let _ = service_clone.update_inventory(product_id, 100 - i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify product still exists and has valid inventory
        let final_product = service.get_by_id(product.id);
        assert!(final_product.is_some());
    }

    #[test]
    fn test_decimal_precision_maintained() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Price precision test".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        });

        assert_eq!(product.price.to_string(), "19.99");

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price.to_string(), "19.99");
    }
}
