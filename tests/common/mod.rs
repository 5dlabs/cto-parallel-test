//! Common test utilities and helpers for integration tests.

use cto_parallel_test::catalog::{NewProduct, ProductService};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Creates a new `ProductService` with sample test data.
///
/// # Returns
///
/// A `ProductService` instance pre-populated with sample products for testing.
pub fn get_test_product_service() -> ProductService {
    let service = ProductService::new();

    // Add sample products for testing
    let _ = service.create(NewProduct {
        name: "Laptop".to_string(),
        description: "High-performance laptop".to_string(),
        price: Decimal::from_str("999.99").unwrap(),
        inventory_count: 10,
    });

    let _ = service.create(NewProduct {
        name: "Mouse".to_string(),
        description: "Wireless mouse".to_string(),
        price: Decimal::from_str("29.99").unwrap(),
        inventory_count: 50,
    });

    let _ = service.create(NewProduct {
        name: "Keyboard".to_string(),
        description: "Mechanical keyboard".to_string(),
        price: Decimal::from_str("79.99").unwrap(),
        inventory_count: 0, // Out of stock
    });

    let _ = service.create(NewProduct {
        name: "Monitor".to_string(),
        description: "4K UHD monitor".to_string(),
        price: Decimal::from_str("499.99").unwrap(),
        inventory_count: 5,
    });

    service
}

/// Creates an empty `ProductService` for testing.
///
/// # Returns
///
/// An empty `ProductService` instance.
#[must_use]
pub fn get_empty_product_service() -> ProductService {
    ProductService::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_product_service_creates_sample_data() {
        let service = get_test_product_service();
        let products = service.get_all();

        assert_eq!(products.len(), 4);
        assert_eq!(products[0].name, "Laptop");
        assert_eq!(products[1].name, "Mouse");
        assert_eq!(products[2].name, "Keyboard");
        assert_eq!(products[3].name, "Monitor");
    }

    #[test]
    fn test_get_empty_product_service_creates_empty_service() {
        let service = get_empty_product_service();
        let products = service.get_all();

        assert_eq!(products.len(), 0);
    }
}
