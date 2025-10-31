//! Product service implementation
//!
//! Provides thread-safe in-memory product catalog operations including
//! CRUD operations, inventory management, and advanced filtering.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service
///
/// Uses `Arc<Mutex<>>` for safe concurrent access across multiple threads.
/// Products are stored in a vector with sequential ID assignment.
#[derive(Clone)]
pub struct ProductService {
    /// Thread-safe product storage
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe ID counter for sequential ID assignment
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
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

    /// Creates a new product and assigns it a unique ID
    ///
    /// IDs are assigned sequentially starting from 1.
    ///
    /// # Arguments
    ///
    /// * `new_product` - Product data without ID
    ///
    /// # Returns
    ///
    /// The created product with assigned ID
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Test Product".to_string(),
    ///     description: "A test product".to_string(),
    ///     price: Decimal::new(1999, 2), // $19.99
    ///     inventory_count: 10,
    /// });
    ///
    /// assert_eq!(product.id, 1);
    /// assert_eq!(product.name, "Test Product");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should not happen in normal operation).
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let id = *next_id;
        *next_id += 1;

        let product = Product {
            id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        products.push(product.clone());
        product
    }

    /// Retrieves all products in the catalog
    ///
    /// # Returns
    ///
    /// A vector containing clones of all products
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    ///
    /// let service = ProductService::new();
    /// let all_products = service.get_all();
    /// assert_eq!(all_products.len(), 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should not happen in normal operation).
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
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let created = service.create(NewProduct {
    ///     name: "Test".to_string(),
    ///     description: "Test product".to_string(),
    ///     price: Decimal::new(999, 2),
    ///     inventory_count: 5,
    /// });
    ///
    /// let found = service.get_by_id(created.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().id, created.id);
    ///
    /// let not_found = service.get_by_id(9999);
    /// assert!(not_found.is_none());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should not happen in normal operation).
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
    /// `Some(Product)` with updated inventory if product exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Test".to_string(),
    ///     description: "Test".to_string(),
    ///     price: Decimal::new(1000, 2),
    ///     inventory_count: 10,
    /// });
    ///
    /// let updated = service.update_inventory(product.id, 5);
    /// assert!(updated.is_some());
    /// assert_eq!(updated.unwrap().inventory_count, 5);
    ///
    /// // Verify the update persisted
    /// let retrieved = service.get_by_id(product.id).unwrap();
    /// assert_eq!(retrieved.inventory_count, 5);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should not happen in normal operation).
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

    /// Filters products based on multiple criteria
    ///
    /// All filter criteria are optional. When a field is `None`, it doesn't restrict results.
    /// Multiple filters combine with AND logic (all conditions must match).
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter criteria to apply
    ///
    /// # Returns
    ///
    /// A vector of products matching all specified criteria
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::{NewProduct, ProductFilter}};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    ///
    /// // Create test products
    /// let _ = service.create(NewProduct {
    ///     name: "Apple".to_string(),
    ///     description: "Fresh apple".to_string(),
    ///     price: Decimal::new(150, 2), // $1.50
    ///     inventory_count: 10,
    /// });
    ///
    /// let _ = service.create(NewProduct {
    ///     name: "Banana".to_string(),
    ///     description: "Yellow banana".to_string(),
    ///     price: Decimal::new(75, 2), // $0.75
    ///     inventory_count: 0,
    /// });
    ///
    /// let _ = service.create(NewProduct {
    ///     name: "Orange".to_string(),
    ///     description: "Juicy orange".to_string(),
    ///     price: Decimal::new(200, 2), // $2.00
    ///     inventory_count: 5,
    /// });
    ///
    /// // Filter by name (case-insensitive)
    /// let filtered = service.filter(&ProductFilter {
    ///     name_contains: Some("app".to_string()),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 1);
    /// assert_eq!(filtered[0].name, "Apple");
    ///
    /// // Filter by price range
    /// let filtered = service.filter(&ProductFilter {
    ///     min_price: Some(Decimal::new(100, 2)),
    ///     max_price: Some(Decimal::new(180, 2)),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 1);
    /// assert_eq!(filtered[0].name, "Apple");
    ///
    /// // Filter by stock status
    /// let filtered = service.filter(&ProductFilter {
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 2); // Apple and Orange
    ///
    /// // Combined filters
    /// let filtered = service.filter(&ProductFilter {
    ///     name_contains: Some("a".to_string()),
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 2); // Apple and Orange (both contain 'a' and in stock)
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should not happen in normal operation).
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                // Min price filter: inclusive (>=)
                let min_price_match = filter
                    .min_price
                    .as_ref()
                    .is_none_or(|min| p.price >= *min);

                // Max price filter: inclusive (<=)
                let max_price_match = filter
                    .max_price
                    .as_ref()
                    .is_none_or(|max| p.price <= *max);

                // Stock status filter
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // All filters must match (AND logic)
                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
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

    #[test]
    fn test_product_creation() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Test Product");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        assert_eq!(product2.id, 2);
        assert_eq!(product2.name, "Product 2");
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test product".to_string(),
            price: Decimal::new(999, 2),
            inventory_count: 5,
        });

        // Test get_by_id
        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);

        // Test non-existent ID
        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());

        // Test get_all
        let all = service.get_all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, created.id);
    }

    #[test]
    fn test_inventory_update() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        // Update inventory
        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        // Verify update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        // Test updating non-existent product
        let not_updated = service.update_inventory(9999, 10);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_product_filtering() {
        let service = ProductService::new();

        // Create test products
        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow banana".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test name filter (case-insensitive)
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // $1.00
            max_price: Some(Decimal::new(180, 2)), // $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test in_stock filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test out_of_stock filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Banana");

        // Test combined filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange contain "a" and in stock

        // Test empty filter returns all
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = ProductService::new();

        // Create products from multiple threads
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let service_clone = service.clone();
                thread::spawn(move || {
                    service_clone.create(NewProduct {
                        name: format!("Product {i}"),
                        description: format!("Description {i}"),
                        price: Decimal::new(100 * i64::from(i), 2),
                        inventory_count: i,
                    })
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify IDs are unique and sequential
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        assert_eq!(ids, (1..=10).collect::<Vec<i32>>());
    }
}
