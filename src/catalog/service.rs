use super::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

/// Thread-safe in-memory product catalog service
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

    /// Creates a new product and returns it with an assigned ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut next_id = self.next_id.lock().expect("Mutex poisoned");
        let id = *next_id;
        *next_id += 1;

        let product = Product {
            id,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
            inventory_count: new_product.inventory_count,
        };

        let mut products = self.products.lock().expect("Mutex poisoned");
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
        self.products.lock().expect("Mutex poisoned").clone()
    }

    /// Finds a product by ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        self.products
            .lock()
            .expect("Mutex poisoned")
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }

    /// Updates the inventory count for a product
    ///
    /// Returns `true` if the product was found and updated, `false` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> bool {
        let mut products = self.products.lock().expect("Mutex poisoned");
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            true
        } else {
            false
        }
    }

    /// Filters products based on the provided criteria
    ///
    /// All filter conditions are combined with AND logic.
    /// `None` values are treated as "no filter" for that criterion.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self.products.lock().expect("Mutex poisoned");

        products
            .iter()
            .filter(|p| {
                // Name filter
                if let Some(ref name_contains) = filter.name_contains {
                    if !p
                        .name
                        .to_lowercase()
                        .contains(&name_contains.to_lowercase())
                    {
                        return false;
                    }
                }

                // Min price filter
                if let Some(min_price) = filter.min_price {
                    if p.price < min_price {
                        return false;
                    }
                }

                // Max price filter
                if let Some(max_price) = filter.max_price {
                    if p.price > max_price {
                        return false;
                    }
                }

                // In stock filter
                if let Some(in_stock) = filter.in_stock {
                    if in_stock && p.inventory_count <= 0 {
                        return false;
                    }
                    if !in_stock && p.inventory_count > 0 {
                        return false;
                    }
                }

                true
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
    fn test_create_product() {
        let service = ProductService::new();
        let new_product = NewProduct {
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            price: Decimal::new(1999, 2), // $19.99
            inventory_count: 10,
        };

        let product = service.create(new_product);
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Decimal::new(1999, 2));
    }

    #[test]
    fn test_get_all_products() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });
        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 10,
        });

        let products = service.get_all();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 1,
        });

        let found = service.get_by_id(product.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");

        let not_found = service.get_by_id(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_inventory() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test".to_string(),
            description: "Desc".to_string(),
            price: Decimal::new(100, 2),
            inventory_count: 10,
        });

        assert!(service.update_inventory(product.id, 5));
        let updated = service.get_by_id(product.id).unwrap();
        assert_eq!(updated.inventory_count, 5);

        assert!(!service.update_inventory(999, 5));
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

        let filter = ProductFilter {
            name_contains: Some("Apple".to_string()),
            ..Default::default()
        };
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Apple iPhone");
    }

    #[test]
    fn test_filter_by_price_range() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "Cheap".to_string(),
            description: "Low price".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Expensive".to_string(),
            description: "High price".to_string(),
            price: Decimal::new(10000, 2),
            inventory_count: 5,
        });

        let filter = ProductFilter {
            min_price: Some(Decimal::new(5000, 2)),
            max_price: Some(Decimal::new(15000, 2)),
            ..Default::default()
        };
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Expensive");
    }

    #[test]
    fn test_filter_in_stock() {
        let service = ProductService::new();
        let _ = service.create(NewProduct {
            name: "In Stock".to_string(),
            description: "Available".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 10,
        });
        let _ = service.create(NewProduct {
            name: "Out of Stock".to_string(),
            description: "Not available".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 0,
        });

        let filter = ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        };
        let results = service.filter(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "In Stock");
    }
}
