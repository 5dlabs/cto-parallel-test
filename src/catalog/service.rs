use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe product catalog service with in-memory storage
#[derive(Debug, Clone)]
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

    /// Creates a new product with auto-incrementing ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (which should never happen in normal operation)
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        let mut next_id = self.next_id.lock().expect("Next ID mutex poisoned");

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
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// Returns the updated product if found, None otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("Products mutex poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// All filters use AND logic (product must match all specified criteria)
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
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

    /// Deletes a product by ID
    ///
    /// Returns true if a product was deleted, false if no product with that ID exists
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("Products mutex poisoned");
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
    fn test_auto_incrementing_ids() {
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
            description: "First product".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: dec!(20.00),
            inventory_count: 3,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0].name, "Product 1");
        assert_eq!(all_products[1].name, "Product 2");
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let found = service.get_by_id(product.id);
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
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 20);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 20);

        let fetched = service.get_by_id(product.id);
        assert_eq!(fetched.unwrap().inventory_count, 20);

        let not_found = service.update_inventory(999, 100);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Wireless mouse".to_string(),
            price: dec!(29.99),
            inventory_count: 20,
        });

        let filter = ProductFilter::new().with_name("lap".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_case_insensitive() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Gaming laptop".to_string(),
            price: dec!(999.99),
            inventory_count: 5,
        });

        let filter = ProductFilter::new().with_name("LAPTOP".to_string());
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: dec!(5.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Medium price".to_string(),
            price: dec!(50.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: dec!(500.00),
            inventory_count: 2,
        });

        let filter = ProductFilter::new()
            .with_min_price(dec!(20.00))
            .with_max_price(dec!(100.00));

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium Item");
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
            description: "Unavailable".to_string(),
            price: dec!(20.00),
            inventory_count: 0,
        });

        let filter_in_stock = ProductFilter::new().with_in_stock(true);
        let in_stock_results = service.filter(&filter_in_stock);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock");

        let filter_out_of_stock = ProductFilter::new().with_in_stock(false);
        let out_of_stock_results = service.filter(&filter_out_of_stock);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "Professional laptop".to_string(),
            price: dec!(1500.00),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Basic laptop".to_string(),
            price: dec!(500.00),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Desktop".to_string(),
            description: "Desktop computer".to_string(),
            price: dec!(800.00),
            inventory_count: 5,
        });

        let filter = ProductFilter::new()
            .with_name("laptop".to_string())
            .with_min_price(dec!(400.00))
            .with_in_stock(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop Pro");
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        assert!(service.delete(product.id));
        assert!(service.get_by_id(product.id).is_none());
        assert!(!service.delete(product.id)); // Already deleted
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Test decimal precision".to_string(),
            price: dec!(19.995),
            inventory_count: 1,
        });

        let fetched = service.get_by_id(product.id).unwrap();
        assert_eq!(fetched.price, dec!(19.995));
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
                    price: dec!(10.00),
                    inventory_count: 1,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 10..20 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: dec!(20.00),
                    inventory_count: 2,
                });
            }
        });

        handle1.join().expect("Thread 1 panicked");
        handle2.join().expect("Thread 2 panicked");

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 20);
    }

    #[test]
    fn test_empty_filter_returns_all() {
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
            inventory_count: 0,
        });

        let filter = ProductFilter::new();
        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test".to_string(),
            price: dec!(10.00),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, -5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -5);
    }
}
