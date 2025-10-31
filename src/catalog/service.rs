//! Product catalog service
//!
//! This module provides the business logic for product catalog management,
//! including CRUD operations, inventory management, and filtering capabilities.
//! The service is thread-safe and can be shared across multiple threads.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service
///
/// This service provides in-memory storage and management of products.
/// It uses `Arc<Mutex<>>` for thread-safe shared mutable access, making it
/// suitable for use in multi-threaded web servers.
///
/// # Examples
///
/// ```
/// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
/// use rust_decimal::Decimal;
///
/// let service = ProductService::new();
///
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
#[derive(Debug, Clone)]
pub struct ProductService {
    /// Storage for all products
    products: Arc<Mutex<Vec<Product>>>,
    /// Counter for generating unique product IDs
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new `ProductService` instance
    ///
    /// Initializes an empty product catalog with the ID counter starting at 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    ///
    /// let service = ProductService::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product in the catalog
    ///
    /// Assigns a unique ID to the product and stores it in the catalog.
    /// The ID is auto-incremented starting from 1.
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product data without an ID
    ///
    /// # Returns
    ///
    /// The created product with its assigned ID
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Widget".to_string(),
    ///     description: "A useful widget".to_string(),
    ///     price: Decimal::new(999, 2), // $9.99
    ///     inventory_count: 100,
    /// });
    ///
    /// assert_eq!(product.id, 1);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This should only happen if another
    /// thread panicked while holding the lock.
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Failed to lock products");
        let mut next_id = self.next_id.lock().expect("Failed to lock next_id");

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
    /// Returns a cloned vector of all products. The clone is necessary to avoid
    /// holding the lock while the caller processes the results.
    ///
    /// # Returns
    ///
    /// A vector containing all products in the catalog
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// service.create(NewProduct {
    ///     name: "Product 1".to_string(),
    ///     description: "First product".to_string(),
    ///     price: Decimal::new(1000, 2),
    ///     inventory_count: 5,
    /// });
    ///
    /// let all_products = service.get_all();
    /// assert_eq!(all_products.len(), 1);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This should only happen if another
    /// thread panicked while holding the lock.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product
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
    ///     name: "Widget".to_string(),
    ///     description: "A widget".to_string(),
    ///     price: Decimal::new(500, 2),
    ///     inventory_count: 10,
    /// });
    ///
    /// let found = service.get_by_id(created.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().name, "Widget");
    ///
    /// let not_found = service.get_by_id(9999);
    /// assert!(not_found.is_none());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This should only happen if another
    /// thread panicked while holding the lock.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    ///
    /// `Some(Product)` with the updated inventory if the product exists,
    /// `None` if the product is not found
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Widget".to_string(),
    ///     description: "A widget".to_string(),
    ///     price: Decimal::new(500, 2),
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
    /// Panics if the internal mutex is poisoned. This should only happen if another
    /// thread panicked while holding the lock.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Failed to lock products");

