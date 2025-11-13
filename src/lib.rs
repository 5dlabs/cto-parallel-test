pub mod config;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use models::{NewCart, NewCartItem, NewProduct, NewUser};
    use std::str::FromStr;

    /// Test that verifies the module structure and imports compile correctly
    #[test]
    fn test_module_structure() {
        // This test ensures all modules are properly declared and accessible
        use config::db::{DbConnection, Pool};

        // Verify type aliases are accessible - just checking compilation
        let pool_type: Option<Pool> = None;
        let conn_type: Option<DbConnection> = None;
        assert!(pool_type.is_none());
        assert!(conn_type.is_none());

        // Verify all schema modules exist and can be referenced
        // This compilation test ensures the schema.rs file is correctly generated
        let _ = &schema::users::table;
        let _ = &schema::products::table;
        let _ = &schema::carts::table;
        let _ = &schema::cart_items::table;
    }

    /// Test that verifies model structs can be instantiated
    #[test]
    fn test_model_instantiation() {
        // Test NewUser instantiation
        let new_user = NewUser {
            username: String::from("testuser"),
            email: String::from("test@example.com"),
            password_hash: String::from("hashed_password"),
        };
        assert_eq!(new_user.username, "testuser");
        assert_eq!(new_user.email, "test@example.com");

        // Test NewProduct instantiation
        let new_product = NewProduct {
            name: String::from("Test Product"),
            description: Some(String::from("A test product")),
            price: BigDecimal::from_str("19.99").expect("Invalid price"),
            inventory_count: 100,
        };
        assert_eq!(new_product.name, "Test Product");
        assert_eq!(new_product.inventory_count, 100);

        // Test NewCart instantiation
        let new_cart = NewCart { user_id: 1 };
        assert_eq!(new_cart.user_id, 1);

        // Test NewCartItem instantiation
        let new_cart_item = NewCartItem {
            cart_id: 1,
            product_id: 1,
            quantity: 2,
        };
        assert_eq!(new_cart_item.quantity, 2);
    }

    /// Integration test that requires a live database connection.
    /// This test will be skipped if `DATABASE_URL` is not configured or database is not available.
    #[test]
    #[ignore = "Requires live PostgreSQL database"]
    fn test_database_connection() {
        use config::db::establish_connection_pool;
        use diesel::prelude::*;

        dotenv::dotenv().ok();
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection from pool");

        // Test that we can query the database
        let result = diesel::sql_query("SELECT 1")
            .execute(&mut conn)
            .expect("Failed to execute query");

        assert!(result > 0);
    }

    /// Integration test that verifies database schema.
    /// This test will be skipped if `DATABASE_URL` is not configured or database is not available.
    #[test]
    #[ignore = "Requires live PostgreSQL database"]
    fn test_schema_exists() {
        use config::db::establish_connection_pool;
        use diesel::prelude::*;
        use schema::{cart_items, carts, products, users};

        dotenv::dotenv().ok();
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection from pool");

        // Verify all tables exist by querying their structure
        // This will fail at compile time if the schema doesn't match
        let _users_count = users::table
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Failed to query users table");

        let _products_count = products::table
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Failed to query products table");

        let _carts_count = carts::table
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Failed to query carts table");

        let _cart_items_count = cart_items::table
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Failed to query cart_items table");
    }
}
