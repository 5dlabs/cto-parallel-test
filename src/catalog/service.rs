use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Create a new empty product service
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new product and assign it a unique ID
    ///
    /// # Arguments
    /// * `new_product` - The product data without an ID
    ///
    /// # Returns
    /// The created product with an assigned ID
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

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

    /// Get all products in the catalog
    ///
    /// # Returns
    /// A vector of all products
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Get a product by its ID
    ///
    /// # Arguments
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    /// `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Update the inventory count for a product
    ///
    /// # Arguments
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    /// `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
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

    /// Filter products based on various criteria
    ///
    /// # Arguments
    /// * `filter` - The filter criteria to apply
    ///
    /// # Returns
    /// A vector of products matching all specified criteria
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products
            .iter()
            .filter(|p| {
                let name_match = filter
                    .name_contains
                    .as_ref()
                    .is_none_or(|name| p.name.to_lowercase().contains(&name.to_lowercase()));

                let min_price_match = filter.min_price.is_none_or(|min| p.price >= min);

                let max_price_match = filter.max_price.is_none_or(|max| p.price <= max);

                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

                name_match && min_price_match && max_price_match && in_stock_match
            })
            .cloned()
            .collect()
    }

    /// Delete a product by its ID
    ///
    /// # Arguments
    /// * `id` - The product ID to delete
    ///
    /// # Returns
    /// `true` if the product was found and deleted, `false` otherwise
    ///
    /// # Panics
    /// Panics if the internal mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().unwrap();
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
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
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
            inventory_count: 3,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(15.00),
            inventory_count: 7,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Product");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(25.00),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 15);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 15);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 15);

        let not_updated = service.update_inventory(999, 20);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Computer".to_string(),
            description: "High-end laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Desktop Computer".to_string(),
            description: "Powerful desktop".to_string(),
            price: dec!(1299.99),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 20,
        });

        let filter = ProductFilter {
            name_contains: Some("computer".to_string()),
            ..ProductFilter::new()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|p| p.name.to_lowercase().contains("computer")));
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: dec!(5.00),
            inventory_count: 100,
        });

        let _ = service.create(NewProduct {
            name: "Mid Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 50,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            min_price: Some(dec!(20.00)),
            max_price: Some(dec!(100.00)),
            ..ProductFilter::new()
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_stock_status() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let in_stock_filter = ProductFilter {
            in_stock: Some(true),
            ..ProductFilter::new()
        };

        let in_stock_results = service.filter(&in_stock_filter);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock");

        let out_of_stock_filter = ProductFilter {
            in_stock: Some(false),
            ..ProductFilter::new()
        };

        let out_of_stock_results = service.filter(&out_of_stock_filter);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock");
    }

    #[test]
    fn test_combined_filters() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High-end gaming".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Business Laptop".to_string(),
            description: "Professional use".to_string(),
            price: dec!(800.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Basic tasks".to_string(),
            price: dec!(400.00),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(500.00)),
            max_price: Some(dec!(1000.00)),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Business Laptop");
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Will be deleted".to_string(),
            price: dec!(10.00),
            inventory_count: 1,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());

        let not_deleted = service.delete(999);
        assert!(!not_deleted);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precision Test".to_string(),
            description: "Testing decimal precision".to_string(),
            price: dec!(19.999),
            inventory_count: 1,
        });

        assert_eq!(product.price, dec!(19.999));

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, dec!(19.999));
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Product Thread 1 - {i}"),
                    description: "From thread 1".to_string(),
                    price: dec!(10.00),
                    inventory_count: 1,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Product Thread 2 - {i}"),
                    description: "From thread 2".to_string(),
                    price: dec!(20.00),
                    inventory_count: 1,
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
    fn test_concurrent_read_write() {
        let service = ProductService::new();

        // Pre-create some products
        for i in 0..5 {
            let _ = service.create(NewProduct {
                name: format!("Product {i}"),
                description: "Initial product".to_string(),
                price: dec!(10.00),
                inventory_count: 10,
            });
        }

        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let read_handle = thread::spawn(move || {
            for _ in 0..20 {
                let _ = service_clone1.get_all();
                let _ = service_clone1.get_by_id(1);
            }
        });

        let write_handle = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.update_inventory(1, i);
            }
        });

        read_handle.join().unwrap();
        write_handle.join().unwrap();

        // Verify service is still in a valid state
        let all_products = service.get_all();
        assert_eq!(all_products.len(), 5);
    }
}
