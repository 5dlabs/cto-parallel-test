pub mod config;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::*;
    use config::db::establish_connection_pool;
    use diesel::prelude::*;
    use schema::{cart_items, carts, products, users};

    #[test]
    fn test_database_connection() {
        dotenv::dotenv().ok();
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection from pool");

        // Test that we can query the database
        let result = diesel::sql_query("SELECT 1")
            .execute(&mut conn)
            .expect("Failed to execute query");

        assert!(result > 0);
    }

    #[test]
    fn test_schema_exists() {
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
