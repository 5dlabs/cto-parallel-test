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

    /// Creates a new product with auto-incrementing ID
    ///
    /// # Arguments
    /// * `new_product` - The product data to create
    ///
    /// # Returns
    /// The created product with assigned ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should only happen in case of panic during lock)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Products lock poisoned");
        let mut next_id = self.next_id.lock().expect("Next ID lock poisoned");

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
    /// # Returns
    /// Vector of all products (cloned)
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Products lock poisoned");
        products.clone()
    }

    /// Finds a product by ID
    ///
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Products lock poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Products lock poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on provided criteria
    ///
    /// # Arguments
    /// * `filter` - Filter criteria to apply
    ///
    /// # Returns
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Products lock poisoned");
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
    /// `true` if the product was deleted, `false` if not found
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Products lock poisoned");
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

    fn create_test_product() -> NewProduct {
        NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        }
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = create_test_product();

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(create_test_product());
        let product2 = service.create(create_test_product());
        let product3 = service.create(create_test_product());

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(create_test_product());
        let _ = service.create(create_test_product());

        let products = service.get_all();

        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let created = service.create(create_test_product());

        let found = service.get_by_id(created.id);

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);
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
        let product = service.create(create_test_product());

        let updated = service.update_inventory(product.id, 25);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();

        let result = service.update_inventory(999, 25);

        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 10,
        });

        let filter = ProductFilter::new().with_name("lap");
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
            price: dec!(999.99),
            inventory_count: 5,
        });

        let filter = ProductFilter::new().with_name("LAPTOP");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
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
            price: dec!(100.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_min_price(dec!(20.00))
            .with_max_price(dec!(80.00));
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
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");

        let filter = ProductFilter::new().with_in_stock(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "Professional laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Basic laptop".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Gaming".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(2000.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::new()
            .with_name("laptop")
            .with_min_price(dec!(400.00))
            .with_max_price(dec!(1600.00))
            .with_in_stock(true);

        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Laptop Pro"));
        assert!(results.iter().any(|p| p.name == "Laptop Basic"));
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(create_test_product());
        let _ = service.create(create_test_product());
        let _ = service.create(create_test_product());

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(create_test_product());

        let deleted = service.delete(product.id);

        assert!(deleted);
        assert!(service.get_by_id(product.id).is_none());
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_delete_product_not_found() {
        let service = ProductService::new();

        let deleted = service.delete(999);

        assert!(!deleted);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Test".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(19.999));

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, dec!(19.999));
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Concurrent test".to_string(),
                    price: dec!(10.00),
                    inventory_count: 1,
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

        // Should have 10 products total
        assert_eq!(service.get_all().len(), 10);
    }

    #[test]
    fn test_concurrent_reads_and_writes() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Initial Product {i}"),
                description: "Test".to_string(),
                price: dec!(10.00),
                inventory_count: 5,
            });
        }

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                let _ = service_clone.get_all();
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for i in 0..5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Concurrent Product {i}"),
                    description: "Test".to_string(),
                    price: dec!(20.00),
                    inventory_count: 3,
                });
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have initial 5 + 5 new products = 10 total
        assert_eq!(service.get_all().len(), 10);
    }
}
