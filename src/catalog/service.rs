use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product service
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

    /// Creates a new product and assigns it an ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
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
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self.products.lock().unwrap();
        products.clone()
    }

    /// Retrieves a product by its ID
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self.products.lock().unwrap();
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
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

    /// Filters products based on provided criteria
    ///
    /// # Panics
    /// Panics if the mutex is poisoned (extremely rare in normal operation)
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().unwrap();

        products
            .iter()
            .filter(|p| {
                // Name filter: case-insensitive substring match
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                // Min price filter: price >= minimum
                let min_price_match = filter.min_price.as_ref().is_none_or(|min| p.price >= *min);

                // Max price filter: price <= maximum
                let max_price_match = filter.max_price.as_ref().is_none_or(|max| p.price <= *max);

                // In stock filter: inventory_count > 0
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
    use std::str::FromStr;

    #[test]
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::from_str("19.99").unwrap(),
            inventory_count: 10,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::from_str("19.99").unwrap());
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 3,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_str("15.00").unwrap(),
            inventory_count: 8,
        });

        let found = service.get_by_id(product.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Product 1");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::from_str("25.00").unwrap(),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 15);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 15);

        let retrieved = service.get_by_id(product.id);
        assert_eq!(retrieved.unwrap().inventory_count, 15);
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
            inventory_count: 10,
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
    fn test_filter_by_price_range() {
        let service = ProductService::new();

        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Cheap product".to_string(),
            price: Decimal::from_str("5.00").unwrap(),
            inventory_count: 10,
        });

        let _ = service.create(NewProduct {
            name: "Medium".to_string(),
            description: "Medium product".to_string(),
            price: Decimal::from_str("50.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "Expensive product".to_string(),
            price: Decimal::from_str("500.00").unwrap(),
            inventory_count: 2,
        });

        let filter = ProductFilter {
            name_contains: None,
            min_price: Some(Decimal::from_str("10.00").unwrap()),
            max_price: Some(Decimal::from_str("100.00").unwrap()),
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
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 5,
        });

        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::from_str("30.00").unwrap(),
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
            name: "Gaming Laptop".to_string(),
            description: "High-end laptop".to_string(),
            price: Decimal::from_str("1500.00").unwrap(),
            inventory_count: 3,
        });

        let _ = service.create(NewProduct {
            name: "Office Laptop".to_string(),
            description: "Business laptop".to_string(),
            price: Decimal::from_str("800.00").unwrap(),
            inventory_count: 0,
        });

        let _ = service.create(NewProduct {
            name: "Budget Laptop".to_string(),
            description: "Entry-level laptop".to_string(),
            price: Decimal::from_str("400.00").unwrap(),
            inventory_count: 10,
        });

        let filter = ProductFilter {
            name_contains: Some("laptop".to_string()),
            min_price: Some(Decimal::from_str("300.00").unwrap()),
            max_price: Some(Decimal::from_str("1000.00").unwrap()),
            in_stock: Some(true),
        };

        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Budget Laptop");
    }
}
