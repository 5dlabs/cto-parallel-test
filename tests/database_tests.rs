// Integration tests for database operations
// These tests require a running PostgreSQL instance with the DATABASE_URL environment variable set
// To run these tests:
// 1. Ensure PostgreSQL is running
// 2. Create the test database: createdb ecommerce_db
// 3. Set DATABASE_URL in .env file
// 4. Run migrations: diesel migration run
// 5. Run tests: cargo test --test database_tests -- --test-threads=1

use diesel::prelude::*;
use ecommerce_api::config::db::establish_connection_pool;
use ecommerce_api::models::{NewCart, NewCartItem, NewProduct, NewUser};
use ecommerce_api::schema::{cart_items, carts, products, users};

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_establish_connection_pool() {
    // Test that we can create a connection pool
    let pool = establish_connection_pool();
    assert!(
        pool.get().is_ok(),
        "Should be able to get a connection from the pool"
    );
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_create_and_retrieve_user() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Create a new user
    let new_user = NewUser {
        username: format!("test_user_{}", chrono::Utc::now().timestamp()),
        email: format!("test_{}@example.com", chrono::Utc::now().timestamp()),
        password_hash: "hashed_password".to_string(),
    };

    let inserted_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert user");

    assert!(inserted_user.id > 0);
    assert_eq!(inserted_user.username, new_user.username);
    assert_eq!(inserted_user.email, new_user.email);

    // Clean up
    diesel::delete(users::table.find(inserted_user.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_unique_username_constraint() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let timestamp = chrono::Utc::now().timestamp();
    let username = format!("unique_test_{timestamp}");
    let email1 = format!("email1_{timestamp}@example.com");
    let email2 = format!("email2_{timestamp}@example.com");

    // Insert first user
    let user1 = NewUser {
        username: username.clone(),
        email: email1,
        password_hash: "hash1".to_string(),
    };

    let inserted = diesel::insert_into(users::table)
        .values(&user1)
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert first user");

    // Try to insert second user with same username
    let user2 = NewUser {
        username: username.clone(),
        email: email2,
        password_hash: "hash2".to_string(),
    };

    let result = diesel::insert_into(users::table)
        .values(&user2)
        .get_result::<ecommerce_api::models::User>(&mut conn);

    assert!(
        result.is_err(),
        "Should fail due to unique username constraint"
    );

    // Clean up
    diesel::delete(users::table.find(inserted.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_unique_email_constraint() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let timestamp = chrono::Utc::now().timestamp();
    let email = format!("unique_email_{timestamp}@example.com");
    let username1 = format!("user1_{timestamp}");
    let username2 = format!("user2_{timestamp}");

    // Insert first user
    let user1 = NewUser {
        username: username1,
        email: email.clone(),
        password_hash: "hash1".to_string(),
    };

    let inserted = diesel::insert_into(users::table)
        .values(&user1)
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert first user");

    // Try to insert second user with same email
    let user2 = NewUser {
        username: username2,
        email: email.clone(),
        password_hash: "hash2".to_string(),
    };

    let result = diesel::insert_into(users::table)
        .values(&user2)
        .get_result::<ecommerce_api::models::User>(&mut conn);

    assert!(
        result.is_err(),
        "Should fail due to unique email constraint"
    );

    // Clean up
    diesel::delete(users::table.find(inserted.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_create_and_retrieve_product() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Create a new product
    let new_product = NewProduct {
        name: format!("Test Product {}", chrono::Utc::now().timestamp()),
        description: Some("A test product".to_string()),
        price: rust_decimal::Decimal::new(1999, 2), // $19.99
        inventory_count: 100,
    };

    let inserted_product = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result::<ecommerce_api::models::Product>(&mut conn)
        .expect("Failed to insert product");

    assert!(inserted_product.id > 0);
    assert_eq!(inserted_product.name, new_product.name);
    assert_eq!(inserted_product.price, new_product.price);
    assert_eq!(
        inserted_product.inventory_count,
        new_product.inventory_count
    );

    // Clean up
    diesel::delete(products::table.find(inserted_product.id))
        .execute(&mut conn)
        .expect("Failed to delete test product");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_create_cart_with_foreign_key() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // First create a user
    let new_user = NewUser {
        username: format!("cart_user_{}", chrono::Utc::now().timestamp()),
        email: format!("cart_user_{}@example.com", chrono::Utc::now().timestamp()),
        password_hash: "hashed_password".to_string(),
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert user");

    // Create a cart for the user
    let new_cart = NewCart { user_id: user.id };

    let cart = diesel::insert_into(carts::table)
        .values(&new_cart)
        .get_result::<ecommerce_api::models::Cart>(&mut conn)
        .expect("Failed to insert cart");

    assert!(cart.id > 0);
    assert_eq!(cart.user_id, user.id);

    // Clean up (cart should be deleted automatically due to CASCADE)
    diesel::delete(users::table.find(user.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");

    // Verify cart was cascade deleted
    let cart_exists = carts::table
        .find(cart.id)
        .first::<ecommerce_api::models::Cart>(&mut conn)
        .optional()
        .expect("Failed to query cart");

    assert!(
        cart_exists.is_none(),
        "Cart should be cascade deleted when user is deleted"
    );
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_create_cart_item_with_foreign_keys() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Create a user
    let new_user = NewUser {
        username: format!("item_user_{}", chrono::Utc::now().timestamp()),
        email: format!("item_user_{}@example.com", chrono::Utc::now().timestamp()),
        password_hash: "hashed_password".to_string(),
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert user");

    // Create a cart
    let new_cart = NewCart { user_id: user.id };
    let cart = diesel::insert_into(carts::table)
        .values(&new_cart)
        .get_result::<ecommerce_api::models::Cart>(&mut conn)
        .expect("Failed to insert cart");

    // Create a product
    let new_product = NewProduct {
        name: format!("Cart Item Product {}", chrono::Utc::now().timestamp()),
        description: Some("Test product for cart".to_string()),
        price: rust_decimal::Decimal::new(2999, 2), // $29.99
        inventory_count: 50,
    };

    let product = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result::<ecommerce_api::models::Product>(&mut conn)
        .expect("Failed to insert product");

    // Create a cart item
    let new_cart_item = NewCartItem {
        cart_id: cart.id,
        product_id: product.id,
        quantity: 2,
    };

    let cart_item = diesel::insert_into(cart_items::table)
        .values(&new_cart_item)
        .get_result::<ecommerce_api::models::CartItem>(&mut conn)
        .expect("Failed to insert cart item");

    assert!(cart_item.id > 0);
    assert_eq!(cart_item.cart_id, cart.id);
    assert_eq!(cart_item.product_id, product.id);
    assert_eq!(cart_item.quantity, 2);

    // Clean up (cascade delete should handle cart and cart_items)
    diesel::delete(users::table.find(user.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");

    diesel::delete(products::table.find(product.id))
        .execute(&mut conn)
        .expect("Failed to delete test product");

    // Verify cart_item was cascade deleted
    let item_exists = cart_items::table
        .find(cart_item.id)
        .first::<ecommerce_api::models::CartItem>(&mut conn)
        .optional()
        .expect("Failed to query cart item");

    assert!(item_exists.is_none(), "Cart item should be cascade deleted");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_cart_item_cascade_delete_on_product_deletion() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Create user, cart, product, and cart_item
    let user = diesel::insert_into(users::table)
        .values(&NewUser {
            username: format!("cascade_user_{}", chrono::Utc::now().timestamp()),
            email: format!("cascade_{}@example.com", chrono::Utc::now().timestamp()),
            password_hash: "hash".to_string(),
        })
        .get_result::<ecommerce_api::models::User>(&mut conn)
        .expect("Failed to insert user");

    let cart = diesel::insert_into(carts::table)
        .values(&NewCart { user_id: user.id })
        .get_result::<ecommerce_api::models::Cart>(&mut conn)
        .expect("Failed to insert cart");

    let product = diesel::insert_into(products::table)
        .values(&NewProduct {
            name: format!("Cascade Product {}", chrono::Utc::now().timestamp()),
            description: None,
            price: rust_decimal::Decimal::new(1000, 2),
            inventory_count: 10,
        })
        .get_result::<ecommerce_api::models::Product>(&mut conn)
        .expect("Failed to insert product");

    let cart_item = diesel::insert_into(cart_items::table)
        .values(&NewCartItem {
            cart_id: cart.id,
            product_id: product.id,
            quantity: 1,
        })
        .get_result::<ecommerce_api::models::CartItem>(&mut conn)
        .expect("Failed to insert cart item");

    // Delete the product
    diesel::delete(products::table.find(product.id))
        .execute(&mut conn)
        .expect("Failed to delete product");

    // Verify cart_item was cascade deleted
    let item_exists = cart_items::table
        .find(cart_item.id)
        .first::<ecommerce_api::models::CartItem>(&mut conn)
        .optional()
        .expect("Failed to query cart item");

    assert!(
        item_exists.is_none(),
        "Cart item should be cascade deleted when product is deleted"
    );

    // Clean up
    diesel::delete(users::table.find(user.id))
        .execute(&mut conn)
        .expect("Failed to delete test user");
}

#[test]
#[ignore = "Requires PostgreSQL to be running"]
fn test_numeric_precision_for_price() {
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Test that prices maintain decimal precision
    let product = diesel::insert_into(products::table)
        .values(&NewProduct {
            name: format!("Precision Product {}", chrono::Utc::now().timestamp()),
            description: Some("Tests decimal precision".to_string()),
            price: rust_decimal::Decimal::new(123_456, 2), // $1234.56
            inventory_count: 5,
        })
        .get_result::<ecommerce_api::models::Product>(&mut conn)
        .expect("Failed to insert product");

    // Verify the price was stored correctly
    assert_eq!(product.price.to_string(), "1234.56");

    // Clean up
    diesel::delete(products::table.find(product.id))
        .execute(&mut conn)
        .expect("Failed to delete test product");
}
