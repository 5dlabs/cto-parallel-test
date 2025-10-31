use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service
///
/// This service provides CRUD operations for products with automatic ID assignment,
/// inventory management, and advanced filtering capabilities.
///
/// # Thread Safety
/// Uses Arc<Mutex<>> to allow safe concurrent access across multiple threads.
/// Separate locks for products and ID counter minimize contention.
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
    ///
    /// # Examples
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    /// let service = ProductService::new();
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
    /// # Arguments
    /// * `new_product` - Product details without ID
    ///
    /// # Returns
    /// The created product with assigned ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Product lock poisoned");
        let mut next_id = self.next_id.lock().expect("ID lock poisoned");

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
    /// Vector of all products
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Product lock poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// Some(Product) if found, None otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Product lock poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// Some(Product) with updated inventory if found, None otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Product lock poisoned");

        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on multiple criteria
    ///
    /// All filter criteria are combined with AND logic.
    /// None values are treated as "no filter" for that criterion.
    ///
    /// # Arguments
    /// * `filter` - Filter criteria with optional fields
    ///
    /// # Returns
    /// Vector of products matching all specified criteria
    ///
    /// # Filter Criteria
    /// * `name_contains` - Case-insensitive substring match
    /// * `min_price` - Products with price >= `min_price`
    /// * `max_price` - Products with price <= `max_price`
    /// * `in_stock` - true for inventory > 0, false for inventory == 0
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Product lock poisoned");

        products
            .iter()
            .filter(|p| {
                // Name filter - case-insensitive substring match
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Min price filter - inclusive
                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                // Max price filter - inclusive
                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

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
            description: "A test product".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Test Product");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::new(2999, 2), // $29.99
            inventory_count: 5,
        });

        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
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
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
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
            description: "Red fruit".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow fruit".to_string(),
            price: Decimal::new(75, 2), // $0.75
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Orange fruit".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test name filter - case insensitive
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test name filter with multiple matches
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 3); // Apple, Banana, Orange all contain "a"
    }

    #[test]
    fn test_product_filtering_by_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(100, 2), // $1.00
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Medium price".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 3,
        });

        // Test min price filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(150, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Medium and Expensive

        // Test max price filter
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(150, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Cheap and Medium

        // Test price range
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(180, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Cheap and Medium
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
            price: Decimal::new(150, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Low Stock".to_string(),
            description: "Few remaining".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 1,
        });

        // Test in_stock=true filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // In Stock and Low Stock

        // Test in_stock=false filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Out of Stock
    }

    #[test]
    fn test_product_filtering_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Red fruit".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Banana".to_string(),
            description: "Yellow fruit".to_string(),
            price: Decimal::new(75, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Orange fruit".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 5,
        });

        // Test combined filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            min_price: Some(Decimal::new(100, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange (Banana is out of stock)

        // Test empty filter returns all
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn 10 threads, each creating 10 products
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let _ = service_clone.create(NewProduct {
                        name: format!("Product {i}-{j}"),
                        description: "Test".to_string(),
                        price: Decimal::new(100, 2),
                        inventory_count: 1,
                    });
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Should have 100 products with unique IDs
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 100);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 100);
    }
}
