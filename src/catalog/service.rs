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
    ///
    /// Panics if the mutex lock is poisoned
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
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Finds a product by its ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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
    ///
    /// Panics if the mutex lock is poisoned
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
    /// All filter criteria are combined with AND logic.
    /// Empty/None criteria are ignored.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products
            .iter()
            .filter(|p| {
                // Name filter (case-insensitive substring match)
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                // Minimum price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Maximum price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // In-stock filter
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
    /// Returns true if a product was deleted, false if not found
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, "A test product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 15,
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
            inventory_count: 15,
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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
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

        let updated = service.update_inventory(product.id, 25);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple iPhone".to_string(),
            description: "Smartphone".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Smartphone".to_string(),
            price: Decimal::from_str("899.99").unwrap(),
            inventory_count: 15,
        });

        let filter = ProductFilter::new().with_name("apple".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple iPhone");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low cost".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium cost".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High cost".to_string(),
            price: Decimal::from_str("200.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("25.00").unwrap())
            .with_max_price(Decimal::from_str("150.00").unwrap());

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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple Watch".to_string(),
            description: "Smartwatch".to_string(),
            price: Decimal::from_str("399.99").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple AirPods".to_string(),
            description: "Earbuds".to_string(),
            price: Decimal::from_str("199.99").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Watch".to_string(),
            description: "Smartwatch".to_string(),
            price: Decimal::from_str("299.99").unwrap(),
            inventory_count: 5,
        });

        let filter = ProductFilter::new()
            .with_name("apple".to_string())
            .with_max_price(Decimal::from_str("400.00").unwrap())
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple Watch");
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Will be removed".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_nonexistent_product() {
        let service = ProductService::new();
        let deleted = service.delete(999);
        assert!(!deleted);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Price".to_string(),
            description: "Test decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        });

        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());

        // Test serialization preserves precision
        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_concurrent_creation() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create 10 products concurrently from different threads
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

        // Wait for all threads to complete
        let mut products = vec![];
        for handle in handles {
            products.push(handle.join().unwrap());
        }

        // Verify all products were created with unique IDs
        assert_eq!(products.len(), 10);
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10); // All IDs should be unique

        // Verify all products are in the service
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
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
                inventory_count: 10,
            });
        }

        let mut handles = vec![];

        // Read products concurrently from multiple threads
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || service_clone.get_all());
            handles.push(handle);
        }

        // Wait for all threads and verify results
        for handle in handles {
            let products = handle.join().unwrap();
            assert_eq!(products.len(), 5);
        }
    }
}
