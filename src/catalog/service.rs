use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Create a new empty `ProductService`
    #[must_use]
    pub fn new() -> Self {
        ProductService {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new product with auto-incrementing ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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

    /// Get all products
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Get a product by ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Update inventory count for a product
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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

    /// Filter products based on criteria
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products
            .iter()
            .filter(|p| {
                // Check name filter (case-insensitive substring match)
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Check minimum price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Check maximum price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // Check stock filter
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // All filters must pass (AND logic)
                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
    }

    /// Delete a product by ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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
    use std::thread;

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
        assert_eq!(product.price, dec!(19.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id() {
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

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
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

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let not_found = service.update_inventory(999, 5);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Computer".to_string(),
            description: "High-performance laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse Pad".to_string(),
            description: "Gaming mouse pad".to_string(),
            price: dec!(15.99),
            inventory_count: 20,
        });

        let mut filter = ProductFilter::new();
        filter.name_contains = Some("laptop".to_string());

        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Laptop Computer");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price item".to_string(),
            price: dec!(5.99),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mid Range Item".to_string(),
            description: "Mid price item".to_string(),
            price: dec!(25.99),
            inventory_count: 8,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price item".to_string(),
            price: dec!(199.99),
            inventory_count: 2,
        });

        let mut filter = ProductFilter::new();
        filter.min_price = Some(dec!(10.00));
        filter.max_price = Some(dec!(50.00));

        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Mid Range Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock Item".to_string(),
            description: "Available item".to_string(),
            price: dec!(15.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock Item".to_string(),
            description: "Unavailable item".to_string(),
            price: dec!(25.99),
            inventory_count: 0,
        });

        // Filter for in-stock items
        let mut filter = ProductFilter::new();
        filter.in_stock = Some(true);
        let in_stock = service.filter(&filter);
        assert_eq!(in_stock.len(), 1);
        assert_eq!(in_stock[0].name, "In Stock Item");

        // Filter for out-of-stock items
        let mut filter = ProductFilter::new();
        filter.in_stock = Some(false);
        let out_of_stock = service.filter(&filter);
        assert_eq!(out_of_stock.len(), 1);
        assert_eq!(out_of_stock[0].name, "Out of Stock Item");
    }

    #[test]
    fn test_combined_filters() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming laptop".to_string(),
            price: dec!(1299.99),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business laptop".to_string(),
            price: dec!(699.99),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Gaming Mouse".to_string(),
            description: "Gaming mouse".to_string(),
            price: dec!(49.99),
            inventory_count: 15,
        });

        let mut filter = ProductFilter::new();
        filter.name_contains = Some("gaming".to_string());
        filter.min_price = Some(dec!(100.00));
        filter.in_stock = Some(true);

        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());

        let not_deleted = service.delete(999);
        assert!(!not_deleted);
    }

    #[test]
    fn test_concurrent_access() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create multiple threads that add products concurrently
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: dec!(10.00) + rust_decimal::Decimal::from(i),
                    inventory_count: i,
                })
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        let mut created_products = vec![];
        for handle in handles {
            created_products.push(handle.join().unwrap());
        }

        // Verify all products were created with unique IDs
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        let mut ids: Vec<i32> = created_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let expected_ids: Vec<i32> = (1..=10).collect();
        assert_eq!(ids, expected_ids);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Testing decimal precision".to_string(),
            price: dec!(123.456789),
            inventory_count: 1,
        });

        // Verify decimal precision is maintained
        assert_eq!(product.price, dec!(123.456789));

        // Test price comparisons work correctly
        let mut filter = ProductFilter::new();
        filter.min_price = Some(dec!(123.45));
        filter.max_price = Some(dec!(123.46));

        let filtered = service.filter(&filter);
        assert_eq!(filtered.len(), 1);
    }
}
