use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Creates a test product service with some sample data
#[must_use]
pub fn create_test_product_service() -> ProductService {
    let service = ProductService::new();

    // Add some sample products
    let _ = service.create(NewProduct {
        name: "Apple iPhone 14".to_string(),
        description: "Latest iPhone model with advanced features".to_string(),
        price: Decimal::from_str("999.99").unwrap(),
        inventory_count: 50,
    });

    let _ = service.create(NewProduct {
        name: "Samsung Galaxy S23".to_string(),
        description: "Flagship Android smartphone".to_string(),
        price: Decimal::from_str("899.99").unwrap(),
        inventory_count: 30,
    });

    let _ = service.create(NewProduct {
        name: "Apple Watch Series 8".to_string(),
        description: "Smart watch with health monitoring".to_string(),
        price: Decimal::from_str("399.99").unwrap(),
        inventory_count: 100,
    });

    let _ = service.create(NewProduct {
        name: "Sony WH-1000XM5".to_string(),
        description: "Premium noise-cancelling headphones".to_string(),
        price: Decimal::from_str("349.99").unwrap(),
        inventory_count: 0,
    });

    service
}

/// Creates a product service with a single test product
#[must_use]
pub fn create_minimal_product_service() -> ProductService {
    let service = ProductService::new();

    let _ = service.create(NewProduct {
        name: "Test Product".to_string(),
        description: "A product for testing".to_string(),
        price: Decimal::from_str("19.99").unwrap(),
        inventory_count: 10,
    });

    service
}

/// Helper to create a new product for testing
#[must_use]
pub fn create_test_product(name: &str, price: &str, inventory: i32) -> NewProduct {
    NewProduct {
        name: name.to_string(),
        description: format!("Description for {name}"),
        price: Decimal::from_str(price).unwrap(),
        inventory_count: inventory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_product_service() {
        let service = create_test_product_service();
        let products = service.get_all();
        assert_eq!(products.len(), 4);
    }

    #[test]
    fn test_create_minimal_product_service() {
        let service = create_minimal_product_service();
        let products = service.get_all();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, "Test Product");
    }

    #[test]
    fn test_create_test_product() {
        let product = create_test_product("Widget", "29.99", 5);
        assert_eq!(product.name, "Widget");
        assert_eq!(product.price, Decimal::from_str("29.99").unwrap());
        assert_eq!(product.inventory_count, 5);
    }
}
