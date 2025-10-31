use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with CRUD operations
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<u64>>,
}

impl ProductService {
    /// Creates a new `ProductService` instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product in the catalog
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product data to create
    ///
    /// # Returns
    ///
    /// The newly created `Product` with an assigned ID
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

    /// Retrieves all products from the catalog
    ///
    /// # Returns
    ///
    /// A vector containing all products
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the product to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: u64) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the product to update
    /// * `new_inventory` - The new inventory count
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: u64, new_inventory: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Failed to lock products");

        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_inventory;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    ///
    /// A vector containing all products matching the filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products
            .iter()
            .filter(|p| filter.matches(p))
            .cloned()
            .collect()
    }

    /// Clears all products from the catalog (useful for testing)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[cfg(test)]
    pub fn clear(&self) {
        let mut products = self.products.lock().expect("Failed to lock products");
        let mut next_id = self.next_id.lock().expect("Failed to lock next_id");
        products.clear();
        *next_id = 1;
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

    fn create_test_product(name: &str, price: rust_decimal::Decimal, inventory: i32) -> NewProduct {
        NewProduct {
            name: name.to_string(),
            description: Some(format!("Description for {name}")),
            price,
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = create_test_product("Test Product", dec!(99.99), 10);

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(99.99));
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_create_multiple_products_auto_increment_id() {
        let service = ProductService::new();

        let product1 = service.create(create_test_product("Product 1", dec!(10.00), 5));
        let product2 = service.create(create_test_product("Product 2", dec!(20.00), 10));
        let product3 = service.create(create_test_product("Product 3", dec!(30.00), 15));

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(create_test_product("Product 1", dec!(10.00), 5));
        let _ = service.create(create_test_product("Product 2", dec!(20.00), 10));

        let products = service.get_all();

        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_all_empty() {
        let service = ProductService::new();
        let products = service.get_all();
        assert!(products.is_empty());
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Test Product", dec!(99.99), 10));

        let found = service.get_by_id(created.id);

        assert!(found.is_some());
        let product = found.unwrap();
        assert_eq!(product.id, created.id);
        assert_eq!(product.name, "Test Product");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let found = service.get_by_id(999);
        assert!(found.is_none());
    }

    #[test]
    fn test_update_inventory_success() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", dec!(99.99), 10));

        let updated = service.update_inventory(product.id, 25);

        assert!(updated.is_some());
        let updated_product = updated.unwrap();
        assert_eq!(updated_product.inventory_count, 25);

        // Verify the change persisted
        let fetched = service.get_by_id(product.id).unwrap();
        assert_eq!(fetched.inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let updated = service.update_inventory(999, 25);
        assert!(updated.is_none());
    }

    #[test]
    fn test_update_inventory_to_zero() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", dec!(99.99), 10));

        let updated = service.update_inventory(product.id, 0);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 0);
    }

    #[test]
    fn test_update_inventory_negative() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", dec!(99.99), 10));

        let updated = service.update_inventory(product.id, -5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Gaming Laptop", dec!(1500.00), 5));
        let _ = service.create(create_test_product("Office Laptop", dec!(800.00), 10));
        let _ = service.create(create_test_product("Desktop PC", dec!(1200.00), 3));

        let filter = ProductFilter::new().with_name("laptop".to_string());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Office Laptop"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap Item", dec!(50.00), 10));
        let _ = service.create(create_test_product("Mid Item", dec!(500.00), 5));
        let _ = service.create(create_test_product("Expensive Item", dec!(2000.00), 2));

        let filter = ProductFilter::new()
            .with_min_price(dec!(100.00))
            .with_max_price(dec!(1000.00));
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_in_stock_only() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", dec!(100.00), 5));
        let _ = service.create(create_test_product("Out of Stock", dec!(100.00), 0));
        let _ = service.create(create_test_product("Also In Stock", dec!(100.00), 1));

        let filter = ProductFilter::new().with_in_stock_only(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Gaming Laptop", dec!(1500.00), 5));
        let _ = service.create(create_test_product("Office Laptop", dec!(800.00), 0));
        let _ = service.create(create_test_product("Budget Laptop", dec!(400.00), 3));
        let _ = service.create(create_test_product("Desktop PC", dec!(1200.00), 10));

        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(dec!(500.00))
            .with_max_price(dec!(2000.00))
            .with_in_stock_only(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_no_matches() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product", dec!(100.00), 5));

        let filter = ProductFilter::new().with_name("nonexistent".to_string());
        let results = service.filter(&filter);

        assert!(results.is_empty());
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = ProductService::new();
        let mut handles = vec![];

        // Create products from multiple threads
        for i in 0..10 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                let _ = service_clone.create(create_test_product(
                    &format!("Product {i}"),
                    dec!(100.00),
                    i,
                ));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify all IDs are unique
        let mut ids: Vec<u64> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn test_concurrent_inventory_updates() {
        use std::thread;

        let service = ProductService::new();
        let product = service.create(create_test_product("Test", dec!(100.00), 100));
        let product_id = product.id;

        let mut handles = vec![];

        // Update inventory from multiple threads
        for i in 0..5 {
            let service_clone = service.clone();
            let handle = thread::spawn(move || {
                let _ = service_clone.update_inventory(product_id, 100 + i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify the product exists and has some inventory value
        let final_product = service.get_by_id(product_id);
        assert!(final_product.is_some());
        let inventory = final_product.unwrap().inventory_count;
        assert!((100..105).contains(&inventory));
    }
}
