use actix_web::{web, HttpResponse, Responder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub inventory_count: i32,
}

/// In-memory product service for testing
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

    /// Creates a new product
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
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

    /// Gets all products
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_all(&self) -> Vec<Product> {
        self.products.lock().unwrap().clone()
    }

    /// Gets a product by ID
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_by_id(&self, id: i32) -> Option<Product> {
        self.products
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }
}

impl Default for ProductService {
    fn default() -> Self {
        Self::new()
    }
}

/// Get all products endpoint
pub async fn get_all_products(service: web::Data<ProductService>) -> impl Responder {
    let products = service.get_all();
    HttpResponse::Ok().json(products)
}

/// Get product by ID endpoint
pub async fn get_product_by_id(
    service: web::Data<ProductService>,
    id: web::Path<i32>,
) -> impl Responder {
    match service.get_by_id(*id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Product not found"})),
    }
}
