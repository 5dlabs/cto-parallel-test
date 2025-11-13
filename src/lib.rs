pub mod config;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::prelude::*;

    #[test]
    fn test_connection_pool_creation() {
        // This test verifies that the connection pool can be created successfully
        let pool = config::db::establish_connection_pool();
        assert!(pool.get().is_ok());
    }

    #[test]
    fn test_database_schema() {
        // This test verifies that we can query the database and the schema is correct
        use crate::schema::users::dsl::*;

        let pool = config::db::establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        // Verify we can query the users table (even if empty)
        let result = users.load::<models::User>(&mut conn);
        assert!(result.is_ok());
    }
}
