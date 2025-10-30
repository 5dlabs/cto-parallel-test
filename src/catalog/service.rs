use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service
///
/// Provides CRUD operations and filtering for products with concurrent access support.
/// Uses Arc<Mutex<>> for thread safety across multiple web server workers.
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
    ///
    /// Initializes with an empty product list and ID counter starting at 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product in the catalog
    ///
    /// Assigns a unique sequential ID and stores the product.
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
    /// Panics if the mutex is poisoned (should not happen in normal operation)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("products mutex poisoned");
        let mut next_id = self.next_id.lock().expect("next_id mutex poisoned");

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
    /// A cloned vector of all products (safe to use after lock is released)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID to search for
    ///
    /// # Returns
    ///
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID to update
    /// * `new_count` - New inventory count
    ///
    /// # Returns
    ///
    /// `Some(Product)` with updated inventory if found, `None` if product doesn't exist
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("products mutex poisoned");

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
    ///
    /// * `filter` - Filter criteria
    ///
    /// # Returns
    ///
    /// Vector of products matching all filter criteria
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("products mutex poisoned");

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                // Minimum price filter (inclusive)
                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                // Maximum price filter (inclusive)
                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

                // In-stock filter
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
        assert_eq!(product2.name, "Product 2");
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        // Initially empty
        assert_eq!(service.get_all().len(), 0);

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 10,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0].id, 1);
        assert_eq!(all_products[1].id, 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1500, 2),
            inventory_count: 8,
        });

        // Found
        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        let found_product = found.unwrap();
        assert_eq!(found_product.id, created.id);
        assert_eq!(found_product.name, "Test Product");

        // Not found
        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        // Update existing product
        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        // Verify update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        // Update non-existent product
        let not_updated = service.update_inventory(9999, 100);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple iPhone".to_string(),
            description: "Phone".to_string(),
            price: Decimal::new(99900, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy".to_string(),
            description: "Phone".to_string(),
            price: Decimal::new(89900, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Apple iPad".to_string(),
            description: "Tablet".to_string(),
            price: Decimal::new(79900, 2),
            inventory_count: 3,
        });

        // Case-insensitive name search
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("apple".to_string()),
            ..Default::default()
        });

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|p| p.name == "Apple iPhone"));
        assert!(filtered.iter().any(|p| p.name == "Apple iPad"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(1000, 2), // $10.00
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: Decimal::new(5000, 2), // $50.00
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(10000, 2), // $100.00
            inventory_count: 2,
        });

        // Filter by minimum price
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(4000, 2)), // >= $40.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Filter by maximum price
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(6000, 2)), // <= $60.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Filter by price range
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(2000, 2)), // >= $20.00
            max_price: Some(Decimal::new(8000, 2)), // <= $80.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Medium");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Unavailable".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Also In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(3000, 2),
            inventory_count: 5,
        });

        // Filter for in-stock products
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2);

        // Filter for out-of-stock products
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Apple iPhone 15".to_string(),
            description: "Latest iPhone".to_string(),
            price: Decimal::new(99900, 2), // $999.00
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Apple iPhone 14".to_string(),
            description: "Previous iPhone".to_string(),
            price: Decimal::new(79900, 2), // $799.00
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Samsung Galaxy S23".to_string(),
            description: "Samsung phone".to_string(),
            price: Decimal::new(89900, 2), // $899.00
            inventory_count: 5,
        });

        // Combined filter: Apple + in stock + price <= $900
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("apple".to_string()),
            max_price: Some(Decimal::new(90000, 2)), // $900.00
            in_stock: Some(true),
            ..Default::default()
        });

        assert_eq!(filtered.len(), 0); // No Apple products match all criteria

        // Combined filter: in stock + price >= $800
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(80000, 2)), // $800.00
            in_stock: Some(true),
            ..Default::default()
        });

        assert_eq!(filtered.len(), 2); // iPhone 15 and Galaxy S23
    }

    #[test]
    fn test_filter_empty_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 0,
        });

        // Empty filter should return all products
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let service = Arc::new(ProductService::new());

        // Create products from multiple threads
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let service_clone = Arc::clone(&service);
                thread::spawn(move || {
                    service_clone.create(NewProduct {
                        name: format!("Product {i}"),
                        description: format!("Description {i}"),
                        price: Decimal::new(1000 + i64::from(i) * 100, 2),
                        inventory_count: i,
                    })
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify IDs are unique and sequential
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