        products.iter_mut().find(|p| p.id == id).map(|product| {
            product.inventory_count = new_count;
            product.clone()
        })
    }

    /// Filters products based on specified criteria
    ///
    /// All filter criteria are optional (represented by `Option`). When a criterion
    /// is `None`, it is ignored. Multiple criteria are combined with AND logic.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    ///
    /// A vector of products matching all specified criteria
    ///
    /// # Filter Criteria
    ///
    /// * `name_contains` - Case-insensitive substring match on product name
    /// * `min_price` - Products with price >= `min_price` (inclusive)
    /// * `max_price` - Products with price <= `max_price` (inclusive)
    /// * `in_stock` - Products with inventory > 0 (if true) or inventory == 0 (if false)
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::{NewProduct, ProductFilter}};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    ///
    /// service.create(NewProduct {
    ///     name: "Apple".to_string(),
    ///     description: "Fresh apple".to_string(),
    ///     price: Decimal::new(150, 2), // $1.50
    ///     inventory_count: 10,
    /// });
    ///
    /// service.create(NewProduct {
    ///     name: "Banana".to_string(),
    ///     description: "Yellow banana".to_string(),
    ///     price: Decimal::new(75, 2), // $0.75
    ///     inventory_count: 0,
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
    /// // Filter by in-stock status
    /// let in_stock = service.filter(&ProductFilter {
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(in_stock.len(), 1);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned. This should only happen if another
    /// thread panicked while holding the lock.
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                // Min price filter: price >= min_price
                let min_price_match = filter
                    .min_price
                    .as_ref()
                    .is_none_or(|min| p.price >= *min);

                // Max price filter: price <= max_price
                let max_price_match = filter
                    .max_price
                    .as_ref()
                    .is_none_or(|max| p.price <= *max);

                // In stock filter: (inventory > 0) == in_stock
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
    fn test_new_service_is_empty() {
        let service = ProductService::new();
        assert_eq!(service.get_all().len(), 0);
    }

    #[test]
    fn test_product_creation_assigns_sequential_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Product 1");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        assert_eq!(product2.id, 2);
        assert_eq!(product2.name, "Product 2");
    }

    #[test]
    fn test_get_all_returns_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 20,
        });

        let all = service.get_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_get_by_id_returns_correct_product() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test description".to_string(),
            price: Decimal::new(1500, 2),
            inventory_count: 7,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.name, "Test Product");
        assert_eq!(found.price, Decimal::new(1500, 2));
    }

    #[test]
    fn test_get_by_id_returns_none_for_nonexistent() {
        let service = ProductService::new();
        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory_modifies_product() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.inventory_count, 5);

        // Verify persistence
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);
    }

    #[test]
    fn test_update_inventory_returns_none_for_nonexistent() {
        let service = ProductService::new();
        let result = service.update_inventory(9999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 0,
        });

        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Red apple".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow banana".to_string(),
            price: Decimal::new(75, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Fresh orange".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 8,
        });

        // Test case-insensitive search
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test partial match
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("an".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Banana and Orange
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Cheap item".to_string(),
            price: Decimal::new(500, 2), // $5.00
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Medium item".to_string(),
            price: Decimal::new(1500, 2), // $15.00
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "Expensive item".to_string(),
            price: Decimal::new(5000, 2), // $50.00
            inventory_count: 2,
        });

        // Test min price
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(1000, 2)), // >= $10.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Medium and Expensive

        // Test max price
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(2000, 2)), // <= $20.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Cheap and Medium

        // Test price range
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(1000, 2)), // >= $10.00
            max_price: Some(Decimal::new(2000, 2)), // <= $20.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Only Medium
        assert_eq!(filtered[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Also In Stock".to_string(),
            description: "Available too".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        // Filter for in-stock items
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Filter for out-of-stock items
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_with_multiple_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple Juice".to_string(),
            description: "Fresh juice".to_string(),
            price: Decimal::new(350, 2), // $3.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple Pie".to_string(),
            description: "Delicious pie".to_string(),
            price: Decimal::new(1200, 2), // $12.00
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange Juice".to_string(),
            description: "Citrus juice".to_string(),
            price: Decimal::new(400, 2), // $4.00
            inventory_count: 8,
        });

        // Combine name and stock filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("apple".to_string()),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple Juice");

        // Combine name, price, and stock filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("juice".to_string()),
            min_price: Some(Decimal::new(300, 2)), // >= $3.00
            max_price: Some(Decimal::new(500, 2)), // <= $5.00
            in_stock: Some(true),
        });
        assert_eq!(filtered.len(), 2); // Both juices match all criteria
    }

    #[test]
    fn test_service_is_clonable() {
        let service1 = ProductService::new();
        let _ = service1.create(NewProduct {
            name: "Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        // Clone the service
        let service2 = service1.clone();

        // Both should see the same product (shared Arc)
        assert_eq!(service1.get_all().len(), 1);
        assert_eq!(service2.get_all().len(), 1);

        // Create product in service2
        let _ = service2.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 5,
        });

        // Both should see both products
        assert_eq!(service1.get_all().len(), 2);
        assert_eq!(service2.get_all().len(), 2);
    }
}
