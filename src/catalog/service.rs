use std::sync::{Arc, Mutex};

use super::models::{NewProduct, Product, ProductFilter};

/// Thread-safe in-memory product catalog service
///
/// This service manages product data in memory with automatic ID assignment
/// and supports concurrent access through Arc<Mutex<>> wrappers.
#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new empty product service
    ///
    /// The service starts with no products and ID counter at 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product and assigns it an auto-incrementing ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare, indicates thread panic while holding lock)
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

    /// Retrieves all products in the catalog
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// Returns `None` if no product with the given ID exists.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a specific product
    ///
    /// Returns the updated product if found, or `None` if the product doesn't exist.
    ///
    /// # Panics
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

    /// Filters products based on multiple criteria (all must match)
    ///
    /// All filter fields are optional:
    /// - `name_contains`: Case-insensitive substring match on product name
    /// - `min_price`: Products with price >= `min_price`
    /// - `max_price`: Products with price <= `max_price`
    /// - `in_stock`: Products where (`inventory_count` > 0) matches the flag
    ///
    /// If a filter field is `None`, it's ignored (no filtering on that criterion).
    /// All active filters combine with AND logic.
    ///
    /// # Panics
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Products mutex poisoned");

        products
            .iter()
            .filter(|product| {
                // Check name_contains (case-insensitive)
                let name_match = filter.name_contains.as_ref().is_none_or(|pattern| {
                    product
                        .name
                        .to_lowercase()
                        .contains(&pattern.to_lowercase())
                });

                // Check min_price
                let min_price_match = filter
                    .min_price
                    .is_none_or(|min_price| product.price >= min_price);

                // Check max_price
                let max_price_match = filter
                    .max_price
                    .is_none_or(|max_price| product.price <= max_price);

                // Check in_stock
                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (product.inventory_count > 0) == in_stock);

                // All filters must pass (AND logic)
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
    use std::str::FromStr;

    #[test]
    fn test_create_product_assigns_sequential_ids() {
        let service = ProductService::new();

        let product1 = service.create(NewProduct {
            name: "Widget".to_string(),
            description: "A useful widget".to_string(),
            price: Decimal::from_str("10.99").unwrap(),
            inventory_count: 100,
        });

        let product2 = service.create(NewProduct {
            name: "Gadget".to_string(),
            description: "A handy gadget".to_string(),
            price: Decimal::from_str("25.50").unwrap(),
            inventory_count: 50,
        });

        assert_eq!(product1.id, 1);
        assert_eq!(product2.id, 2);
    }

    #[test]
    fn test_get_all_returns_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product A".to_string(),
            description: "Description A".to_string(),
            price: Decimal::from_str("5.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product B".to_string(),
            description: "Description B".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 20,
        });

        let all_products = service.get_all();
        assert_eq!(all_products.len(), 2);
    }

    #[test]
    fn test_get_by_id_returns_correct_product() {
        let service = ProductService::new();

        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: Decimal::from_str("99.99").unwrap(),
            inventory_count: 5,
        });

        let retrieved = service.get_by_id(created.id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Product");
    }

    #[test]
    fn test_get_by_id_returns_none_for_nonexistent() {
        let service = ProductService::new();
        let result = service.get_by_id(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_update_inventory_modifies_count() {
        let service = ProductService::new();

        let product = service.create(NewProduct {
            name: "Inventory Test".to_string(),
            description: "Testing inventory".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 100,
        });

        let updated = service.update_inventory(product.id, 50);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 50);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 50);
    }

    #[test]
    fn test_update_inventory_returns_none_for_nonexistent() {
        let service = ProductService::new();
        let result = service.update_inventory(999, 100);
        assert!(result.is_none());
    }

    #[test]
    fn test_filter_by_name_contains() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Blue Widget".to_string(),
            description: "Blue colored widget".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Red Gadget".to_string(),
            description: "Red colored gadget".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 20,
        });

        let filter = ProductFilter {
            name_contains: Some("widget".to_string()),
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert!(results[0].name.contains("Widget"));
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
            name: "Mid Item".to_string(),
            description: "Medium price".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Expensive Item".to_string(),
            description: "High price".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(Decimal::from_str("10.00").unwrap()),
            max_price: Some(Decimal::from_str("20.00").unwrap()),
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mid Item");
    }

    #[test]
    fn test_filter_by_in_stock() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Available".to_string(),
            description: "In stock".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
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
        assert_eq!(results[0].name, "Available");
    }

    #[test]
    fn test_filter_combines_all_criteria() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Blue Widget".to_string(),
            description: "Match all".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Blue Gadget".to_string(),
            description: "Wrong price".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Red Widget".to_string(),
            description: "Wrong name".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("blue".to_string()),
            min_price: Some(Decimal::from_str("10.00").unwrap()),
            max_price: Some(Decimal::from_str("20.00").unwrap()),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Blue Widget");
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: None,
            max_price: None,
            in_stock: None,
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_service_is_thread_safe() {
        use std::thread;

        let service = ProductService::new();
        let service_clone = service.clone();

        let handle = thread::spawn(move || {
            let _ = service_clone.create(NewProduct {
                name: "Thread Product".to_string(),
                description: "Created in thread".to_string(),
                price: Decimal::from_str("10.00").unwrap(),
                inventory_count: 1,
            });
        });

        handle.join().unwrap();

        let products = service.get_all();
        assert_eq!(products.len(), 1);
    }
}
