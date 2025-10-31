//! Product service implementation
//!
//! Provides thread-safe in-memory storage and operations for products.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service
///
/// Uses `Arc<Mutex<>>` for safe concurrent access across multiple threads.
/// Suitable for use in multi-threaded web servers like Actix-web.
pub struct ProductService {
    /// Product storage
    products: Arc<Mutex<Vec<Product>>>,
    /// Auto-incrementing ID counter
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Create a new empty product service
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

    /// Create a new product
    ///
    /// Automatically assigns a unique sequential ID starting from 1.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned (unlikely in normal usage).
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

    /// Get all products
    ///
    /// Returns a cloned vector of all products. Safe to use after the lock is released.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned (unlikely in normal usage).
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
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Get a product by ID
    ///
    /// Returns `Some(Product)` if found, `None` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned (unlikely in normal usage).
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
    ///     description: "Test".to_string(),
    ///     price: Decimal::new(1000, 2),
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
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Update the inventory count for a product
    ///
    /// Returns `Some(Product)` with the updated product if found, `None` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned (unlikely in normal usage).
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
    /// // Verify update persisted
    /// let retrieved = service.get_by_id(product.id).unwrap();
    /// assert_eq!(retrieved.inventory_count, 5);
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

    /// Filter products by multiple criteria
    ///
    /// All filter fields are optional. When `None`, the field doesn't restrict results.
    /// All filters are combined with AND logic.
    ///
    /// # Panics
    ///
    /// Panics if the mutex lock is poisoned (unlikely in normal usage).
    ///
    /// # Filter Criteria
    ///
    /// - `name_contains`: Case-insensitive substring match
    /// - `min_price`: Price >= `min_price` (inclusive)
    /// - `max_price`: Price <= `max_price` (inclusive)
    /// - `in_stock`: If true, `inventory_count` > 0; if false, `inventory_count` == 0
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
    ///     description: "Ripe banana".to_string(),
    ///     price: Decimal::new(75, 2), // $0.75
    ///     inventory_count: 0,
    /// });
    ///
    /// // Filter by name
    /// let filtered = service.filter(&ProductFilter {
    ///     name_contains: Some("app".to_string()),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 1);
    /// assert_eq!(filtered[0].name, "Apple");
    ///
    /// // Filter by stock status
    /// let in_stock = service.filter(&ProductFilter {
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(in_stock.len(), 1);
    /// assert_eq!(in_stock[0].name, "Apple");
    /// ```
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

                // In stock filter: inventory_count > 0 matches true
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // Combine all filters with AND logic
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
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 3,
        });

        let all = service.get_all();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "Product 1");
        assert_eq!(all[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id_finds_existing_product() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test description".to_string(),
            price: Decimal::new(1500, 2),
            inventory_count: 8,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        let found_product = found.unwrap();
        assert_eq!(found_product.id, created.id);
        assert_eq!(found_product.name, "Test Product");
    }

    #[test]
    fn test_get_by_id_returns_none_for_nonexistent() {
        let service = ProductService::new();

        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory_modifies_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        // Verify the update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);
    }

    #[test]
    fn test_update_inventory_returns_none_for_nonexistent() {
        let service = ProductService::new();

        let updated = service.update_inventory(9999, 100);
        assert!(updated.is_none());
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Test".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 3,
        });

        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Ripe banana".to_string(),
            price: Decimal::new(75, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 8,
        });

        // Test case-insensitive substring match
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test another substring
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("an".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Banana and Orange
    }

    #[test]
    fn test_filter_by_min_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(50, 2), // $0.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(500, 2), // $5.00
            inventory_count: 3,
        });

        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // >= $1.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Medium and Expensive
    }

    #[test]
    fn test_filter_by_max_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(50, 2), // $0.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(500, 2), // $5.00
            inventory_count: 3,
        });

        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(180, 2)), // <= $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Cheap and Medium
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(50, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(500, 2),
            inventory_count: 3,
        });

        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(200, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Only Medium
        assert_eq!(filtered[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_in_stock_true() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available".to_string(),
            description: "In stock".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out".to_string(),
            description: "Out of stock".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Available2".to_string(),
            description: "Also in stock".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 5,
        });

        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Available and Available2
    }

    #[test]
    fn test_filter_by_in_stock_false() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available".to_string(),
            description: "In stock".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out".to_string(),
            description: "Out of stock".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 0,
        });

        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Out");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Ripe banana".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Pineapple".to_string(),
            description: "Sweet pineapple".to_string(),
            price: Decimal::new(300, 2), // $3.00
            inventory_count: 8,
        });

        // Filter: name contains "a", in stock, price between $1.00 and $2.50
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(250, 2)),
            in_stock: Some(true),
        });

        // Should match: Apple (has 'a', $1.50, in stock) and Orange (has 'a', $2.00, in stock)
        // Should NOT match: Banana (out of stock), Pineapple (price > $2.50)
        assert_eq!(filtered.len(), 2);
        let names: Vec<&str> = filtered.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"Apple"));
        assert!(names.contains(&"Orange"));
    }

    #[test]
    fn test_service_is_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<ProductService>();
        assert_sync::<ProductService>();
    }
}
