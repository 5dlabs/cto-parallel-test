/// Integration tests for database schema and connectivity
///
/// Note: These tests require a running `PostgreSQL` instance with the `DATABASE_URL`
/// environment variable properly configured. They are marked as ignored by default
/// to prevent CI failures when database is not available.
#[cfg(test)]
mod database_tests {
    use cto_parallel_test::config::db::establish_connection_pool;

    #[test]
    #[ignore = "requires PostgreSQL instance"]
    fn test_connection_pool_creation() {
        // This test verifies that the connection pool can be created
        // when DATABASE_URL is properly configured
        let pool = establish_connection_pool();

        // If we get here without panic, pool creation succeeded
        assert!(pool.max_size() > 0);
    }

    #[test]
    #[ignore = "requires PostgreSQL instance"]
    fn test_database_connectivity() {
        // This test verifies that we can actually connect to the database
        use diesel::prelude::*;

        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection from pool");

        // Execute a simple query to verify connectivity
        let result = diesel::sql_query("SELECT 1 as test").execute(&mut conn);

        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "requires PostgreSQL instance with migrations applied"]
    fn test_schema_tables_exist() {
        // This test verifies that all expected tables exist in the database
        use diesel::prelude::*;
        use diesel::sql_types::Text;

        #[derive(QueryableByName)]
        struct TableName {
            #[diesel(sql_type = Text)]
            tablename: String,
        }

        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection from pool");

        let tables: Vec<TableName> =
            diesel::sql_query("SELECT tablename FROM pg_tables WHERE schemaname = 'public'")
                .load(&mut conn)
                .expect("Failed to query tables");

        let table_names: Vec<String> = tables.into_iter().map(|t| t.tablename).collect();

        // Verify all expected tables exist
        assert!(
            table_names.contains(&"users".to_string()),
            "users table missing"
        );
        assert!(
            table_names.contains(&"products".to_string()),
            "products table missing"
        );
        assert!(
            table_names.contains(&"carts".to_string()),
            "carts table missing"
        );
        assert!(
            table_names.contains(&"cart_items".to_string()),
            "cart_items table missing"
        );
    }
}
