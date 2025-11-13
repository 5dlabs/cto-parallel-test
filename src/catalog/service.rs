use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex, MutexGuard};

/// Acquire a mutex or fail closed with context
fn lock_or_panic<'a, T>(mutex: &'a Mutex<T>, context: &str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(e) => panic!("mutex poisoned while {context}: {e}"),
    }
}

/// Thread-safe in-memory product catalog service
///
/// This service provides CRUD operations and filtering capabilities for products.
/// All operations are thread-safe using `Arc<Mutex<T>>` for internal state.
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

    /// Creates a new product with an auto-incrementing ID
    ///
    /// # Arguments
    /// * `new_product` - The product data to create
    ///
    /// # Returns
    /// The created product with assigned ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        // Basic input hardening: trim text, prevent negative values.
        // This keeps the API infallible while enforcing safe defaults.
        let NewProduct {
            name,
            description,
            price,
            inventory_count,
        } = new_product;
        let name = name.trim().to_string();
        let description = description.trim().to_string();
        let price = if price.is_sign_negative() {
            rust_decimal::Decimal::ZERO
        } else {
            price
        };
        let inventory_count = inventory_count.max(0);
        // Allocate ID under its own lock, then release before locking products
        let id = {
            let mut next_id = lock_or_panic(&self.next_id, "allocating next product id");
            let id = *next_id;
            *next_id += 1;
            id
        };

        let product = Product {
            id,
            name,
            description,
            price,
            inventory_count,
        };

        let mut products = lock_or_panic(&self.products, "pushing new product");
        products.push(product.clone());
        product
    }

    /// Returns all products in the catalog
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = lock_or_panic(&self.products, "reading all products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = lock_or_panic(&self.products, "reading product by id");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = lock_or_panic(&self.products, "updating inventory");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            // Prevent negative inventory; clamp to zero.
            product.inventory_count = new_count.max(0);
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// All filter criteria use AND logic. If a criterion is None, it's ignored.
    ///
    /// # Arguments
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    /// A vector of products matching all specified criteria
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = lock_or_panic(&self.products, "filtering products");
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

    /// Deletes a product by ID
    ///
    /// # Arguments
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    /// `true` if a product was deleted, `false` if not found
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = lock_or_panic(&self.products, "deleting product");
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
            description: format!("Description for {name}"),
            price: Decimal::from_str(price).expect("valid decimal"),
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
        let created = service.create(create_test_product("Test Product", "50.00", 10));

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
        let product = service.create(create_test_product("Product", "10.00", 5));

        let updated = service.update_inventory(product.id, 15);

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.inventory_count, 15);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 15);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();

        let updated = service.update_inventory(999, 10);

        assert!(updated.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Laptop", "1000.00", 5));
        let _ = service.create(create_test_product("Desktop", "1500.00", 3));
        let _ = service.create(create_test_product("Gaming Laptop", "2000.00", 2));

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            ..Default::default()
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Laptop"));
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Cheap", "10.00", 10));
        let _ = service.create(create_test_product("Medium", "50.00", 5));
        let _ = service.create(create_test_product("Expensive", "100.00", 2));

        let filter = ProductFilter {
            min_price: Some(Decimal::from_str("20.00").unwrap()),
            max_price: Some(Decimal::from_str("80.00").unwrap()),
            ..Default::default()
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
    }

    #[test]
    fn test_filter_in_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));
        let _ = service.create(create_test_product("Also Available", "30.00", 1));

        let filter = ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.inventory_count > 0));
    }

    #[test]
    fn test_filter_out_of_stock() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Available", "10.00", 5));
        let _ = service.create(create_test_product("Out of Stock", "20.00", 0));

        let filter = ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].inventory_count, 0);
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Gaming Laptop", "1500.00", 5));
        let _ = service.create(create_test_product("Office Laptop", "800.00", 0));
        let _ = service.create(create_test_product("Gaming Desktop", "2000.00", 3));
        let _ = service.create(create_test_product("Budget Laptop", "500.00", 10));

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(Decimal::from_str("600.00").unwrap()),
            in_stock: Some(true),
            ..Default::default()
        };

        let results = service.filter(&filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Laptop");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();
        let _ = service.create(create_test_product("Product 1", "10.00", 5));
        let _ = service.create(create_test_product("Product 2", "20.00", 3));

        let filter = ProductFilter::default();
        let results = service.filter(&filter);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(create_test_product("To Delete", "10.00", 5));

        let deleted = service.delete(product.id);

        assert!(deleted);
        assert!(service.get_by_id(product.id).is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let service = ProductService::new();

        let deleted = service.delete(999);

        assert!(!deleted);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(create_test_product("Precise", "19.99", 10));

        let retrieved = service.get_by_id(product.id).unwrap();

        assert_eq!(retrieved.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..50 {
                let _ = service_clone1.create(create_test_product(
                    &format!("Product A{i}"),
                    "10.00",
                    5,
                ));
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..50 {
                let _ = service_clone2.create(create_test_product(
                    &format!("Product B{i}"),
                    "20.00",
                    3,
                ));
            }
        });

        handle1.join().expect("thread 1 panicked");
        handle2.join().expect("thread 2 panicked");

        let products = service.get_all();
        assert_eq!(products.len(), 100);

        // Check that all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), 100);
    }

    #[test]
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-populate with some products
        for i in 0..10 {
            let _ = service.create(create_test_product(&format!("Initial {i}"), "10.00", 5));
        }

        let service_clone1 = service.clone();
        let service_clone2 = service.clone();
        let service_clone3 = service.clone();

        let writer = thread::spawn(move || {
            for i in 0..20 {
                let _ = service_clone1.create(create_test_product(&format!("New {i}"), "15.00", 3));
            }
        });

        let reader1 = thread::spawn(move || {
            for _ in 0..30 {
                let _ = service_clone2.get_all();
            }
        });

        let reader2 = thread::spawn(move || {
            for i in 1..=10 {
                let _ = service_clone3.get_by_id(i);
            }
        });

        writer.join().expect("writer panicked");
        reader1.join().expect("reader1 panicked");
        reader2.join().expect("reader2 panicked");

        let final_products = service.get_all();
        assert_eq!(final_products.len(), 30);
    }

    #[test]
    fn test_create_sanitizes_and_clamps() {
        let service = ProductService::new();

        let new_product = NewProduct {
            name: "  Trimmed Name  ".to_string(),
            description: "  Some description  ".to_string(),
            price: Decimal::from_str("-5.00").unwrap(),
            inventory_count: -10,
        };

        let p = service.create(new_product);
        assert_eq!(p.name, "Trimmed Name");
        assert_eq!(p.description, "Some description");
        assert_eq!(p.price, Decimal::from_str("0").unwrap());
        assert_eq!(p.inventory_count, 0);
    }

    #[test]
    fn test_update_inventory_clamps_negative_to_zero() {
        let service = ProductService::new();
        let p = service.create(create_test_product("Item", "10.00", 5));

        let updated = service.update_inventory(p.id, -3).unwrap();
        assert_eq!(updated.inventory_count, 0);
        let fetched = service.get_by_id(p.id).unwrap();
        assert_eq!(fetched.inventory_count, 0);
    }
}
