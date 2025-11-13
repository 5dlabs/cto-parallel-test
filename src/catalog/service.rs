use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
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

    /// Creates a new product and returns it with an assigned ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
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

    /// Returns all products in the catalog
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Failed to lock products");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
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

    /// Filters products based on the provided criteria
    ///
    /// All filter criteria are combined with AND logic.
    /// Empty/None criteria are ignored.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
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

                // Min price filter
                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                // Max price filter
                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                // In stock filter
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
    /// # Returns
    /// `true` if the product was found and deleted, `false` otherwise
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (should not happen in normal operation)
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
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::thread;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_increment_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 3,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _p2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 3,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, "Product 1");
        assert_eq!(products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::from_str("15.50").unwrap(),
            inventory_count: 7,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), created);

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 5);

        let not_found = service.update_inventory(999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Laptop Computer".to_string(),
            description: "High-end laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _p2 = service.create(NewProduct {
            name: "Desktop Computer".to_string(),
            description: "Gaming desktop".to_string(),
            price: Decimal::from_str("1499.99").unwrap(),
            inventory_count: 3,
        });

        let _p3 = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 20,
        });

        let filter = ProductFilter::new().with_name("computer".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Laptop Computer"));
        assert!(results.iter().any(|p| p.name == "Desktop Computer"));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low cost".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 50,
        });

        let _p2 = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium cost".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 25,
        });

        let _p3 = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High cost".to_string(),
            price: Decimal::from_str("100.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_min_price(Decimal::from_str("20.00").unwrap())
            .with_max_price(Decimal::from_str("75.00").unwrap());

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "In Stock Item".to_string(),
            description: "Available".to_string(),
            price: Decimal::from_str("30.00").unwrap(),
            inventory_count: 5,
        });

        let _p2 = service.create(NewProduct {
            name: "Out of Stock Item".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("40.00").unwrap(),
            inventory_count: 0,
        });

        let in_stock_filter = ProductFilter::new().with_in_stock(true);
        let in_stock_results = service.filter(&in_stock_filter);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock Item");

        let out_of_stock_filter = ProductFilter::new().with_in_stock(false);
        let out_of_stock_results = service.filter(&out_of_stock_filter);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock Item");
    }

    #[test]
    fn test_combined_filters() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming".to_string(),
            price: Decimal::from_str("1299.99").unwrap(),
            inventory_count: 3,
        });

        let _p2 = service.create(NewProduct {
            name: "Gaming Mouse".to_string(),
            description: "RGB mouse".to_string(),
            price: Decimal::from_str("59.99").unwrap(),
            inventory_count: 0,
        });

        let _p3 = service.create(NewProduct {
            name: "Gaming Keyboard".to_string(),
            description: "Mechanical keyboard".to_string(),
            price: Decimal::from_str("89.99").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter::new()
            .with_name("gaming".to_string())
            .with_max_price(Decimal::from_str("100.00").unwrap())
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gaming Keyboard");
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _p1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Desc 1".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _p2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Desc 2".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 3,
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
            description: "Will be deleted".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 5,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Already deleted
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Price".to_string(),
            description: "Testing decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 1,
        });

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, Decimal::from_str("19.99").unwrap());
        assert_eq!(retrieved.price.to_string(), "19.99");
    }

    #[test]
    fn test_concurrent_access() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: i,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 10..20 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("20.00").unwrap(),
                    inventory_count: i,
                });
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = all_products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        // Update to negative inventory (e.g., for backorders)
        let updated = service.update_inventory(product.id, -3);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -3);
    }
}
