use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new `ProductService` with empty product list and ID counter starting at 1
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product with auto-incrementing ID
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
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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

    /// Filters products based on provided criteria
    ///
    /// All filter criteria are combined with AND logic.
    /// - `name_contains`: Case-insensitive substring match on product name
    /// - `min_price`: Products with price >= `min_price`
    /// - `max_price`: Products with price <= `max_price`
    /// - `in_stock`: Products with `inventory_count` > 0 when true, == 0 when false
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();

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
    fn test_new_service_is_empty() {
        let service = ProductService::new();
        let products = service.get_all();
        assert_eq!(products.len(), 0);
    }

    #[test]
    fn test_product_creation_assigns_sequential_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Test Product 1".to_string(),
            description: "First test product".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Test Product 1");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        let product2 = service.create(NewProduct {
            name: "Test Product 2".to_string(),
            description: "Second test product".to_string(),
            price: Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        assert_eq!(product2.id, 2);
        assert_eq!(product2.name, "Test Product 2");
    }

    #[test]
    fn test_get_all_returns_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 20,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id_finds_existing_product() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Find Me".to_string(),
            description: "Test product".to_string(),
            price: Decimal::new(1500, 2),
            inventory_count: 15,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        let found_product = found.unwrap();
        assert_eq!(found_product.id, created.id);
        assert_eq!(found_product.name, "Find Me");
    }

    #[test]
    fn test_get_by_id_returns_none_for_nonexistent_id() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory_modifies_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 25);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        // Verify the change persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_returns_none_for_nonexistent_id() {
        let service = ProductService::new();

        let result = service.update_inventory(9999, 100);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(75, 2),
            inventory_count: 0,
        });

        let filter = ProductFilter::default();
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(75, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 8,
        });

        let filter = ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        };
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(50, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(300, 2),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(200, 2)),
            ..Default::default()
        };
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Also In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        };
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 2);

        let filter = ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        };
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple Fresh".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple Rotten".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(50, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Fruit".to_string(),
            price: Decimal::new(175, 2),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            name_contains: Some("apple".to_string()),
            min_price: Some(Decimal::new(100, 2)),
            in_stock: Some(true),
            ..Default::default()
        };
        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple Fresh");
    }

    #[test]
    fn test_service_is_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create products from multiple threads
        for i in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(100 * i64::from(i), 2),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 5);
    }
}
