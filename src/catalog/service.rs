use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service
#[derive(Debug, Clone)]
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
    /// Panics if the internal mutex is poisoned
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

    /// Returns all products in the catalog
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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

    /// Filters products based on the provided criteria
    ///
    /// All filters are applied with AND logic. Empty filters return all products.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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

    /// Deletes a product by ID
    ///
    /// Returns true if the product was found and deleted, false otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Laptop".to_string(),
            description: "A powerful laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, Decimal::from_str("999.99").unwrap());
        assert_eq!(product.inventory_count, 5);
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
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0].name, "Product 1");
        assert_eq!(all_products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 3,
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
            description: "Test".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 5);

        let not_found = service.update_inventory(999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end laptop".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Budget laptop".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Gaming Mouse".to_string(),
            description: "RGB mouse".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 20,
        });

        let filter = ProductFilter::new().with_name("laptop");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Office Laptop"));
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "LAPTOP".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("100.00").unwrap(),
            inventory_count: 1,
        });

        let filter = ProductFilter::new().with_name("laptop");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
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
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Budget".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Entry level".to_string(),
            price: Decimal::from_str("300.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_name("laptop")
            .with_max_price(Decimal::from_str("600.00").unwrap())
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Budget Laptop");
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 2,
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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Second delete returns false
    }

    #[test]
    fn test_concurrent_creates() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn 10 threads, each creating 10 products
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(NewProduct {
                        name: format!("Product {i}-{j}"),
                        description: "Concurrent creation".to_string(),
                        price: Decimal::from_str("10.00").unwrap(),
                        inventory_count: 1,
                    });
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 100);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        for i in 0..ids.len() - 1 {
            assert_ne!(ids[i], ids[i + 1], "Duplicate ID found: {}", ids[i]);
        }
    }

    #[test]
    fn test_concurrent_reads_and_writes() {
        let service = Arc::new(ProductService::new());

        // Create some initial products
        for i in 0..10 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: "Initial product".to_string(),
                price: Decimal::from_str("10.00").unwrap(),
                inventory_count: 10,
            });
        }

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = service_clone.get_all();
                    let _ = service_clone.get_by_id(1);
                }
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let product = service_clone.create(NewProduct {
                        name: "New Product".to_string(),
                        description: "Created during test".to_string(),
                        price: Decimal::from_str("15.00").unwrap(),
                        inventory_count: 5,
                    });
                    let _ = service_clone.update_inventory(product.id, 3);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify service is still functional
        let all_products = service.get_all();
        assert!(all_products.len() >= 10); // At least the initial products
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Testing decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 1,
        });

        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());

        // Test that precision is maintained after serialization
        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_negative_inventory_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test negative inventory".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        // Should be able to set negative inventory (e.g., for backordered items)
        let updated = service.update_inventory(product.id, -2);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -2);
    }
}
