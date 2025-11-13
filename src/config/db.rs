// Database configuration and connection pooling
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

/// Type alias for the connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled database connection
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a database connection pool
///
/// This function loads environment variables from a .env file and creates
/// a connection pool for `PostgreSQL` using the `DATABASE_URL` environment variable.
///
/// # Panics
///
/// Panics if:
/// - `DATABASE_URL` environment variable is not set
/// - Connection pool creation fails
///
/// # Examples
///
/// ```no_run
/// use ecommerce_api::config::db::establish_connection_pool;
///
/// let pool = establish_connection_pool();
/// ```
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .build(manager)
        .expect("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires a running PostgreSQL database"]
    fn test_connection_pool_creation() {
        let pool = establish_connection_pool();
        assert!(pool.get().is_ok());
    }
}
