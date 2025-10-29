use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::auth::validate_token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: i32,
    pub quantity: i32,
}

/// In-memory cart service for testing
pub struct CartService {
    carts: Arc<Mutex<HashMap<String, Cart>>>,
}

impl CartService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds an item to the cart
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    pub fn add_item(&self, user_id: &str, product_id: i32, quantity: i32) {
        let mut carts = self.carts.lock().unwrap();
        let cart = carts.entry(user_id.to_string()).or_insert_with(|| Cart {
            user_id: user_id.to_string(),
            items: Vec::new(),
        });

        // Check if product already exists in cart
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                product_id,
                quantity,
            });
        }
    }

    /// Gets the cart for a user
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned (only happens if another thread panicked while holding the lock)
    #[must_use]
    pub fn get_cart(&self, user_id: &str) -> Cart {
        let carts = self.carts.lock().unwrap();
        carts.get(user_id).cloned().unwrap_or_else(|| Cart {
            user_id: user_id.to_string(),
            items: Vec::new(),
        })
    }
}

impl Default for CartService {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract user ID from JWT token in Authorization header
fn extract_user_id(req: &HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "Missing authorization header"}))
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "Invalid authorization header"}))
    })?;

    let claims = validate_token(token).map_err(|_| {
        HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid token"}))
    })?;

    Ok(claims.sub)
}

/// Add item to cart endpoint
pub async fn add_to_cart(
    service: web::Data<CartService>,
    req: HttpRequest,
    body: web::Json<AddToCartRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    service.add_item(&user_id, body.product_id, body.quantity);
    HttpResponse::Ok().json(serde_json::json!({"status": "success"}))
}

/// Get cart endpoint
pub async fn get_cart(service: web::Data<CartService>, req: HttpRequest) -> impl Responder {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let cart = service.get_cart(&user_id);
    HttpResponse::Ok().json(cart)
}
