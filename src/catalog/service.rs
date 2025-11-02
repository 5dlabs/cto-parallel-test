//! Product service with thread-safe in-memory storage.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage.
///
/// Provides CRUD operations, inventory management, and filtering
/// with automatic ID generation and concurrent access support.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe product storage
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe ID counter
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service.
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

    /// Creates a new product with an auto-generated ID.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned (extremely rare, only happens if another
    /// thread panicked while holding the lock).
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "A powerful laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let product = service.create(new_product);
    /// assert_eq!(product.id, 1);
    /// assert_eq!(product.name, "Laptop");
    /// ```
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Failed to lock products");
        let mut next_id = self.next_id.lock().expect("Failed to lock next_id");

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
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// service.create(NewProduct {
    ///     name: "Product 1".to_string(),
    ///     description: "First product".to_string(),
    ///     price: dec!(10.00),
    ///     inventory_count: 5,
    /// });
    ///
    /// let products = service.get_all();
    /// assert_eq!(products.len(), 1);
    /// ```
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// });
    ///
    /// let found = service.get_by_id(product.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().name, "Laptop");
    ///
    /// let not_found = service.get_by_id(999);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// Returns the updated product if found, or `None` if the product doesn't exist.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// });
    ///
    /// let updated = service.update_inventory(product.id, 5);
    /// assert!(updated.is_some());
    /// assert_eq!(updated.unwrap().inventory_count, 5);
    ///
    /// let not_found = service.update_inventory(999, 0);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Failed to lock products");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria.
    ///
    /// All filter criteria are combined with AND logic. An empty filter returns all products.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductFilter, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// service.create(NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// });
    /// service.create(NewProduct {
    ///     name: "Mouse".to_string(),
    ///     description: "A mouse".to_string(),
    ///     price: dec!(29.99),
    ///     inventory_count: 0,
    /// });
    ///
    /// // Filter by name
    /// let filter = ProductFilter::with_name("lap");
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Laptop");
    ///
    /// // Filter by stock status
    /// let filter = ProductFilter::with_stock_status(true);
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Laptop");
    /// ```
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
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

    /// Deletes a product by its ID.
    ///
    /// Returns `true` if the product was deleted, `false` if it wasn't found.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{NewProduct, ProductService};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "A laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// });
    ///
    /// assert!(service.delete(product.id));
    /// assert!(!service.delete(product.id)); // Already deleted
    /// assert_eq!(service.get_all().len(), 0);
    /// ```
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Failed to lock products");
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
            description: "A powerful laptop".to_string(),
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
    fn test_get_all() {
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

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let found = service.get_by_id(product.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Laptop");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let found = service.get_by_id(product.id);
        assert_eq!(found.unwrap().inventory_count, 5);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });
        let _ = service.create(NewProduct {
            name: "Office Mouse".to_string(),
            description: "Ergonomic mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 20,
        });
        let _ = service.create(NewProduct {
            name: "Laptop Stand".to_string(),
            description: "Adjustable stand".to_string(),
            price: dec!(49.99),
            inventory_count: 15,
        });

        let filter = ProductFilter::with_name("laptop");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Laptop Stand"));
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });

        let filter = ProductFilter::with_name("LAPTOP");
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
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter::with_price_range(Some(dec!(20.00)), Some(dec!(100.00)));
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_min_price_only() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low".to_string(),
            price: dec!(5.00),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter::with_price_range(Some(dec!(100.00)), None);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Expensive");
    }

    #[test]
    fn test_filter_by_stock_status_in_stock() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(50.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::with_stock_status(true);
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
            price: dec!(50.00),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(50.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::with_stock_status(false);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });
        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Budget".to_string(),
            price: dec!(500.00),
            inventory_count: 0,
        });
        let _ = service.create(NewProduct {
            name: "Pro Laptop".to_string(),
            description: "Premium".to_string(),
            price: dec!(2000.00),
            inventory_count: 3,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(1000.00)),
            max_price: Some(dec!(1800.00)),
            in_stock: Some(true),
        };

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
            name: "Laptop".to_string(),
            description: "A laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        assert!(service.delete(product.id));
        assert_eq!(service.get_all().len(), 0);
        assert!(service.get_by_id(product.id).is_none());
    }

    #[test]
    fn test_delete_not_found() {
        let service = ProductService::new();
        assert!(!service.delete(999));
    }

    #[test]
    fn test_concurrent_create() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();
        let service_clone3 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Concurrent test".to_string(),
                    price: dec!(10.00),
                    inventory_count: 1,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 10..20 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Concurrent test".to_string(),
                    price: dec!(20.00),
                    inventory_count: 2,
                });
            }
        });

        let handle3 = thread::spawn(move || {
            for i in 20..30 {
                let _ = service_clone3.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Concurrent test".to_string(),
                    price: dec!(30.00),
                    inventory_count: 3,
                });
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");
        handle3.join().expect("Thread 3 panicked");

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 30);

        // Verify all IDs are unique and sequential
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(*id, i32::try_from(i + 1).expect("ID overflow"));
        }
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Create initial products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: "Initial".to_string(),
                price: dec!(10.00),
                inventory_count: 10,
            });
        }

        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let writer = thread::spawn(move || {
            for i in 5..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Added".to_string(),
                    price: dec!(20.00),
                    inventory_count: 5,
                });
            }
        });

        let reader = thread::spawn(move || {
            let mut count = 0;
            for _ in 0..20 {
                count = service_clone2.get_all().len();
                thread::sleep(std::time::Duration::from_micros(100));
            }
            count
        });

        writer.join().expect("Writer thread panicked");
        let final_count = reader.join().expect("Reader thread panicked");

        assert!((5..=10).contains(&final_count));
        assert_eq!(service.get_all().len(), 10);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Testing decimal precision".to_string(),
            price: dec!(123.456789),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(123.456789));

        let retrieved = service.get_by_id(product.id).expect("Product not found");
        assert_eq!(retrieved.price, dec!(123.456789));
    }

    #[test]
    fn test_service_clone() {
        let service1 = ProductService::new();
        let _ = service1.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let service2 = service1.clone();
        let _ = service2.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        // Both services should see both products (same underlying storage)
        assert_eq!(service1.get_all().len(), 2);
        assert_eq!(service2.get_all().len(), 2);
    }
}
