//! Integration tests for the e-commerce application
//!
//! This module contains end-to-end integration tests that verify the complete
//! functionality of the application, including database operations, API endpoints,
//! and business logic.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::ignored_unit_patterns)]

use diesel::prelude::*;
use ecommerce_catalog::config::establish_connection_pool;
use ecommerce_catalog::models::{NewProduct, NewUser};
use ecommerce_catalog::schema::{products, users};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Helper function to set up a test database connection pool
fn setup_test_db() -> ecommerce_catalog::config::Pool {
    dotenv::dotenv().ok();
    establish_connection_pool()
}

/// Helper function to clean up test data
fn cleanup_test_data(conn: &mut ecommerce_catalog::config::DbConnection) {
    use ecommerce_catalog::schema::cart_items;
    use ecommerce_catalog::schema::carts;

    // Delete in order to respect foreign key constraints
    diesel::delete(cart_items::table).execute(conn).ok();
    diesel::delete(carts::table).execute(conn).ok();
    diesel::delete(products::table).execute(conn).ok();
    diesel::delete(users::table).execute(conn).ok();
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_database_connection() {
    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Simple query to verify connection works
    let result = diesel::sql_query("SELECT 1 as test").execute(&mut conn);

    assert!(result.is_ok(), "Database connection should work");
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_health_check_database() {
    // This test simulates a health check endpoint that verifies database connectivity
    let pool = setup_test_db();
    let mut conn = pool.get();

    assert!(
        conn.is_ok(),
        "Health check: database connection should be available"
    );

    if let Ok(ref mut connection) = conn {
        let query_result = diesel::sql_query("SELECT 1").execute(connection);
        assert!(
            query_result.is_ok(),
            "Health check: database should respond to queries"
        );
    }
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_create_and_retrieve_user() {
    use ecommerce_catalog::models::User;

    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Clean up any existing test data
    cleanup_test_data(&mut conn);

    // Create a new user
    let new_user = NewUser {
        username: "test_user_integration".to_string(),
        email: "test_integration@example.com".to_string(),
        password_hash: "hashed_password_123".to_string(),
    };

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn);

    assert!(insert_result.is_ok(), "User creation should succeed");

    // Retrieve the user
    let users_list: Vec<User> = users::table
        .filter(users::username.eq("test_user_integration"))
        .load(&mut conn)
        .expect("Failed to load users");

    assert_eq!(users_list.len(), 1, "Should find exactly one user");
    assert_eq!(users_list[0].username, "test_user_integration");
    assert_eq!(users_list[0].email, "test_integration@example.com");

    // Cleanup
    cleanup_test_data(&mut conn);
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_create_and_retrieve_product() {
    use ecommerce_catalog::models::Product;

    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Clean up any existing test data
    cleanup_test_data(&mut conn);

    // Create a new product
    let new_product = NewProduct {
        name: "Test Product Integration".to_string(),
        description: Some("A test product for integration testing".to_string()),
        price: Decimal::from_str("29.99").unwrap(),
        inventory_count: 100,
    };

    let insert_result = diesel::insert_into(products::table)
        .values(&new_product)
        .execute(&mut conn);

    assert!(insert_result.is_ok(), "Product creation should succeed");

    // Retrieve the product
    let products_list: Vec<Product> = products::table
        .filter(products::name.eq("Test Product Integration"))
        .load(&mut conn)
        .expect("Failed to load products");

    assert_eq!(products_list.len(), 1, "Should find exactly one product");
    assert_eq!(products_list[0].name, "Test Product Integration");
    assert_eq!(products_list[0].price, Decimal::from_str("29.99").unwrap());
    assert_eq!(products_list[0].inventory_count, 100);

    // Cleanup
    cleanup_test_data(&mut conn);
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_full_user_flow_preparation() {
    use ecommerce_catalog::models::{Cart, CartItem, NewCart, NewCartItem, Product, User};
    use ecommerce_catalog::schema::{cart_items, carts};

    // This test prepares the groundwork for a full user shopping flow
    // It creates a user and a product that can be used in a shopping cart scenario
    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Clean up any existing test data
    cleanup_test_data(&mut conn);

    // Step 1: Create a user
    let new_user = NewUser {
        username: "shopper_user".to_string(),
        email: "shopper@example.com".to_string(),
        password_hash: "hashed_password_shopper".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Failed to create user");

    // Step 2: Create a product
    let new_product = NewProduct {
        name: "Shopping Test Product".to_string(),
        description: Some("Product for shopping flow test".to_string()),
        price: Decimal::from_str("49.99").unwrap(),
        inventory_count: 50,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(&mut conn)
        .expect("Failed to create product");

    // Step 3: Verify both were created
    let users_count: i64 = users::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to count users");

    let products_count: i64 = products::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to count products");

    assert_eq!(users_count, 1, "Should have exactly one user");
    assert_eq!(products_count, 1, "Should have exactly one product");

    // Step 4: Retrieve the created records
    let user: User = users::table
        .filter(users::username.eq("shopper_user"))
        .first(&mut conn)
        .expect("Failed to retrieve user");

    let product: Product = products::table
        .filter(products::name.eq("Shopping Test Product"))
        .first(&mut conn)
        .expect("Failed to retrieve product");

    // Step 5: Create a cart for the user
    let new_cart = NewCart { user_id: user.id };

    diesel::insert_into(carts::table)
        .values(&new_cart)
        .execute(&mut conn)
        .expect("Failed to create cart");

    // Step 6: Add product to cart
    let cart: Cart = carts::table
        .filter(carts::user_id.eq(user.id))
        .first(&mut conn)
        .expect("Failed to retrieve cart");

    let new_cart_item = NewCartItem {
        cart_id: cart.id,
        product_id: product.id,
        quantity: 2,
    };

    diesel::insert_into(cart_items::table)
        .values(&new_cart_item)
        .execute(&mut conn)
        .expect("Failed to add item to cart");

    // Step 7: Verify the full flow
    let cart_items_list: Vec<CartItem> = cart_items::table
        .filter(cart_items::cart_id.eq(cart.id))
        .load(&mut conn)
        .expect("Failed to load cart items");

    assert_eq!(cart_items_list.len(), 1, "Cart should have one item");
    assert_eq!(cart_items_list[0].product_id, product.id);
    assert_eq!(cart_items_list[0].quantity, 2);

    // Cleanup
    cleanup_test_data(&mut conn);
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_product_inventory_management() {
    use ecommerce_catalog::models::Product;

    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Clean up any existing test data
    cleanup_test_data(&mut conn);

    // Create a product with initial inventory
    let new_product = NewProduct {
        name: "Inventory Test Product".to_string(),
        description: Some("Testing inventory management".to_string()),
        price: Decimal::from_str("19.99").unwrap(),
        inventory_count: 10,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(&mut conn)
        .expect("Failed to create product");

    // Retrieve the product
    let product: Product = products::table
        .filter(products::name.eq("Inventory Test Product"))
        .first(&mut conn)
        .expect("Failed to retrieve product");

    assert_eq!(
        product.inventory_count, 10,
        "Initial inventory should be 10"
    );

    // Update inventory (simulating a purchase)
    let new_inventory = product.inventory_count - 3;
    diesel::update(products::table.find(product.id))
        .set(products::inventory_count.eq(new_inventory))
        .execute(&mut conn)
        .expect("Failed to update inventory");

    // Verify inventory was updated
    let updated_product: Product = products::table
        .find(product.id)
        .first(&mut conn)
        .expect("Failed to retrieve updated product");

    assert_eq!(
        updated_product.inventory_count, 7,
        "Inventory should be decremented to 7"
    );

    // Cleanup
    cleanup_test_data(&mut conn);
}

#[test]
#[ignore = "Requires PostgreSQL database to be running"]
fn test_multiple_cart_items() {
    use ecommerce_catalog::models::{CartItem, NewCart, NewCartItem, Product};
    use ecommerce_catalog::schema::{cart_items, carts};

    let pool = setup_test_db();
    let mut conn = pool.get().expect("Failed to get database connection");

    // Clean up any existing test data
    cleanup_test_data(&mut conn);

    // Create a user
    let new_user = NewUser {
        username: "multi_cart_user".to_string(),
        email: "multi@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Failed to create user");

    let user: ecommerce_catalog::models::User = users::table
        .filter(users::username.eq("multi_cart_user"))
        .first(&mut conn)
        .expect("Failed to retrieve user");

    // Create multiple products
    let products_data = vec![
        NewProduct {
            name: "Product A".to_string(),
            description: Some("First product".to_string()),
            price: Decimal::from_str("10.00").unwrap(),
            inventory_count: 100,
        },
        NewProduct {
            name: "Product B".to_string(),
            description: Some("Second product".to_string()),
            price: Decimal::from_str("20.00").unwrap(),
            inventory_count: 50,
        },
        NewProduct {
            name: "Product C".to_string(),
            description: Some("Third product".to_string()),
            price: Decimal::from_str("30.00").unwrap(),
            inventory_count: 25,
        },
    ];

    for product_data in products_data {
        diesel::insert_into(products::table)
            .values(&product_data)
            .execute(&mut conn)
            .expect("Failed to create product");
    }

    // Create a cart
    let new_cart = NewCart { user_id: user.id };

    diesel::insert_into(carts::table)
        .values(&new_cart)
        .execute(&mut conn)
        .expect("Failed to create cart");

    let cart: ecommerce_catalog::models::Cart = carts::table
        .filter(carts::user_id.eq(user.id))
        .first(&mut conn)
        .expect("Failed to retrieve cart");

    // Add all products to cart
    let all_products: Vec<Product> = products::table
        .load(&mut conn)
        .expect("Failed to load products");

    for (index, product) in all_products.iter().enumerate() {
        let new_item = NewCartItem {
            cart_id: cart.id,
            product_id: product.id,
            quantity: i32::try_from(index + 1).unwrap_or(1),
        };

        diesel::insert_into(cart_items::table)
            .values(&new_item)
            .execute(&mut conn)
            .expect("Failed to add item to cart");
    }

    // Verify all items are in cart
    let cart_items_list: Vec<CartItem> = cart_items::table
        .filter(cart_items::cart_id.eq(cart.id))
        .load(&mut conn)
        .expect("Failed to load cart items");

    assert_eq!(cart_items_list.len(), 3, "Cart should have three items");

    // Cleanup
    cleanup_test_data(&mut conn);
}
