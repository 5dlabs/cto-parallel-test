use crate::catalog::models::{NewProduct, Product, ProductFilter};
use std::sync::{Arc, Mutex};

pub struct ProductService {
    products: Arc<Mutex<Vec<Product>>>,
    next_id: Arc<Mutex<i32>>,
}

impl ProductService {
    /// Creates a new `ProductService` with an empty product catalog.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::ProductService;
    ///
    /// let service = ProductService::new();
    /// assert_eq!(service.get_all().len(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Creates a new product with an auto-generated ID.
    ///
    /// # Arguments
    ///
    /// * `new_product` - The product data without an ID
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Test Product".to_string(),
    ///     description: "A test product".to_string(),
    ///     price: Decimal::new(1999, 2),
    ///     inventory_count: 10,
    /// });
    /// assert_eq!(product.id, 1);
    /// assert_eq!(product.name, "Test Product");
    /// ```
    #[must_use]
    pub fn create(&self, new_product: NewProduct) -> Product {
        let mut products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        let mut next_id = self.next_id.lock().expect("Failed to acquire next_id lock");

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

    /// Returns all products in the catalog.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// service.create(NewProduct {
    ///     name: "Product 1".to_string(),
    ///     description: "First product".to_string(),
    ///     price: Decimal::new(1000, 2),
    ///     inventory_count: 5,
    /// });
    /// service.create(NewProduct {
    ///     name: "Product 2".to_string(),
    ///     description: "Second product".to_string(),
    ///     price: Decimal::new(2000, 2),
    ///     inventory_count: 10,
    /// });
    ///
    /// let all = service.get_all();
    /// assert_eq!(all.len(), 2);
    /// ```
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        products.clone()
    }

