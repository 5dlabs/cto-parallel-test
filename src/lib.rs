#![forbid(unsafe_code)]

pub mod config;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::prelude::*;

    /// Helper function to check if database is available
    fn is_database_available() -> bool {
        dotenv::dotenv().ok();
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            use diesel::pg::PgConnection;
            use diesel::Connection;
            PgConnection::establish(&database_url).is_ok()
        } else {
            false
        }
    }

    #[test]
    fn test_connection_pool_creation() {
        // Skip test if database is not available
        if !is_database_available() {
            // Database not available - test is skipped
            return;
        }

        // This test verifies that the connection pool can be created successfully
        let pool = config::db::establish_connection_pool();
        assert!(pool.get().is_ok());
    }

    #[test]
    fn test_database_schema() {
        use crate::schema::users::dsl::*;

        // Skip test if database is not available
        if !is_database_available() {
            // Database not available - test is skipped
            return;
        }

        // This test verifies that we can query the database and the schema is correct
        let pool = config::db::establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        // Verify we can query the users table (even if empty)
        let result = users.load::<models::User>(&mut conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_models_are_defined() {
        // Test that all model structs are properly defined (no database required)
        // This ensures the models compile and have the expected structure

        // Test User model
        let user = models::User {
            id: 1,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: chrono::NaiveDateTime::default(),
        };
        assert_eq!(user.id, 1);

        // Test NewUser model
        let new_user = models::NewUser {
            username: "new_user".to_string(),
            email: "new@example.com".to_string(),
            password_hash: "hash".to_string(),
        };
        assert_eq!(new_user.username, "new_user");

        // Test Product model
        let product = models::Product {
            id: 1,
            name: "Test Product".to_string(),
            description: Some("Description".to_string()),
            price: bigdecimal::BigDecimal::from(99),
            inventory_count: 10,
        };
        assert_eq!(product.id, 1);

        // Test NewProduct model
        let new_product = models::NewProduct {
            name: "New Product".to_string(),
            description: Some("Description".to_string()),
            price: bigdecimal::BigDecimal::from(99),
            inventory_count: 10,
        };
        assert_eq!(new_product.name, "New Product");

        // Test Cart model
        let cart = models::Cart {
            id: 1,
            user_id: 1,
            created_at: chrono::NaiveDateTime::default(),
        };
        assert_eq!(cart.user_id, 1);

        // Test NewCart model
        let new_cart = models::NewCart { user_id: 1 };
        assert_eq!(new_cart.user_id, 1);

        // Test CartItem model
        let cart_item = models::CartItem {
            id: 1,
            cart_id: 1,
            product_id: 1,
            quantity: 1,
        };
        assert_eq!(cart_item.quantity, 1);

        // Test NewCartItem model
        let new_cart_item = models::NewCartItem {
            cart_id: 1,
            product_id: 1,
            quantity: 1,
        };
        assert_eq!(new_cart_item.quantity, 1);
    }

    #[test]
    fn test_schema_tables_defined() {
        // Test that schema tables are properly defined (no database required)
        use crate::schema::*;

        // Verify table names exist by referencing their columns
        // This ensures the schema is properly generated and accessible
        let users_id = users::id;
        let products_id = products::id;
        let carts_id = carts::id;
        let cart_items_id = cart_items::id;

        // Verify all column references are accessible
        assert!(format!("{users_id:?}").contains("id"));
        assert!(format!("{products_id:?}").contains("id"));
        assert!(format!("{carts_id:?}").contains("id"));
        assert!(format!("{cart_items_id:?}").contains("id"));
    }
}
