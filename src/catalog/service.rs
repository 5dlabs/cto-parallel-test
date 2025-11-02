use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage.
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
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
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Mutex poisoned");
        let mut next_id = self.next_id.lock().expect("Mutex poisoned");

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
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// Returns the updated product if found, `None` otherwise.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Mutex poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the given criteria.
    ///
    /// All filter criteria are combined with AND logic. Empty filter returns all products.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
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
    /// Returns `true` if the product was deleted, `false` if it was not found.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Mutex poisoned");
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
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 100,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 100);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 20,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let _p2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 20,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: dec!(15.50),
            inventory_count: 5,
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
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: dec!(10.00),
            inventory_count: 100,
        });

        let updated = service.update_inventory(product.id, 50);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 50);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 50);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 50);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _laptop = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A computer".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let _mouse = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "A pointing device".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });

        let filter = ProductFilter {
            name_contains: Some("lap".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _laptop = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A computer".to_string(),
            price: dec!(999.99),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("LAPTOP".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _cheap = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: dec!(5.00),
            inventory_count: 10,
        });

        let _medium = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let _expensive = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(dec!(10.00)),
            max_price: Some(dec!(100.00)),
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _in_stock = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _out_of_stock = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter_in_stock = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        };

        let results_in = service.filter(&filter_in_stock);
        assert_eq!(results_in.len(), 1);
        assert_eq!(results_in[0].name, "In Stock");

        let filter_out_of_stock = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(false),
        };

        let results_out = service.filter(&filter_out_of_stock);
        assert_eq!(results_out.len(), 1);
        assert_eq!(results_out[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _pro = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });

        let _basic = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Budget laptop".to_string(),
            price: dec!(500.00),
            inventory_count: 0,
        });

        let _standard = service.create(NewProduct {
            name: "Laptop Standard".to_string(),
            description: "Mid-range laptop".to_string(),
            price: dec!(900.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(400.00)),
            max_price: Some(dec!(1000.00)),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Standard");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let _p2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 20,
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
            price: dec!(10.00),
            inventory_count: 10,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Second delete should return false
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Price".to_string(),
            description: "Has exact decimal price".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_access() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn multiple threads creating products concurrently
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: dec!(10.00),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn test_concurrent_reads_and_writes() {
        let service = Arc::new(ProductService::new());

        // Create initial products
        for i in 0..5 {
            let _product = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: dec!(10.00),
                inventory_count: 10,
            });
        }

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let products = service_clone.get_all();
                assert!(!products.is_empty());
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for i in 5..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _product = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: dec!(20.00),
                    inventory_count: 20,
                });
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify final state
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_negative_inventory_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        // Should allow negative inventory (for cases like back-orders)
        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }
}
