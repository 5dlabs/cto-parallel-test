use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service.
///
/// This service provides CRUD operations and filtering capabilities for products.
/// It uses Arc<Mutex<>> for thread safety, making it safe to share across
/// multiple threads (e.g., web server worker threads).
pub struct ProductService {
    /// Thread-safe storage for products.
    products: Arc<Mutex<Vec<Product>>>,
    /// Thread-safe ID counter for assigning unique IDs to new products.
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service.
    ///
    /// The service starts with no products and ID counter initialized to 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and adds it to the catalog.
    ///
    /// # Arguments
    /// * `new_product` - The product data without an ID
    ///
    /// # Returns
    /// The created product with an assigned unique ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (which should not happen in normal operation)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        // Lock both mutexes to ensure atomicity
        let mut products = self.products.lock().expect("Failed to lock products");
        let mut next_id = self.next_id.lock().expect("Failed to lock next_id");

        // Assign current ID and increment for next product
        let id = *next_id;
        *next_id += 1;

        // Create the product with assigned ID
        let product = Product {
            id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        // Add to catalog
        products.push(product.clone());

        product
    }

    /// Retrieves all products in the catalog.
    ///
    /// # Returns
    /// A vector of all products (cloned, so safe to use after lock is released)
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the product to find
    ///
    /// # Returns
    /// Some(Product) if found, None otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    /// * `id` - The ID of the product to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// Some(Product) with updated inventory if product found, None otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Failed to lock products");

        // Find the product and update its inventory
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on provided criteria.
    ///
    /// All filter criteria are optional. If a criterion is None, it's ignored.
    /// Multiple criteria are combined with AND logic - all must match.
    ///
    /// # Arguments
    /// * `filter` - The filter criteria
    ///
    /// # Returns
    /// A vector of products matching all filter criteria
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
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

                // Minimum price filter: inclusive
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Maximum price filter: inclusive
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // In-stock filter: inventory > 0 means in stock
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
            description: "Test Description".to_string(),
            price: Decimal::new(1000, 2), // $10.00
            inventory_count: 10,
        });

        // Test get_by_id with existing product
        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.as_ref().unwrap().id, created.id);
        assert_eq!(found.as_ref().unwrap().name, created.name);

        // Test get_by_id with non-existent product
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
            description: "Test Description".to_string(),
            price: Decimal::new(1000, 2), // $10.00
            inventory_count: 10,
        });

        // Update inventory
        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.as_ref().unwrap().inventory_count, 5);

        // Verify the update persisted
        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        // Test update for non-existent product
        let not_updated = service.update_inventory(9999, 10);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_product_filtering_empty_filter() {
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

        // Empty filter should return all products
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_product_filtering_by_name() {
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
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test case-insensitive name filter
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test name filter that matches multiple products
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("an".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Banana and Orange
    }

    #[test]
    fn test_product_filtering_by_price() {
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
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Orange".to_string(),
            description: "Juicy orange".to_string(),
            price: Decimal::new(200, 2), // $2.00
            inventory_count: 5,
        });

        // Test min_price filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // $1.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test max_price filter
        let filtered = service.filter(&ProductFilter {
            max_price: Some(Decimal::new(180, 2)), // $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Banana

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // $1.00
            max_price: Some(Decimal::new(180, 2)), // $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Only Apple
        assert_eq!(filtered[0].name, "Apple");
    }

    #[test]
    fn test_product_filtering_by_stock() {
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

        // Test in_stock = true filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange
        assert!(filtered.iter().all(|p| p.inventory_count > 0));

        // Test in_stock = false filter
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Only Banana
        assert_eq!(filtered[0].name, "Banana");
        assert_eq!(filtered[0].inventory_count, 0);
    }

    #[test]
    fn test_product_filtering_combined() {
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
            name: "Grape".to_string(),
            description: "Sweet grape".to_string(),
            price: Decimal::new(300, 2), // $3.00
            inventory_count: 8,
        });

        // Test combined filters: name contains "a", in stock, price <= $2.00
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            max_price: Some(Decimal::new(200, 2)), // $2.00
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange (Banana is out of stock, Grape is too expensive)

        let names: Vec<&str> = filtered.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"Apple"));
        assert!(names.contains(&"Orange"));
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        // Spawn multiple threads that create products concurrently
        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::new(1000 + i64::from(i), 2),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify all products were created
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);

        // Verify IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
