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

    /// Creates a new product and assigns it an auto-incrementing ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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
    /// Returns `Some(Product)` if found, `None` otherwise
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
    /// Returns the updated product if found, `None` if the product doesn't exist
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
    /// All filter criteria are combined with AND logic.
    /// Empty/None criteria are ignored (match all).
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
                // Name filter: case-insensitive substring matching
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                // Minimum price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Maximum price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // Stock status filter
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // All filters must match (AND logic)
                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
    }

    /// Deletes a product by ID
    ///
    /// Returns `true` if the product was deleted, `false` if it wasn't found
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
    use rust_decimal_macros::dec;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, "A test product");
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_increment_id() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
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

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0].name, "Product 1");
        assert_eq!(all_products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let found = service.get_by_id(product.id);
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
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 25);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        // Verify persistence
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
            price: dec!(999.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Smartphone".to_string(),
            price: dec!(899.99),
            inventory_count: 3,
        });

        let filter = ProductFilter {
            name_contains: Some("apple".to_string()), // Case insensitive
            ..ProductFilter::new()
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
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(100.00),
            inventory_count: 2,
        });

        let filter = ProductFilter {
            min_price: Some(dec!(25.00)),
            max_price: Some(dec!(75.00)),
            ..ProductFilter::new()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_stock_status_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available Product".to_string(),
            description: "In stock".to_string(),
            price: dec!(19.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock Product".to_string(),
            description: "Not available".to_string(),
            price: dec!(29.99),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            in_stock: Some(true),
            ..ProductFilter::new()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Available Product");
    }

    #[test]
    fn test_filter_by_stock_status_out_of_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available Product".to_string(),
            description: "In stock".to_string(),
            price: dec!(19.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock Product".to_string(),
            description: "Not available".to_string(),
            price: dec!(29.99),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            in_stock: Some(false),
            ..ProductFilter::new()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock Product");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple Watch".to_string(),
            description: "Smartwatch".to_string(),
            price: dec!(399.99),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple iPhone".to_string(),
            description: "Smartphone".to_string(),
            price: dec!(999.99),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Smartphone".to_string(),
            price: dec!(299.99),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            name_contains: Some("apple".to_string()),
            min_price: Some(dec!(300.00)),
            max_price: Some(dec!(500.00)),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple Watch");
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
            name: "Test Product".to_string(),
            description: "To be deleted".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
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
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Testing decimal precision".to_string(),
            price: dec!(19.99),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let service = ProductService::new();

        // Create multiple threads that add products concurrently
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let service_clone = service.clone();
                thread::spawn(move || {
                    service_clone.create(NewProduct {
                        name: format!("Product {i}"),
                        description: format!("Description {i}"),
                        price: dec!(10.00) * rust_decimal::Decimal::from(i + 1),
                        inventory_count: i,
                    })
                })
            })
            .collect();

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
    fn test_negative_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Testing negative inventory".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        // Should allow negative inventory (representing backorders, etc.)
        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }
}
