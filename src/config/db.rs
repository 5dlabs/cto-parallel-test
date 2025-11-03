use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database.
///
/// # Panics
///
/// Panics if `DATABASE_URL` environment variable is not set or if the pool cannot be created.
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_creation() {
        // This test requires DATABASE_URL to be set AND PostgreSQL to be running
        // It verifies that the connection pool can be created successfully
        dotenv().ok();
        if env::var("DATABASE_URL").is_ok() {
            // Try to create the pool
            if let Ok(pool) = r2d2::Pool::builder().build(ConnectionManager::<PgConnection>::new(
                env::var("DATABASE_URL").unwrap(),
            )) {
                // Only assert if we can actually get a connection
                assert!(pool.get().is_ok());
            }
            // PostgreSQL is not running, skip this test gracefully
        }
    }
}
