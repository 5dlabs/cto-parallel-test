use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service for catalog management.
/// Uses Arc<Mutex<>> for safe concurrent access across multiple threads.
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty `ProductService` with ID counter starting at 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product with an auto-generated ID.
    /// Returns the created product with its assigned ID.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not occur in normal operation).
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("products mutex poisoned");
        let mut next_id = self.next_id.lock().expect("next_id mutex poisoned");

        let id = *next_id;
        *next_id += 1;

        let product = Product {
            id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        products.push(product.clone());
        product
    }

    /// Returns all products in the catalog.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not occur in normal operation).
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID.
    /// Returns None if no product with the given ID exists.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not occur in normal operation).
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    /// Returns the updated product, or None if the product doesn't exist.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not occur in normal operation).
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("products mutex poisoned");
        products.iter_mut().find(|p| p.id == id).map(|p| {
            p.inventory_count = new_count;
            p.clone()
        })
    }

    /// Filters products based on the provided criteria.
    /// All filter fields are optional and combined with AND logic.
    ///
    /// - `name_contains`: Case-insensitive substring match
    /// - `min_price`: Price >= `min_price` (inclusive)
    /// - `max_price`: Price <= `max_price` (inclusive)
    /// - `in_stock`: true = `inventory_count` > 0, false = `inventory_count` == 0
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not occur in normal operation).
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("products mutex poisoned");

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Min price filter: price >= min_price
                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                // Max price filter: price <= max_price
                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

                // In stock filter: true = inventory_count > 0, false = inventory_count == 0
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // Combine all filters with AND logic
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

        // Initially empty
        assert_eq!(service.get_all().len(), 0);

        let created = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test description".to_string(),
            price: Decimal::new(1000, 2), // $10.00
            inventory_count: 5,
        });

        // Found by ID
        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);

        // Not found
        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());

        // Get all returns one product
        let all = service.get_all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, created.id);
    }

    #[test]
    fn test_inventory_update() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        // Update existing product
        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        // Verify update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        // Update non-existent product
        let not_updated = service.update_inventory(9999, 10);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_product_filtering() {
        let service = ProductService::new();

        // Create test products
        let _apple = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _banana = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow banana".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _orange = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test empty filter (returns all)
        let all = service.filter(&ProductFilter::default());
        assert_eq!(all.len(), 3);

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

        // Test in_stock filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Banana
        assert_eq!(filtered[0].name, "Banana");

        // Test combined filters (name + in_stock)
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()), // matches Apple, Banana, Orange
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange (both contain "a" and in stock)
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());

        // Create products from multiple threads
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(1000 + i64::from(i), 2),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().expect("thread panicked");
        }

        // Verify all products were created
        let all = service.get_all();
        assert_eq!(all.len(), 10);
    }
}
