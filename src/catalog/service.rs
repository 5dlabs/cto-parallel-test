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

    /// Returns all products in the catalog
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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

    /// Filters products based on the given criteria
    ///
    /// All filters are applied with AND logic. Empty/None filters match all products.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Min price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Max price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // In stock filter
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
    /// Returns true if the product was deleted, false if it wasn't found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Failed to lock products");
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

    fn create_test_product(name: &str, price: &str, inventory: i32) -> NewProduct {
        NewProduct {
            name: name.to_string(),
            description: format!("Description for {name}"),
            price: Decimal::from_str(price).expect("Invalid price"),
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = create_test_product("Laptop", "999.99", 10);

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.price, Decimal::from_str("999.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
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
        let created = service.create(create_test_product("Laptop", "999.99", 10));

        let found = service.get_by_id(created.id);

        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Laptop");
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
        let product = service.create(create_test_product("Laptop", "999.99", 10));

        let updated = service.update_inventory(product.id, 5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();

        let updated = service.update_inventory(999, 5);

        assert!(updated.is_none());
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Laptop", "999.99", 10));

        let updated = service.update_inventory(product.id, -5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop", "999.99", 10));
        let _ = service.create(create_test_product("Desktop", "1299.99", 5));
        let _ = service.create(create_test_product("Laptop Pro", "1999.99", 3));

        let filter = ProductFilter::new().with_name("laptop");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.name.to_lowercase().contains("laptop")));
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop", "999.99", 10));
        let _ = service.create(create_test_product("DESKTOP", "1299.99", 5));

        let filter = ProductFilter::new().with_name("LAPTOP");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_min_price() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 10));
        let _ = service.create(create_test_product("Medium", "50.00", 5));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let filter = ProductFilter::new().with_min_price(Decimal::from_str("50.00").unwrap());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.price >= Decimal::from_str("50.00").unwrap()));
    }

    #[test]
    fn test_filter_by_max_price() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 10));
        let _ = service.create(create_test_product("Medium", "50.00", 5));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let filter = ProductFilter::new().with_max_price(Decimal::from_str("50.00").unwrap());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.price <= Decimal::from_str("50.00").unwrap()));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 10));
        let _ = service.create(create_test_product("Medium", "50.00", 5));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("20.00").unwrap())
            .with_max_price(Decimal::from_str("80.00").unwrap());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
    }

    #[test]
    fn test_filter_in_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));
        let _ = service.create(create_test_product("Available 2", "30.00", 1));

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));
        let _ = service.create(create_test_product("Available 2", "30.00", 1));

        let filter = ProductFilter::new().with_in_stock(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].inventory_count, 0);
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop Basic", "500.00", 10));
        let _ = service.create(create_test_product("Laptop Pro", "1500.00", 5));
        let _ = service.create(create_test_product("Desktop Pro", "1200.00", 3));
        let _ = service.create(create_test_product("Laptop Ultra", "2000.00", 0));

        let filter = ProductFilter::new()
            .with_name("laptop")
            .with_min_price(Decimal::from_str("600.00").unwrap())
            .with_max_price(Decimal::from_str("1800.00").unwrap())
            .with_in_stock(true);

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Pro");
    }

    #[test]
    fn test_filter_empty() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 0));

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Laptop", "999.99", 10));

        let deleted = service.delete(product.id);

        assert!(deleted);
        assert!(service.get_by_id(product.id).is_none());
        assert_eq!(service.get_all().len(), 0);
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
        let new_product = create_test_product("Precise Product", "99.999", 10);

        let product = service.create(new_product);

        assert_eq!(product.price, Decimal::from_str("99.999").unwrap());

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, Decimal::from_str("99.999").unwrap());
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let service = ProductService::new();
        let mut handles = vec![];

        // Spawn 10 threads that each create 10 products
        for i in 0..10 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(create_test_product(
                        &format!("Product-{i}-{j}"),
                        "10.00",
                        1,
                    ));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify all 100 products were created
        let products = service.get_all();
        assert_eq!(products.len(), 100);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 100);
    }

    #[test]
    fn test_concurrent_read_write() {
        use std::thread;
        use std::time::Duration;

        let service = ProductService::new();

        // Create some initial products
        for i in 0..10 {
            let _ = service.create(create_test_product(&format!("Product {i}"), "10.00", 10));
        }

        let mut handles = vec![];

        // Spawn reader threads
        for _ in 0..5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                for _ in 0..20 {
                    let products = service_clone.get_all();
                    assert!(!products.is_empty());
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }

        // Spawn writer threads
        for i in 0..5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(create_test_product(&format!("New-{i}-{j}"), "20.00", 5));
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify final state
        let products = service.get_all();
        assert_eq!(products.len(), 60); // 10 initial + 50 new
    }
}
