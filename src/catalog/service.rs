use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product with an auto-incremented ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
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
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
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
    /// # Panics
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products
            .iter()
            .filter(|p| {
                let name_match = filter.name_contains.as_ref().map_or_else(
                    || true,
                    |name| p.name.to_lowercase().contains(&name.to_lowercase()),
                );

                let min_price_match = filter.min_price.map_or_else(|| true, |min| p.price >= min);

                let max_price_match = filter.max_price.map_or_else(|| true, |max| p.price <= max);

                let in_stock_match = filter
                    .in_stock
                    .map_or_else(|| true, |in_stock| (p.inventory_count > 0) == in_stock);

                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
    }

    /// Deletes a product by its ID
    ///
    /// Returns true if the product was deleted, false if not found
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned
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
    fn test_get_all() {
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
        assert_eq!(all_products[0].name, "Product 1");
        assert_eq!(all_products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::from_str("15.50").unwrap(),
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
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 15);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 15);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 15);
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
            description: "Phone".to_string(),
            price: Decimal::from_str("999.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple Watch".to_string(),
            description: "Watch".to_string(),
            price: Decimal::from_str("399.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Phone".to_string(),
            price: Decimal::from_str("899.00").unwrap(),
            inventory_count: 8,
        });

        let filter = ProductFilter {
            name_contains: Some("Apple".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.name.contains("Apple")));
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple iPhone".to_string(),
            description: "Phone".to_string(),
            price: Decimal::from_str("999.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("apple".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple iPhone");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
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
            price: Decimal::from_str("100.00").unwrap(),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(Decimal::from_str("20.00").unwrap()),
            max_price: Some(Decimal::from_str("80.00").unwrap()),
            in_stock: None,
        };

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
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable".to_string(),
            price: Decimal::from_str("30.00").unwrap(),
            inventory_count: 0,
        });

        let in_stock_filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        };

        let in_stock_results = service.filter(&in_stock_filter);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock");

        let out_of_stock_filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(false),
        };

        let out_of_stock_results = service.filter(&out_of_stock_filter);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple iPhone".to_string(),
            description: "Phone".to_string(),
            price: Decimal::from_str("999.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple Watch".to_string(),
            description: "Watch".to_string(),
            price: Decimal::from_str("399.00").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Phone".to_string(),
            price: Decimal::from_str("899.00").unwrap(),
            inventory_count: 8,
        });

        let filter = ProductFilter {
            name_contains: Some("phone".to_string()),
            min_price: Some(Decimal::from_str("500.00").unwrap()),
            max_price: Some(Decimal::from_str("1000.00").unwrap()),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple iPhone");
    }

    #[test]
    fn test_empty_filter_returns_all() {
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
            name: "Test Product".to_string(),
            description: "To be deleted".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 0);
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
            description: "Testing decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        });

        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());

        // Test that serialization/deserialization maintains precision
        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_concurrent_access() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();
        let service_clone3 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Thread 1 Product {i}"),
                    description: "From thread 1".to_string(),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: 5,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Thread 2 Product {i}"),
                    description: "From thread 2".to_string(),
                    price: Decimal::from_str("20.00").unwrap(),
                    inventory_count: 3,
                });
            }
        });

        let handle3 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone3.create(NewProduct {
                    name: format!("Thread 3 Product {i}"),
                    description: "From thread 3".to_string(),
                    price: Decimal::from_str("30.00").unwrap(),
                    inventory_count: 7,
                });
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 30);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: std::collections::HashSet<i32> = ids.iter().copied().collect();
        assert_eq!(unique_ids.len(), 30);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 1..=5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: Decimal::from_str("10.00").unwrap(),
                inventory_count: 10,
            });
        }

        let service_write = service.clone();
        let service_read1 = service.clone();
        let service_read2 = service.clone();

        let write_handle = thread::spawn(move || {
            for i in 6..=10 {
                let _ = service_write.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: 10,
                });
            }
        });

        let read_handle1 = thread::spawn(move || {
            for _ in 0..10 {
                let _ = service_read1.get_all();
            }
        });

        let read_handle2 = thread::spawn(move || {
            for _ in 0..10 {
                let _ = service_read2.get_all();
            }
        });

        write_handle.join().unwrap();
        read_handle1.join().unwrap();
        read_handle2.join().unwrap();

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Testing negative inventory".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        // Allow negative inventory (for tracking backorders, etc.)
        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_service_clone() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Original".to_string(),
            description: "Created in original service".to_string(),
            price: Decimal::from_str("100.00").unwrap(),
            inventory_count: 5,
        });

        // Clone the service
        let cloned_service = service.clone();

        // Both should see the same product
        assert_eq!(service.get_all().len(), 1);
        assert_eq!(cloned_service.get_all().len(), 1);

        // Changes in clone should be visible in original
        let _ = cloned_service.create(NewProduct {
            name: "Cloned".to_string(),
            description: "Created in cloned service".to_string(),
            price: Decimal::from_str("200.00").unwrap(),
            inventory_count: 3,
        });

        assert_eq!(service.get_all().len(), 2);
        assert_eq!(cloned_service.get_all().len(), 2);

        // Verify product retrieval
        let found = cloned_service.get_by_id(product.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Original");
    }
}
