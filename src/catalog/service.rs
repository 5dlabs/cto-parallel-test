//! Product service implementation providing business logic for product management.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product service for managing products in-memory.
///
/// This service provides CRUD operations, inventory management, and filtering
/// capabilities with thread-safe concurrent access using `Arc<Mutex>`.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe storage for products
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe counter for auto-incrementing product IDs
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    ///
    /// let service = ProductService::new();
    /// assert_eq!(service.get_all().len(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and assigns it a unique auto-incrementing ID.
    ///
    /// This method is thread-safe and can be called concurrently from multiple threads.
    /// IDs start at 1 and increment sequentially.
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product data to create (without an ID)
    ///
    /// # Returns
    ///
    /// The created product with its assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Laptop".to_string(),
    ///     description: "High-end laptop".to_string(),
    ///     price: dec!(999.99),
    ///     inventory_count: 10,
    /// };
    ///
    /// let product = service.create(new_product);
    /// assert_eq!(product.id, 1);
    /// assert_eq!(product.name, "Laptop");
    /// ```
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

    /// Retrieves all products from the catalog.
    ///
    /// Returns a cloned vector of all products, ensuring thread safety
    /// by not exposing the internal storage directly.
    ///
    /// # Returns
    ///
    /// A vector containing all products
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Mouse".to_string(),
    ///     description: "Wireless mouse".to_string(),
    ///     price: dec!(29.99),
    ///     inventory_count: 50,
    /// };
    /// let _ = service.create(new_product);
    ///
    /// let products = service.get_all();
    /// assert_eq!(products.len(), 1);
    /// ```
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product
    ///
    /// # Returns
    ///
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Keyboard".to_string(),
    ///     description: "Mechanical keyboard".to_string(),
    ///     price: dec!(149.99),
    ///     inventory_count: 20,
    /// };
    /// let product = service.create(new_product);
    ///
    /// let found = service.get_by_id(product.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().name, "Keyboard");
    ///
    /// let not_found = service.get_by_id(999);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product
    /// * `new_count` - The new inventory count (can be negative)
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` if product doesn't exist
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Monitor".to_string(),
    ///     description: "4K monitor".to_string(),
    ///     price: dec!(399.99),
    ///     inventory_count: 15,
    /// };
    /// let product = service.create(new_product);
    ///
    /// let updated = service.update_inventory(product.id, 10);
    /// assert!(updated.is_some());
    /// assert_eq!(updated.unwrap().inventory_count, 10);
    ///
    /// let not_found = service.update_inventory(999, 5);
    /// assert!(not_found.is_none());
    /// ```
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

    /// Filters products based on the provided criteria.
    ///
    /// All filter criteria are combined with AND logic. An empty filter returns all products.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filtering criteria
    ///
    /// # Returns
    ///
    /// A vector of products matching all specified criteria
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct, ProductFilter};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    ///
    /// let _ = service.create(NewProduct {
    ///     name: "Laptop Pro".to_string(),
    ///     description: "High-end laptop".to_string(),
    ///     price: dec!(1299.99),
    ///     inventory_count: 5,
    /// });
    ///
    /// let _ = service.create(NewProduct {
    ///     name: "Laptop Basic".to_string(),
    ///     description: "Budget laptop".to_string(),
    ///     price: dec!(499.99),
    ///     inventory_count: 0,
    /// });
    ///
    /// // Filter by name (case-insensitive substring)
    /// let filter = ProductFilter::with_name("laptop");
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 2);
    ///
    /// // Filter by price range
    /// let filter = ProductFilter::with_price_range(Some(dec!(1000.00)), None);
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].name, "Laptop Pro");
    ///
    /// // Filter by stock status
    /// let filter = ProductFilter::with_stock_status(true);
    /// let results = service.filter(&filter);
    /// assert_eq!(results.len(), 1);
    /// assert_eq!(results[0].inventory_count, 5);
    /// ```
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

                // Minimum price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Maximum price filter
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

    /// Deletes a product by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product to delete
    ///
    /// # Returns
    ///
    /// `true` if the product was deleted, `false` if it wasn't found
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This can only happen if another
    /// thread panicked while holding the lock, which indicates a critical failure
    /// in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, NewProduct};
    /// use rust_decimal_macros::dec;
    ///
    /// let service = ProductService::new();
    /// let new_product = NewProduct {
    ///     name: "Headphones".to_string(),
    ///     description: "Noise-cancelling headphones".to_string(),
    ///     price: dec!(199.99),
    ///     inventory_count: 25,
    /// };
    /// let product = service.create(new_product);
    ///
    /// assert!(service.delete(product.id));
    /// assert!(service.get_by_id(product.id).is_none());
    /// assert!(!service.delete(product.id)); // Already deleted
    /// ```
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
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 10,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test product".to_string(),
            price: dec!(15.00),
            inventory_count: 3,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let result = service.get_by_id(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 100,
        });

        let updated = service.update_inventory(product.id, 50);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 50);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 50);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end".to_string(),
            price: dec!(1000.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Desktop PC".to_string(),
            description: "Gaming PC".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let filter = ProductFilter::with_name("laptop");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Pro");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "LAPTOP".to_string(),
            description: "Test".to_string(),
            price: dec!(100.00),
            inventory_count: 1,
        });

        let filter = ProductFilter::with_name("laptop");
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_min_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 5,
        });

        let filter = ProductFilter::with_price_range(Some(dec!(100.00)), None);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Expensive");
    }

    #[test]
    fn test_filter_by_max_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 5,
        });

        let filter = ProductFilter::with_price_range(None, Some(dec!(100.00)));
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Cheap");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Low".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mid".to_string(),
            description: "Test".to_string(),
            price: dec!(50.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "High".to_string(),
            description: "Test".to_string(),
            price: dec!(100.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::with_price_range(Some(dec!(20.00)), Some(dec!(80.00)));
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid");
    }

    #[test]
    fn test_filter_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Test".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::with_stock_status(true);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Available");
    }

    #[test]
    fn test_filter_out_of_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Test".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter = ProductFilter::with_stock_status(false);
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end".to_string(),
            price: dec!(1200.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Budget".to_string(),
            price: dec!(500.00),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Desktop".to_string(),
            description: "Gaming".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(1000.00)),
            max_price: None,
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Pro");
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test".to_string(),
            price: dec!(20.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_delete_nonexistent() {
        let service = ProductService::new();
        assert!(!service.delete(999));
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Thread1-{i}"),
                    description: "Test".to_string(),
                    price: dec!(10.00),
                    inventory_count: 1,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Thread2-{i}"),
                    description: "Test".to_string(),
                    price: dec!(20.00),
                    inventory_count: 2,
                });
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");

        let products = service.get_all();
        assert_eq!(products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, -3);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -3);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise".to_string(),
            description: "Test".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(19.999));

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().price, dec!(19.999));
    }
}
