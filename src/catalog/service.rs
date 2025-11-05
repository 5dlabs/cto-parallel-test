use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and returns it with an auto-generated ID.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self.products.lock().expect("products lock poisoned");
        let mut next_id = self.next_id.lock().expect("next_id lock poisoned");

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

    /// Returns all products.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("products lock poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("products lock poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// Returns the updated product if found, or `None` if the product doesn't exist.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self.products.lock().expect("products lock poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("products lock poisoned");
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

    /// Deletes a product by ID.
    ///
    /// Returns `true` if the product was deleted, `false` if it wasn't found.
    ///
    /// # Panics
    /// Panics if the mutex lock is poisoned.
    #[must_use]
    pub fn delete(&self, id: i32) -> bool {
        let mut products = self.products.lock().expect("products lock poisoned");
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

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, "A test product");
        assert_eq!(product.price, Decimal::from_str("29.99").unwrap());
        assert_eq!(product.inventory_count, 10);
    }

    #[test]
    fn test_auto_incrementing_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let product2 = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
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
            description: "Description".to_string(),
            price: Decimal::from_str("15.99").unwrap(),
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
            description: "Description".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let not_found = service.update_inventory(999, 10);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_filter_by_name() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Mouse".to_string(),
            description: "A mouse".to_string(),
            price: Decimal::from_str("29.99").unwrap(),
            inventory_count: 20,
        });

        let mut filter = ProductFilter::new();
        filter.name_contains = Some("lap".to_string());

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_name_case_insensitive() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Laptop".to_string(),
            description: "A laptop".to_string(),
            price: Decimal::from_str("999.99").unwrap(),
            inventory_count: 5,
        });

        let mut filter = ProductFilter::new();
        filter.name_contains = Some("LAPTOP".to_string());

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap Item".to_string(),
            description: "Low price".to_string(),
            price: Decimal::from_str("5.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium Item".to_string(),
            description: "Medium price".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 2,
        });

        let mut filter = ProductFilter::new();
        filter.min_price = Some(Decimal::from_str("20.00").unwrap());
        filter.max_price = Some(Decimal::from_str("100.00").unwrap());

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
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 0,
        });

        let mut filter_in_stock = ProductFilter::new();
        filter_in_stock.in_stock = Some(true);

        let in_stock_results = service.filter(&filter_in_stock);
        assert_eq!(in_stock_results.len(), 1);
        assert_eq!(in_stock_results[0].name, "In Stock");

        let mut filter_out_of_stock = ProductFilter::new();
        filter_out_of_stock.in_stock = Some(false);

        let out_of_stock_results = service.filter(&filter_out_of_stock);
        assert_eq!(out_of_stock_results.len(), 1);
        assert_eq!(out_of_stock_results[0].name, "Out of Stock");
    }

    #[test]
    fn test_filter_combined() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Gaming Laptop".to_string(),
            description: "High performance".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business use".to_string(),
            price: Decimal::from_str("800.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Basic use".to_string(),
            price: Decimal::from_str("400.00").unwrap(),
            inventory_count: 0,
        });

        let mut filter = ProductFilter::new();
        filter.name_contains = Some("laptop".to_string());
        filter.min_price = Some(Decimal::from_str("500.00").unwrap());
        filter.in_stock = Some(true);

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|p| p.name == "Gaming Laptop"));
        assert!(results.iter().any(|p| p.name == "Office Laptop"));
    }

    #[test]
    fn test_delete_product() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "To Delete".to_string(),
            description: "Will be deleted".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 1,
        });

        assert!(service.delete(product.id));
        assert_eq!(service.get_all().len(), 0);

        assert!(!service.delete(999));
    }

    #[test]
    fn test_decimal_precision() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Precise Product".to_string(),
            description: "Tests decimal precision".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 5,
        });

        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let service = Arc::new(ProductService::new());
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                let _ = service_clone.create(NewProduct {
                    name: format!("Product {i}"),
                    description: format!("Description {i}"),
                    price: Decimal::from_str("10.00").unwrap(),
                    inventory_count: i,
                });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 10);
    }

    #[test]
    fn test_negative_inventory() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Testing negative inventory".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let updated = service.update_inventory(product.id, -1);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, -1);
    }
}
