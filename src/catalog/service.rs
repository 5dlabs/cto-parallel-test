//! Product catalog service with thread-safe in-memory storage.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service.
///
/// Provides CRUD operations, inventory management, and filtering capabilities
/// with concurrent access support using `Arc<Mutex>`.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe storage for products
    products: Arc<Mutex<Vec<Product>>>,
    /// Auto-incrementing ID counter
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and returns it with an assigned ID.
    ///
    /// # Arguments
    ///
    /// * `new_product` - Product data without ID
    ///
    /// # Returns
    ///
    /// The created product with auto-generated ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
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

    /// Returns all products in the catalog.
    ///
    /// # Returns
    ///
    /// A vector of all products (cloned)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID.
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
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID
    /// * `new_count` - The new inventory count (can be negative for tracking backorders)
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated count if found, `None` if product doesn't exist
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
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

    /// Filters products based on the provided criteria.
    ///
    /// All specified filters are applied with AND logic.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter criteria (empty filter returns all products)
    ///
    /// # Returns
    ///
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
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

    /// Deletes a product by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    ///
    /// `true` if the product was deleted, `false` if it wasn't found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
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
    use rust_decimal_macros::dec;
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Laptop".to_string(),
            description: "High-performance laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, dec!(999.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
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
            description: "Desc 1".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test Desc".to_string(),
            price: dec!(15.50),
            inventory_count: 7,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Product");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Desc".to_string(),
            price: dec!(25.00),
            inventory_count: 100,
        });

        let updated = service.update_inventory(product.id, 50);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 50);

        let not_found = service.update_inventory(999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "Desc".to_string(),
            price: dec!(1200.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Office Desktop".to_string(),
            description: "Desc".to_string(),
            price: dec!(800.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()), // Case-insensitive
            ..Default::default()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Desc".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Desc".to_string(),
            price: dec!(50.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "Desc".to_string(),
            price: dec!(100.00),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            min_price: Some(dec!(20.00)),
            max_price: Some(dec!(80.00)),
            ..Default::default()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Desc".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Desc".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "Desc".to_string(),
            price: dec!(1200.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Desc".to_string(),
            price: dec!(400.00),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Desc".to_string(),
            price: dec!(800.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(500.00)),
            max_price: Some(dec!(1000.00)),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Office Laptop");
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Desc".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let not_found = service.get_by_id(product.id);
        assert!(not_found.is_none());

        let deleted_again = service.delete(product.id);
        assert!(!deleted_again);
    }

    #[test]
    fn test_concurrent_creation() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: dec!(10.00),
                    inventory_count: 5,
                })
            });
            handles.push(handle);
        }

        let mut products = vec![];
        for handle in handles {
            products.push(handle.join().unwrap());
        }

        // All products should have unique IDs
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);

        // Service should have all 10 products
        assert_eq!(service.get_all().len(), 10);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = Arc::new(ProductService::new());

        // Create initial products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: dec!(10.00),
                inventory_count: 100,
            });
        }

        let mut handles = vec![];

        // Spawn readers
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let products = service_clone.get_all();
                assert!(!products.is_empty());
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
                    price: dec!(20.00),
                    inventory_count: 50,
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
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Price".to_string(),
            description: "Desc".to_string(),
            price: dec!(19.99),
            inventory_count: 5,
        });

        assert_eq!(product.price, dec!(19.99));

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, dec!(19.99));
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Desc".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_negative_inventory_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Backorder Product".to_string(),
            description: "Desc".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }
}
