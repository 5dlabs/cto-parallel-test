use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage.
///
/// This service provides CRUD operations for products with auto-incrementing IDs,
/// inventory management, and flexible filtering capabilities.
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

    /// Creates a new product with an auto-generated ID.
    ///
    /// # Arguments
    /// * `new_product` - The product details without an ID
    ///
    /// # Returns
    /// The created product with its assigned ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned (should not happen in normal operation)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");

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
    /// # Returns
    /// A vector containing clones of all products
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria.
    ///
    /// All filter criteria use AND logic. Empty filters return all products.
    ///
    /// # Arguments
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    /// A vector of products matching all filter criteria
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
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
    /// # Arguments
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    /// `true` if a product was deleted, `false` if no product with that ID exists
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Products mutex poisoned");
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
    use std::thread;

    fn create_test_product(name: &str, price: &str, inventory: i32) -> NewProduct {
        NewProduct {
            name: name.to_string(),
            description: format!("Description of {name}"),
            price: Decimal::from_str(price).expect("Invalid price"),
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
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(create_test_product("Product 1", "10.00", 5));
        let product2 = service.create(create_test_product("Product 2", "20.00", 10));
        let product3 = service.create(create_test_product("Product 3", "30.00", 15));

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 10));

        let all_products = service.get_all();

        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0].name, "Product 1");
        assert_eq!(all_products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Test Product", "15.99", 8));

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

        let fetched = service.get_by_id(product.id);
        assert_eq!(fetched.unwrap().inventory_count, 25);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();

        let updated = service.update_inventory(999, 10);

        assert!(updated.is_none());
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", "10.00", 10));

        let deleted = service.delete(product.id);

        assert!(deleted);
        assert!(service.get_by_id(product.id).is_none());
    }

    #[test]
    fn test_delete_product_not_found() {
        let service = ProductService::new();

        let deleted = service.delete(999);

        assert!(!deleted);
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop Computer", "999.99", 5));
        let _ = service.create(create_test_product("Desktop Computer", "1299.99", 3));
        let _ = service.create(create_test_product("Tablet Device", "499.99", 10));

        let filter = ProductFilter::new().with_name("computer");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Laptop Computer"));
        assert!(results.iter().any(|p| p.name == "Desktop Computer"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap Item", "10.00", 10));
        let _ = service.create(create_test_product("Mid Item", "50.00", 10));
        let _ = service.create(create_test_product("Expensive Item", "100.00", 10));

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("20.00").unwrap())
            .with_max_price(Decimal::from_str("80.00").unwrap());
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_in_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available Product", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock Product", "20.00", 0));

        let filter = ProductFilter::new().with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Available Product");
    }

    #[test]
    fn test_filter_by_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available Product", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock Product", "20.00", 0));

        let filter = ProductFilter::new().with_in_stock(false);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock Product");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop A", "500.00", 5));
        let _ = service.create(create_test_product("Laptop B", "1500.00", 3));
        let _ = service.create(create_test_product("Desktop A", "800.00", 0));
        let _ = service.create(create_test_product("Tablet", "300.00", 10));

        let filter = ProductFilter::new()
            .with_name("laptop")
            .with_max_price(Decimal::from_str("1000.00").unwrap())
            .with_in_stock(true);
        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop A");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 10));

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Precise Item", "19.99", 10));

        // Verify decimal precision is maintained
        assert_eq!(
            product.price,
            Decimal::from_str("19.99").expect("Invalid decimal")
        );

        // Test price comparisons work correctly
        let filter =
            ProductFilter::new().with_min_price(Decimal::from_str("19.98").expect("Invalid"));
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);

        let filter =
            ProductFilter::new().with_max_price(Decimal::from_str("20.00").expect("Invalid"));
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_concurrent_access() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(create_test_product(
                    &format!("Thread1-Product{i}"),
                    "10.00",
                    5,
                ));
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(create_test_product(
                    &format!("Thread2-Product{i}"),
                    "20.00",
                    10,
                ));
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(*id, i32::try_from(i + 1).expect("Invalid cast"));
        }
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 0..5 {
            let _ = service.create(create_test_product(
                &format!("Initial Product {i}"),
                "10.00",
                5,
            ));
        }

        let service_write = service.clone();
        let service_read = service.clone();

        let write_handle = thread::spawn(move || {
            for i in 5..10 {
                let _ = service_write.create(create_test_product(&format!("New Product {i}"), "15.00", 3));
            }
        });

        let read_handle = thread::spawn(move || {
            let mut read_count = 0;
            for _ in 0..100 {
                let products = service_read.get_all();
                read_count = products.len();
            }
            read_count
        });

        write_handle.join().expect("Write thread panicked");
        let final_read_count = read_handle.join().expect("Read thread panicked");

        assert!(final_read_count >= 5);
        assert_eq!(service.get_all().len(), 10);
    }

    #[test]
    fn test_negative_inventory_allowed() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", "10.00", 10));

        let updated = service.update_inventory(product.id, -5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_case_insensitive_name_filter() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("UPPERCASE PRODUCT", "10.00", 5));
        let _ = service.create(create_test_product("lowercase product", "20.00", 10));
        let _ = service.create(create_test_product("MiXeD CaSe PrOdUcT", "30.00", 15));

        let filter = ProductFilter::new().with_name("product");
        let results = service.filter(&filter);

        assert_eq!(results.len(), 3);
    }
}
