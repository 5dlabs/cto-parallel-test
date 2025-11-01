//! Database configuration and connection pooling
//!
//! This module provides connection pooling for `PostgreSQL` using `r2d2` and `Diesel`.

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

/// Type alias for the connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled database connection
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database.
///
/// # Panics
///
/// This function will panic if:
/// - The `DATABASE_URL` environment variable is not set
/// - The connection pool cannot be created
///
/// # Examples
///
/// ```no_run
/// use cto_parallel_test::config::db::establish_connection_pool;
///
/// let pool = establish_connection_pool();
/// ```
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "This test requires a running PostgreSQL instance"]
    fn test_connection_pool() {
        dotenv().ok();
        let pool = establish_connection_pool();
        assert!(pool.get().is_ok());
    }
}
