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
    ///
    /// Panics if the mutex is poisoned
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

    /// Returns all products in the catalog
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
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
        let mut products = self.products.lock().unwrap();
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// All non-None filter fields are combined with AND logic
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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

    /// Deletes a product by ID
    ///
    /// Returns true if the product was deleted, false if it wasn't found
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
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
            inventory_count: 100,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, dec!(19.99));
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 20,
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
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: dec!(20.00),
            inventory_count: 20,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id_found() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Description".to_string(),
            price: dec!(15.00),
            inventory_count: 5,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Product");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let service = ProductService::new();
        let found = service.get_by_id(999);
        assert!(found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product".to_string(),
            description: "Description".to_string(),
            price: dec!(10.00),
            inventory_count: 100,
        });

        let updated = service.update_inventory(product.id, 50);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 50);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 50);
    }

    #[test]
    fn test_update_inventory_not_found() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Computer".to_string(),
            price: dec!(1000.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "Peripheral".to_string(),
            price: dec!(25.00),
            inventory_count: 50,
        });

        let filter = ProductFilter {
            name_contains: Some("lap".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "Computer".to_string(),
            price: dec!(1000.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("LAP".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Mid price".to_string(),
            price: dec!(50.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: dec!(100.00),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(dec!(20.00)),
            max_price: Some(dec!(80.00)),
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Medium");
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
            price: dec!(10.00),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop Pro".to_string(),
            description: "High-end".to_string(),
            price: dec!(1500.00),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Basic".to_string(),
            description: "Budget".to_string(),
            price: dec!(500.00),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Laptop Ultra".to_string(),
            description: "Premium".to_string(),
            price: dec!(2000.00),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(dec!(400.00)),
            max_price: Some(dec!(1600.00)),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Will be removed".to_string(),
            price: dec!(10.00),
            inventory_count: 10,
        });

        let deleted = service.delete(product.id);
        assert!(deleted);

        let found = service.get_by_id(product.id);
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_nonexistent_product() {
        let service = ProductService::new();
        let deleted = service.delete(999);
        assert!(!deleted);
    }

    #[test]
    fn test_concurrent_creation() {
        let service = ProductService::new();
        let service_clone1 = service.clone();
        let service_clone2 = service.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone1.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Thread 1".to_string(),
                    price: dec!(10.00),
                    inventory_count: 10,
                });
            }
        });

        let handle2 = thread::spawn(move || {
            for i in 0..10 {
                let _ = service_clone2.create(NewProduct {
                    name: format!("Product {i}"),
                    description: "Thread 2".to_string(),
                    price: dec!(20.00),
                    inventory_count: 20,
                });
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let products = service.get_all();
        assert_eq!(products.len(), 20);

        // Verify all IDs are unique
        let mut ids: Vec<i32> = products.iter().map(|p| p.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Precise".to_string(),
            description: "Test".to_string(),
            price: dec!(19.99),
            inventory_count: 10,
        });

        assert_eq!(product.price, dec!(19.99));

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, dec!(19.99));
    }
}
