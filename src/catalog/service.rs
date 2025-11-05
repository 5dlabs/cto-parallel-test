use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
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

    /// Creates a new product and returns it with an assigned ID
    ///
    /// # Panics
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        let mut next_id = self.next_id.lock().expect("Failed to acquire next_id lock");

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
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        products.clone()
    }

    /// Finds a product by its ID
    ///
    /// # Returns
    /// - `Some(product)` if found
    /// - `None` if not found
    ///
    /// # Panics
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Returns
    /// - `Some(product)` with updated inventory if the product was found
    /// - `None` if the product doesn't exist
    ///
    /// # Panics
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// All filter criteria use AND logic. Empty filter returns all products.
    ///
    /// # Panics
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
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
    /// # Returns
    /// - `true` if the product was found and deleted
    /// - `false` if the product doesn't exist
    ///
    /// # Panics
    /// May panic if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
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
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
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
            price: Decimal::from_str("15.99").unwrap(),
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
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
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
            name: "Red Widget".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Blue Widget".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 3,
        });

        let filter = ProductFilter::new().with_name("red");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Red Widget");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("5.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("10.00").unwrap())
            .with_max_price(Decimal::from_str("20.00").unwrap());

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "In Stock Item".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock Item".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_stock_status(true);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock Item");

        let filter = ProductFilter::new().with_stock_status(false);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock Item");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Red Widget".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Red Gadget".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Blue Widget".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("12.00").unwrap(),
            inventory_count: 3,
        });

        let filter = ProductFilter::new()
            .with_name("widget")
            .with_max_price(Decimal::from_str("15.00").unwrap())
            .with_stock_status(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test".to_string(),
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
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_delete_nonexistent_product() {
        let service = ProductService::new();
        assert!(!service.delete(999));
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 1,
        });

        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, Decimal::from_str("19.99").unwrap());
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
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        let products = service.get_all();
        assert_eq!(products.len(), 10);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), 10);
    }

    #[test]
    fn test_concurrent_reads() {
        let service = Arc::new(ProductService::new());

        // Create some products first
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: Decimal::from_str("10.00").unwrap(),
                inventory_count: i,
            });
        }

        let mut handles = vec![];

        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let products = service_clone.get_all();
                assert_eq!(products.len(), 5);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        // System should allow negative inventory (e.g., for back-orders)
        let updated = service.update_inventory(product.id, -3);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -3);
    }
}
