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

    /// Creates a new product and returns it with an assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// Returns `Some(Product)` if found, `None` otherwise
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
    /// Returns the updated product if found, `None` otherwise
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

    /// Filters products based on the provided criteria
    ///
    /// All criteria are combined with AND logic. Empty/None criteria match all products.
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
                // Name filter (case-insensitive substring match)
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Min price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Max price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // Stock status filter
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
    /// Returns `true` if the product was deleted, `false` if not found
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
    use rust_decimal::prelude::*;
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
        let product3 = service.create(create_test_product("Product 3", "30.00", 8));

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
    fn test_get_all_empty() {
        let service = ProductService::new();
        let products = service.get_all();
        assert_eq!(products.len(), 0);
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Laptop", "999.99", 10));

        let product = service.get_by_id(created.id);

        assert!(product.is_some());
        let product = product.unwrap();
        assert_eq!(product.id, created.id);
        assert_eq!(product.name, "Laptop");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop", "999.99", 10));

        let product = service.get_by_id(999);

        assert!(product.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Laptop", "999.99", 10));

        let updated = service.update_inventory(created.id, 5);

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.inventory_count, 5);

        // Verify the change persists
        let retrieved = service.get_by_id(created.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);
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
        let created = service.create(create_test_product("Laptop", "999.99", 10));

        let updated = service.update_inventory(created.id, -5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Gaming Laptop", "1500.00", 5));
        let _ = service.create(create_test_product("Office Laptop", "800.00", 10));
        let _ = service.create(create_test_product("Gaming Mouse", "50.00", 20));

        let filter = ProductFilter::with_name("laptop");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Office Laptop"));
    }

    #[test]
    fn test_filter_case_insensitive() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("LAPTOP", "1000.00", 5));

        let filter = ProductFilter::with_name("laptop");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "LAPTOP");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 5));
        let _ = service.create(create_test_product("Medium", "50.00", 10));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let min = Decimal::from_str("20.00").unwrap();
        let max = Decimal::from_str("80.00").unwrap();
        let filter = ProductFilter::with_price_range(Some(min), Some(max));
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_min_price_only() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 5));
        let _ = service.create(create_test_product("Medium", "50.00", 10));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let min = Decimal::from_str("50.00").unwrap();
        let filter = ProductFilter::with_price_range(Some(min), None);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Medium"));
        assert!(results.iter().any(|p| p.name == "Expensive"));
    }

    #[test]
    fn test_filter_by_max_price_only() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 5));
        let _ = service.create(create_test_product("Medium", "50.00", 10));
        let _ = service.create(create_test_product("Expensive", "100.00", 3));

        let max = Decimal::from_str("50.00").unwrap();
        let filter = ProductFilter::with_price_range(None, Some(max));
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Cheap"));
        assert!(results.iter().any(|p| p.name == "Medium"));
    }

    #[test]
    fn test_filter_in_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));
        let _ = service.create(create_test_product("Also In Stock", "30.00", 1));

        let filter = ProductFilter::with_stock_status(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));

        let filter = ProductFilter::with_stock_status(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock");
        assert_eq!(results[0].inventory_count, 0);
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Gaming Laptop", "1500.00", 5));
        let _ = service.create(create_test_product("Office Laptop", "800.00", 10));
        let _ = service.create(create_test_product("Budget Laptop", "400.00", 0));
        let _ = service.create(create_test_product("Gaming Mouse", "50.00", 20));

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(Decimal::from_str("500.00").unwrap()),
            max_price: Some(Decimal::from_str("2000.00").unwrap()),
            in_stock: Some(true),
        };
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Office Laptop"));
    }

    #[test]
    fn test_filter_empty() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 3));

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Laptop", "999.99", 10));

        let deleted = service.delete(created.id);

        assert!(deleted);
        assert!(service.get_by_id(created.id).is_none());
    }

    #[test]
    fn test_delete_not_found() {
        let service = ProductService::new();
        let deleted = service.delete(999);
        assert!(!deleted);
    }

    #[test]
    fn test_concurrent_creation() {
        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn 10 threads each creating 10 products
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(create_test_product(
                        &format!("Product-{i}-{j}"),
                        "10.00",
                        5,
                    ));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all products were created with unique IDs
        let products = service.get_all();
        assert_eq!(products.len(), 100);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        for i in 0..ids.len() - 1 {
            assert_ne!(ids[i], ids[i + 1], "Found duplicate ID");
        }
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = Arc::new(ProductService::new());

        // Pre-populate with some products
        for i in 0..10 {
            let _ = service.create(create_test_product(&format!("Product {i}"), "10.00", 5));
        }

        let mut handles = vec![];

        // Spawn readers
        for _ in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for _ in 0..20 {
                    let _ = service_clone.get_all();
                }
            });
            handles.push(handle);
        }

        // Spawn writers
        for i in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(create_test_product(
                        &format!("New-{i}-{j}"),
                        "20.00",
                        3,
                    ));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify final state
        let products = service.get_all();
        assert_eq!(products.len(), 60); // 10 initial + 50 new
    }

    #[test]
    fn test_decimal_precision_maintained() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Precise Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 1,
        };

        let product = service.create(new_product);
        let retrieved = service.get_by_id(product.id).unwrap();

        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_service_clone() {
        let service1 = ProductService::new();
        let _ = service1.create(create_test_product("Product 1", "10.00", 5));

        let service2 = service1.clone();
        let _ = service2.create(create_test_product("Product 2", "20.00", 3));

        // Both services should see all products (they share the same storage)
        assert_eq!(service1.get_all().len(), 2);
        assert_eq!(service2.get_all().len(), 2);
    }
}
