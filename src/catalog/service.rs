use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
#[derive(Debug, Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Create a new empty product service
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new product with auto-incrementing ID
    ///
    /// # Panics
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

    /// Get all products
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Get a product by ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Update the inventory count for a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
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

    /// Filter products based on criteria
    ///
    /// # Panics
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

    /// Delete a product by ID
    ///
    /// Returns true if the product was deleted, false if not found
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (another thread panicked while holding the lock)
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
    use std::sync::Arc;
    use std::thread;

    fn create_test_product(name: &str, price: &str, inventory: i32) -> NewProduct {
        NewProduct {
            name: name.to_string(),
            description: format!("Description for {name}"),
            price: Decimal::from_str(price).unwrap(),
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = create_test_product("Test Product", "19.99", 10);

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(create_test_product("Product 1", "10.00", 5));
        let product2 = service.create(create_test_product("Product 2", "20.00", 3));
        let product3 = service.create(create_test_product("Product 3", "30.00", 7));

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 3));

        let products = service.get_all();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Test Product", "15.50", 8));

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
        let product = service.create(create_test_product("Test Product", "10.00", 10));

        let updated = service.update_inventory(product.id, 25);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 25);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let updated = service.update_inventory(999, 25);
        assert!(updated.is_none());
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", "10.00", 10));

        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Blue Widget", "10.00", 5));
        let _ = service.create(create_test_product("Red Widget", "15.00", 3));
        let _ = service.create(create_test_product("Blue Gadget", "20.00", 7));

        let filter = ProductFilter::new().with_name("blue".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Blue Widget"));
        assert!(results.iter().any(|p| p.name == "Blue Gadget"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "15.00", 3));
        let _ = service.create(create_test_product("Product 3", "20.00", 7));
        let _ = service.create(create_test_product("Product 4", "25.00", 2));

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("12.00").unwrap())
            .with_max_price(Decimal::from_str("22.00").unwrap());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Product 2"));
        assert!(results.iter().any(|p| p.name == "Product 3"));
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "15.00", 0));
        let _ = service.create(create_test_product("Also In Stock", "20.00", 1));

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "15.00", 0));
        let _ = service.create(create_test_product("Also Out", "20.00", 0));

        let filter = ProductFilter::new().with_in_stock(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count == 0));
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Blue Widget", "10.00", 5));
        let _ = service.create(create_test_product("Blue Gadget", "15.00", 0));
        let _ = service.create(create_test_product("Red Widget", "12.00", 3));
        let _ = service.create(create_test_product("Blue Tool", "8.00", 2));

        let filter = ProductFilter::new()
            .with_name("blue".to_string())
            .with_min_price(Decimal::from_str("9.00").unwrap())
            .with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Blue Widget");
    }

    #[test]
    fn test_filter_empty() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "15.00", 3));

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", "10.00", 5));

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
        let product = service.create(create_test_product("Precise Product", "19.99", 5));

        assert_eq!(product.price.to_string(), "19.99");

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_creation() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let product = create_test_product(&format!("Product {i}"), "10.00", i);
                service_clone.create(product)
            });
            handles.push(handle);
        }

        let mut products: Vec<Product> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        products.sort_by_key(|p| p.id);

        // All products should have unique IDs
        assert_eq!(products.len(), 10);
        for (i, product) in products.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let expected_id = i as i32 + 1;
            assert_eq!(product.id, expected_id);
        }

        // All products should be in the service
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = Arc::new(ProductService::new());

        // Create initial products
        for i in 0..5 {
            let _ = service.create(create_test_product(&format!("Product {i}"), "10.00", 10));
        }

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let products = service_clone.get_all();
                assert!(products.len() >= 5);
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for i in 5..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ =
                    service_clone.create(create_test_product(&format!("Product {i}"), "15.00", 5));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_default_implementation() {
        let service = ProductService::default();
        let product = service.create(create_test_product("Test", "10.00", 5));
        assert_eq!(product.id, 1);
    }
}
