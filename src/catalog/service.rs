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

    /// Creates a new product and assigns it a unique ID
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product details to create
    ///
    /// # Returns
    ///
    /// The created product with its assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    ///
    /// Some(product) if found, None otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    ///
    /// `Some(updated_product)` if found, None otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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

    /// Filters products based on the provided criteria
    ///
    /// All filter criteria are combined with AND logic.
    /// Empty/None criteria match all products.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    ///
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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

    /// Deletes a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    ///
    /// true if the product was deleted, false if not found
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned
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
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;
    use std::thread;

    fn create_test_product(name: &str, price: f64, inventory: i32) -> NewProduct {
        NewProduct {
            name: String::from(name),
            description: format!("Description for {name}"),
            price: Decimal::from_f64(price).unwrap(),
            inventory_count: inventory,
        }
    }

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = create_test_product("Test Product", 19.99, 10);

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.inventory_count, 10);
        assert_eq!(product.price, Decimal::from_f64(19.99).unwrap());
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(create_test_product("Product 1", 10.0, 5));
        let product2 = service.create(create_test_product("Product 2", 20.0, 10));
        let product3 = service.create(create_test_product("Product 3", 30.0, 15));

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
        assert_eq!(product3.id, 3);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(create_test_product("Product 1", 10.0, 5));
        let _ = service.create(create_test_product("Product 2", 20.0, 10));

        let products = service.get_all();

        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(create_test_product("Test Product", 15.0, 8));

        let found = service.get_by_id(created.id);

        assert!(found.is_some());
        let product = found.unwrap();
        assert_eq!(product.id, created.id);
        assert_eq!(product.name, "Test Product");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();

        let result = service.get_by_id(999);

        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory_success() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", 10.0, 5));

        let updated = service.update_inventory(product.id, 20);

        assert!(updated.is_some());
        let updated_product = updated.unwrap();
        assert_eq!(updated_product.inventory_count, 20);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 20);
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
        let product = service.create(create_test_product("Test Product", 10.0, 5));

        let updated = service.update_inventory(product.id, -5);

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }

    #[test]
    fn test_delete_product_success() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Test Product", 10.0, 5));

        let deleted = service.delete(product.id);

        assert!(deleted);
        assert!(service.get_by_id(product.id).is_none());
        assert_eq!(service.get_all().len(), 0);
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
        let _ = service.create(create_test_product("Apple iPhone", 999.0, 10));
        let _ = service.create(create_test_product("Samsung Galaxy", 899.0, 15));
        let _ = service.create(create_test_product("Apple MacBook", 1999.0, 5));

        let filter = ProductFilter {
            name_contains: Some(String::from("apple")),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.name.to_lowercase().contains("apple")));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap Product", 10.0, 10));
        let _ = service.create(create_test_product("Mid Product", 50.0, 10));
        let _ = service.create(create_test_product("Expensive Product", 100.0, 10));

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(Decimal::from_f64(20.0).unwrap()),
            max_price: Some(Decimal::from_f64(80.0).unwrap()),
            in_stock: None,
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Product");
    }

    #[test]
    fn test_filter_by_stock_status_in_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", 10.0, 5));
        let _ = service.create(create_test_product("Out of Stock", 20.0, 0));
        let _ = service.create(create_test_product("Also In Stock", 30.0, 1));

        let filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_by_stock_status_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("In Stock", 10.0, 5));
        let _ = service.create(create_test_product("Out of Stock", 20.0, 0));

        let filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(false),
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].inventory_count, 0);
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Apple iPhone", 999.0, 10));
        let _ = service.create(create_test_product("Apple Watch", 399.0, 0));
        let _ = service.create(create_test_product("Apple MacBook", 1999.0, 5));
        let _ = service.create(create_test_product("Samsung Galaxy", 899.0, 15));

        let filter = ProductFilter {
            name_contains: Some(String::from("apple")),
            min_price: Some(Decimal::from_f64(500.0).unwrap()),
            max_price: Some(Decimal::from_f64(1500.0).unwrap()),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple iPhone");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", 10.0, 5));
        let _ = service.create(create_test_product("Product 2", 20.0, 10));

        let filter = ProductFilter::new();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let price = "19.99".parse::<Decimal>().unwrap();
        let new_product = NewProduct {
            name: String::from("Precise Product"),
            description: String::from("Test precision"),
            price,
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_create() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ =
                    service_clone1.create(create_test_product(&format!("Product-T1-{i}"), 10.0, 5));
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(create_test_product(
                    &format!("Product-T2-{i}"),
                    20.0,
                    10,
                ));
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let products = service.get_all();
        assert_eq!(products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 0..5 {
            let _ = service.create(create_test_product(
                &format!("Product {i}"),
                10.0 * f64::from(i),
                10,
            ));
        }

        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let reader = thread::spawn(move || {
            for _ in 0..100 {
                let _ = service_clone1.get_all();
            }
        });

        let writer = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(create_test_product(
                    &format!("New Product {i}"),
                    50.0,
                    5,
                ));
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();

        assert_eq!(service.get_all().len(), 15);
    }
}
