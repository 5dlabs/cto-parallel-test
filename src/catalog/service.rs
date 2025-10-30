use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Create a new product service with empty catalog
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new product and assign it a unique ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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

    /// Get all products in the catalog
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Get a product by its ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Update the inventory count of a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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

    /// Filter products based on provided criteria
    ///
    /// All filter criteria are optional. If a criterion is None, it doesn't filter.
    /// All criteria are combined with AND logic.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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

                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
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

    #[test]
    fn test_product_creation() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Test Product");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        assert_eq!(product2.id, 2);
        assert_eq!(product2.name, "Product 2");
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);

        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());

        let all = service.get_all();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_inventory_update() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);
    }

    #[test]
    fn test_inventory_update_nonexistent() {
        let service = ProductService::new();
        let updated = service.update_inventory(9999, 5);
        assert!(updated.is_none());
    }

    #[test]
    fn test_product_filtering() {
        let service = ProductService::new();

        // Create test products
        let _apple = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh fruit".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _banana = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow fruit".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _orange = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Citrus fruit".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test name filter (case-insensitive)
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // $1.00
            max_price: Some(Decimal::new(180, 2)), // $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test in_stock filter (true)
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test in_stock filter (false)
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Banana
        assert_eq!(filtered[0].name, "Banana");

        // Test combined filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange contain "a"

        // Test empty filter returns all
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_get_all_empty() {
        let service = ProductService::new();
        let all = service.get_all();
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create products from multiple threads
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _product = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(1000 + i64::from(i), 2),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify all products were created
        let all = service.get_all();
        assert_eq!(all.len(), 10);
    }
}
