use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage.
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and returns it with an assigned ID.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
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

    /// Returns all products in the catalog.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// Returns the updated product if found, `None` otherwise.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Mutex poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the given criteria.
    ///
    /// All filter criteria are combined with AND logic. Empty filter returns all products.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
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

    /// Deletes a product by its ID.
    ///
    /// Returns `true` if the product was deleted, `false` if it was not found.
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Mutex poisoned");
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

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, dec!(1299.99));
        assert_eq!(product.inventory_count, 5);
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
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
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Laptop");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, 10);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 10);

        let found = service.get_by_id(product.id);
        assert_eq!(found.unwrap().inventory_count, 10);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, -1);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -1);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Office Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });

        let filter = ProductFilter::new().with_name("laptop".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 50,
        });

        let _ = service.create(NewProduct {
            name: "Keyboard".to_string(),
            description: "Mechanical keyboard".to_string(),
            price: dec!(149.99),
            inventory_count: 20,
        });

        let filter = ProductFilter::new()
            .with_min_price(dec!(50.00))
            .with_max_price(dec!(500.00));

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Keyboard");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock Item".to_string(),
            description: "Available".to_string(),
            price: dec!(99.99),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock Item".to_string(),
            description: "Not available".to_string(),
            price: dec!(99.99),
            inventory_count: 0,
        });

        let in_stock_filter = ProductFilter::new().with_in_stock(true);
        let in_stock_results = service.filter(&in_stock_filter);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock Item");

        let out_of_stock_filter = ProductFilter::new().with_in_stock(false);
        let out_of_stock_results = service.filter(&out_of_stock_filter);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock Item");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Budget laptop".to_string(),
            price: dec!(499.99),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Gaming Mouse".to_string(),
            description: "RGB mouse".to_string(),
            price: dec!(79.99),
            inventory_count: 30,
        });

        let filter = ProductFilter::new()
            .with_name("gaming".to_string())
            .with_min_price(dec!(1000.00))
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_empty() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 5,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Try deleting again
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
                    price: dec!(10.00),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        let mut ids = vec![];
        for handle in handles {
            let product = handle.join().expect("Thread panicked");
            ids.push(product.id);
        }

        // All IDs should be unique
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);

        // All products should be in the service
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_concurrent_read() {
        let service = Arc::new(ProductService::new());

        // Create some products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: format!("Description {i}"),
                price: dec!(10.00),
                inventory_count: i,
            });
        }

        let mut handles = vec![];

        // Multiple threads reading concurrently
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
    fn test_decimal_precision_in_service() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "High Precision Item".to_string(),
            description: "Test".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        });

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price.to_string(), "19.999");
    }
}
