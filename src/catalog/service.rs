//! Product catalog service
//!
//! Provides thread-safe in-memory product management with CRUD operations and filtering.

use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product service with in-memory storage
///
/// This service manages products using a thread-safe in-memory vector.
/// It's designed to be shared across multiple threads in a web server context.
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
///     name: "Laptop".to_string(),
///     description: "High-performance laptop".to_string(),
///     price: Decimal::new(99999, 2), // $999.99
///     inventory_count: 10,
/// });
///
/// assert_eq!(product.id, 1);
/// assert_eq!(product.name, "Laptop");
/// ```
pub struct ProductService {
    /// Thread-safe storage for products
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe counter for generating unique IDs
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new `ProductService` instance
    ///
    /// Initializes with an empty product list and ID counter starting at 1.
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

    /// Creates a new product and adds it to the catalog
    ///
    /// Assigns a unique sequential ID to the product and stores it.
    ///
    /// # Arguments
    ///
    /// * `new_product` - Product data without ID
    ///
    /// # Returns
    ///
    /// The created product with assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (should only happen if another thread panicked while holding the lock)
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Mouse".to_string(),
    ///     description: "Wireless mouse".to_string(),
    ///     price: Decimal::new(2999, 2),
    ///     inventory_count: 50,
    /// });
    ///
    /// assert_eq!(product.id, 1);
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

    /// Retrieves all products from the catalog
    ///
    /// # Returns
    ///
    /// A cloned vector of all products
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Keyboard".to_string(),
    ///     description: "Mechanical keyboard".to_string(),
    ///     price: Decimal::new(8999, 2),
    ///     inventory_count: 20,
    /// });
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

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Monitor".to_string(),
    ///     description: "4K monitor".to_string(),
    ///     price: Decimal::new(39999, 2),
    ///     inventory_count: 15,
    /// });
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

    /// Filters products based on multiple criteria
    ///
    /// All filter criteria are optional. `None` values are treated as "no filter".
    /// Multiple filters are combined with AND logic.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter criteria to apply
    ///
    /// # Returns
    ///
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
    ///     price: Decimal::new(150, 2),
    ///     inventory_count: 100,
    /// });
    ///
    /// service.create(NewProduct {
    ///     name: "Banana".to_string(),
    ///     description: "Ripe banana".to_string(),
    ///     price: Decimal::new(75, 2),
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
    /// ```
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

                // Minimum price filter: price >= min_price
                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                // Maximum price filter: price <= max_price
                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

                // Stock status filter: (inventory_count > 0) == in_stock
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
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);

        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());

        let all = service.get_all();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_inventory_update() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        let not_found = service.update_inventory(9999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_product_filtering_by_name() {
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

        // Test case-insensitive name filter
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test name filter with uppercase
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("BANANA".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Banana");
    }

    #[test]
    fn test_product_filtering_by_price() {
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
            price: Decimal::new(300, 2),
            inventory_count: 2,
        });

        // Test min_price filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Test max_price filter
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(180, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(200, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Medium");
    }

    #[test]
    fn test_product_filtering_by_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 0,
        });

        // Test in_stock = true
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "In Stock");

        // Test in_stock = false
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Out of Stock");
    }

    #[test]
    fn test_product_filtering_combined() {
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
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 5,
        });

        // Test combined filters: name contains "a" and in stock
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test combined filters: price range and in stock
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(180, 2)),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Desc 1".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 0,
        });

        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn multiple threads creating products concurrently
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(100 * i64::from(i), 2),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }
}
