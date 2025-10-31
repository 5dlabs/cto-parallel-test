//! Database configuration and connection pooling
//!
//! This module provides database connection management using Diesel ORM
//! with `PostgreSQL` and `r2d2` connection pooling.

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

/// Type alias for the connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled database connection
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set or if
/// the connection pool cannot be created.
///
/// # Examples
///
/// ```no_run
/// use ecommerce_catalog::config::db::establish_connection_pool;
///
/// let pool = establish_connection_pool();
/// ```
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment or .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url_environment_variable() {
        // Verify DATABASE_URL is set (from .env or environment)
        dotenv().ok();
        let database_url = env::var("DATABASE_URL");
        assert!(
            database_url.is_ok(),
            "DATABASE_URL should be set in environment or .env file"
        );
        assert!(
            !database_url.unwrap().is_empty(),
            "DATABASE_URL should not be empty"
        );
    }

    #[test]
    fn test_connection_manager_creation() {
        // Test that we can create a connection manager with a valid database URL
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let _manager = ConnectionManager::<PgConnection>::new(database_url);

        // If we got here without panic, the manager was created successfully
        // No assertion needed - test passes if no panic occurs
    }
}
