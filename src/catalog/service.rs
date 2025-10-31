use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service
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
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
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

    /// Returns all products in the catalog
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.clone()
    }

    /// Returns a specific product by ID, or None if not found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product and returns the updated product
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Mutex poisoned");
        products.iter_mut().find(|p| p.id == id).map(|product| {
            product.inventory_count = new_count;
            product.clone()
        })
    }

    /// Filters products based on multiple optional criteria (AND logic)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock).
    /// This is an unrecoverable error that should not occur in normal operation.
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
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
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
            description: "Test Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        let not_found = service.update_inventory(9999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_product_filtering() {
        let service = ProductService::new();

        // Create test products
        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow banana".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test name filter (case-insensitive)
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            name_contains: None,
            min_price: Some(Decimal::new(100, 2)), // $1.00
            max_price: Some(Decimal::new(180, 2)), // $1.80
            in_stock: None,
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test in_stock filter
        let filtered = service.filter(&ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test combined filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange contain "a"

        // Test empty filter (returns all)
        let filtered = service.filter(&ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        });
        assert_eq!(filtered.len(), 3);

        // Test out of stock filter
        let filtered = service.filter(&ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(false),
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Banana");
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());

        // Spawn multiple threads creating products concurrently
        let mut handles = vec![];

        for i in 0_i32..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(1000 + i64::from(i) * 100, 2),
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

        // Verify IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: Vec<i32> = (1..=10).collect();
        assert_eq!(ids, unique_ids);
    }
}
