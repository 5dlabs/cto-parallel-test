use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service
/// Provides CRUD operations and advanced filtering capabilities
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
    /// * `new_product` - Product details without ID
    ///
    /// # Returns
    /// The created product with assigned ID
    ///
    /// # Panics
    /// Panics if unable to acquire locks (should not happen in normal usage)
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

    /// Returns all products in the catalog
    ///
    /// # Returns
    /// Vector of all products (cloned)
    ///
    /// # Panics
    /// Panics if unable to acquire lock (should not happen in normal usage)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Arguments
    /// * `id` - Product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if unable to acquire lock (should not happen in normal usage)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    /// * `id` - Product ID to update
    /// * `new_count` - New inventory count
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if unable to acquire lock (should not happen in normal usage)
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
    /// All criteria are combined with AND logic
    ///
    /// # Arguments
    /// * `filter` - Filter criteria (all fields optional)
    ///
    /// # Returns
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    /// Panics if unable to acquire lock (should not happen in normal usage)
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

                // In stock filter: inventory_count > 0
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                // Combine all filters with AND
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
            description: "Test Description".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product1.name, "Test Product");
        assert_eq!(product1.price, Decimal::new(1999, 2));
        assert_eq!(product1.inventory_count, 10);

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
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
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        // Test get_by_id
        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.name, created.name);

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
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        // Update inventory
        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.inventory_count, 5);

        // Verify update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        // Test non-existent product
        let not_updated = service.update_inventory(9999, 5);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_product_filtering_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple".to_string(),
            description: "Fresh apple".to_string(),
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

        // Test multiple matches
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 3); // Apple, Banana, Orange all contain 'a'
    }

    #[test]
    fn test_product_filtering_price() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(50, 2), // $0.50
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Medium price".to_string(),
            price: Decimal::new(150, 2), // $1.50
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(300, 2), // $3.00
            inventory_count: 3,
        });

        // Test min_price filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // >= $1.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Medium and Expensive

        // Test max_price filter
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(200, 2)), // <= $2.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Cheap and Medium

        // Test price range
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)),
            max_price: Some(Decimal::new(200, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Only Medium
        assert_eq!(filtered[0].name, "Medium Item");
    }

    #[test]
    fn test_product_filtering_stock() {
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

        let _ = service.create(NewProduct {
            name: "Also In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 5,
        });

        // Test in_stock = true
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

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
            name: "Apple Juice".to_string(),
            description: "Fresh juice".to_string(),
            price: Decimal::new(250, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple Pie".to_string(),
            description: "Delicious pie".to_string(),
            price: Decimal::new(150, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Orange Juice".to_string(),
            description: "Citrus juice".to_string(),
            price: Decimal::new(300, 2),
            inventory_count: 5,
        });

        // Test combined filters: name + stock + price
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("juice".to_string()),
            in_stock: Some(true),
            max_price: Some(Decimal::new(280, 2)),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple Juice");
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Desc 1".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: Decimal::new(200, 2),
            inventory_count: 5,
        });

        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Create products from multiple threads
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

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify all products were created
        let all = service.get_all();
        assert_eq!(all.len(), 10);

        // Verify IDs are sequential and unique
        let mut ids: Vec<i32> = all.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