    /// Finds a product by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to search for
    ///
    /// # Returns
    ///
    /// * `Some(Product)` if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Test Product".to_string(),
    ///     description: "A test product".to_string(),
    ///     price: Decimal::new(1999, 2),
    ///     inventory_count: 10,
    /// });
    ///
    /// let found = service.get_by_id(product.id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().id, product.id);
    ///
    /// let not_found = service.get_by_id(9999);
    /// assert!(not_found.is_none());
    /// ```
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");
        products.iter().find(|p| p.id == id).cloned()
    }

    /// Updates the inventory count for a product.
    ///
    /// # Arguments
    ///
    /// * `id` - The product ID to update
    /// * `new_count` - The new inventory count
    ///
    /// # Returns
    ///
    /// * `Some(Product)` with updated inventory if found, `None` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::NewProduct};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let product = service.create(NewProduct {
    ///     name: "Test Product".to_string(),
    ///     description: "A test product".to_string(),
    ///     price: Decimal::new(1999, 2),
    ///     inventory_count: 10,
    /// });
    ///
    /// let updated = service.update_inventory(product.id, 5);
    /// assert!(updated.is_some());
    /// assert_eq!(updated.unwrap().inventory_count, 5);
    ///
    /// let retrieved = service.get_by_id(product.id);
    /// assert_eq!(retrieved.unwrap().inventory_count, 5);
    /// ```
    #[must_use]
    pub fn update_inventory(&self, id: i32, new_count: i32) -> Option<Product> {
        let mut products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");

        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            product.inventory_count = new_count;
            Some(product.clone())
        } else {
            None
        }
    }

    /// Filters products based on the provided criteria.
    ///
    /// All filter criteria are combined with AND logic. Fields set to `None` are ignored.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter criteria to apply
    ///
    /// # Filter Fields
    ///
    /// * `name_contains` - Case-insensitive substring match on product name
    /// * `min_price` - Products with price >= `min_price`
    /// * `max_price` - Products with price <= `max_price`
    /// * `in_stock` - If true, products with `inventory_count` > 0; if false, products with `inventory_count` == 0
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex lock is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cto_parallel_test::catalog::{ProductService, models::{NewProduct, ProductFilter}};
    /// use rust_decimal::Decimal;
    ///
    /// let service = ProductService::new();
    /// let _ = service.create(NewProduct {
    ///     name: "Apple".to_string(),
    ///     description: "Fresh apple".to_string(),
    ///     price: Decimal::new(150, 2),
    ///     inventory_count: 10,
    /// });
    /// let _ = service.create(NewProduct {
    ///     name: "Banana".to_string(),
    ///     description: "Ripe banana".to_string(),
    ///     price: Decimal::new(75, 2),
    ///     inventory_count: 0,
    /// });
    /// let _ = service.create(NewProduct {
    ///     name: "Orange".to_string(),
    ///     description: "Juicy orange".to_string(),
    ///     price: Decimal::new(200, 2),
    ///     inventory_count: 5,
    /// });
    ///
    /// // Filter by name
    /// let filtered = service.filter(&ProductFilter {
    ///     name_contains: Some("app".to_string()),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 1);
    /// assert_eq!(filtered[0].name, "Apple");
    ///
    /// // Filter by price range
    /// let filtered = service.filter(&ProductFilter {
    ///     min_price: Some(Decimal::new(100, 2)),
    ///     max_price: Some(Decimal::new(180, 2)),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 1);
    ///
    /// // Filter by stock status
    /// let filtered = service.filter(&ProductFilter {
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 2); // Apple and Orange
    ///
    /// // Combined filters
    /// let filtered = service.filter(&ProductFilter {
    ///     name_contains: Some("a".to_string()),
    ///     in_stock: Some(true),
    ///     ..Default::default()
    /// });
    /// assert_eq!(filtered.len(), 2); // Apple and Orange contain "a"
    /// ```
    #[must_use]
    pub fn filter(&self, filter: &ProductFilter) -> Vec<Product> {
        let products = self
            .products
            .lock()
            .expect("Failed to acquire products lock");

        products
            .iter()
            .filter(|p| {
                let name_match = filter.name_contains.as_ref().is_none_or(|name| {
                    p.name.to_lowercase().contains(&name.to_lowercase())
                });

                let min_price_match = filter
                    .min_price
                    .as_ref()
                    .is_none_or(|min| p.price >= *min);

                let max_price_match = filter
                    .max_price
                    .as_ref()
                    .is_none_or(|max| p.price <= *max);

                let in_stock_match = filter
                    .in_stock
                    .is_none_or(|in_stock| (p.inventory_count > 0) == in_stock);

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
    }

    #[test]
    fn test_product_retrieval() {
        let service = ProductService::new();
        let created = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        let found = service.get_by_id(created.id);
        assert!(found.is_some());
        assert_eq!(found.as_ref().unwrap().id, created.id);
        assert_eq!(found.unwrap().name, created.name);

        let not_found = service.get_by_id(9999);
        assert!(not_found.is_none());

        let all = service.get_all();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_inventory_update() {
        let service = ProductService::new();
        let product = service.create(NewProduct {
            name: "Test Product".to_string(),
            description: "Description".to_string(),
            price: Decimal::new(1999, 2),
            inventory_count: 10,
        });

        let updated = service.update_inventory(product.id, 5);
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().inventory_count, 5);

        let retrieved = service.get_by_id(product.id).unwrap();
        assert_eq!(retrieved.inventory_count, 5);

        let not_updated = service.update_inventory(9999, 10);
        assert!(not_updated.is_none());
    }

    #[test]
    fn test_product_filtering() {
        let service = ProductService::new();

        // Create test products
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

        // Test name filter (case-insensitive)
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("app".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test price range filter
        let filtered = service.filter(&ProductFilter {
            min_price: Some(Decimal::new(100, 2)), // $1.00
            max_price: Some(Decimal::new(180, 2)), // $1.80
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Apple");

        // Test in_stock filter (true)
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange

        // Test in_stock filter (false)
        let filtered = service.filter(&ProductFilter {
            in_stock: Some(false),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 1); // Banana
        assert_eq!(filtered[0].name, "Banana");

        // Test combined filters
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("a".to_string()),
            in_stock: Some(true),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 2); // Apple and Orange contain "a"

        // Test empty filter returns all
        let filtered = service.filter(&ProductFilter::default());
        assert_eq!(filtered.len(), 3);

        // Test filter with no matches
        let filtered = service.filter(&ProductFilter {
            name_contains: Some("grape".to_string()),
            ..Default::default()
        });
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_get_all() {
        let service = ProductService::new();

        // Empty initially
        assert_eq!(service.get_all().len(), 0);

        // Add products
        let _ = service.create(NewProduct {
            name: "Product 1".to_string(),
            description: "First product".to_string(),
            price: Decimal::new(1000, 2),
            inventory_count: 5,
        });
        let _ = service.create(NewProduct {
            name: "Product 2".to_string(),
            description: "Second product".to_string(),
            price: Decimal::new(2000, 2),
            inventory_count: 10,
        });

        let all = service.get_all();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].id, 1);
        assert_eq!(all[1].id, 2);
    }
}
